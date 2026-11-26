//! Runtime-selected x86 kernels. The crate's default target remains portable.
use super::{DECODED_WORDS, DecodedWord, RESIDUES};
use std::arch::x86_64::*;

pub(super) const SEGMENT_BYTES: usize = 512 * 1024;
pub(super) const DENSE_LIMIT: usize = 256 * 1024;
const CACHE_BLOCK: usize = 32 * 1024;
const CACHE_DENSE_LIMIT: usize = 32 * 1024;
const FILL_GROUP: usize = 17;
pub(super) const AVX_FILL: bool = false;
pub(super) const COMPACT_FILL: bool = true;
const RECONSTRUCT_STREAMS: bool = true;
const _: () = assert!(
    CACHE_BLOCK <= super::SEGMENT_SIZE
        && DENSE_LIMIT <= super::STATIC_DENSE_LIMIT
        && SEGMENT_BYTES.is_power_of_two()
);

#[target_feature(enable = "avx2")]
pub(super) unsafe fn fill_compact(low: usize, sieve: &mut [u8]) {
    let patterns = super::PATTERNS;
    // SAFETY: AVX2 enabled; each helper splits at every input wrap.
    unsafe {
        cyclic::<4, true>(low, sieve, patterns[..4].try_into().unwrap());
        cyclic::<4, false>(low, sieve, patterns[4..8].try_into().unwrap());
        cyclic::<4, false>(low, sieve, patterns[8..12].try_into().unwrap());
        cyclic::<4, false>(low, sieve, patterns[12..16].try_into().unwrap());
        cyclic::<1, false>(low, sieve, patterns[16..].try_into().unwrap());
    }
}

#[target_feature(enable = "avx2")]
unsafe fn cyclic<const N: usize, const INIT: bool>(
    low: usize,
    mut output: &mut [u8],
    patterns: [&[u8]; N],
) {
    let mut offsets: [usize; N] = std::array::from_fn(|i| low % patterns[i].len());
    while !output.is_empty() {
        let len = (0..N).fold(output.len(), |len, i| {
            len.min(patterns[i].len() - offsets[i])
        });
        let inputs = std::array::from_fn(|i| &patterns[i][offsets[i]..offsets[i] + len]);
        let (chunk, tail) = output.split_at_mut(len);
        // SAFETY: every input is exactly chunk.len() bytes; AVX2 enabled.
        unsafe { combine::<N, INIT>(chunk, inputs) };
        for i in 0..N {
            offsets[i] += len;
            if offsets[i] == patterns[i].len() {
                offsets[i] = 0;
            }
        }
        output = tail;
    }
}

#[target_feature(enable = "avx2")]
pub(super) unsafe fn fill_expanded(low: usize, sieve: &mut [u8], patterns: &[Vec<u8>; 17]) {
    let sources: [&[u8]; 17] = std::array::from_fn(|i| {
        let offset = low % super::PATTERNS[i].len();
        &patterns[i][offset..offset + sieve.len()]
    });
    // SAFETY: all slices contain sieve.len() bytes, and AVX2 is enabled.
    unsafe {
        match FILL_GROUP {
            6 => {
                combine::<6, true>(sieve, sources[..6].try_into().unwrap());
                combine::<6, false>(sieve, sources[6..12].try_into().unwrap());
                combine::<5, false>(sieve, sources[12..].try_into().unwrap());
            }
            9 => {
                combine::<9, true>(sieve, sources[..9].try_into().unwrap());
                combine::<8, false>(sieve, sources[9..].try_into().unwrap());
            }
            _ => combine::<17, true>(sieve, sources),
        }
    }
}

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn combine<const N: usize, const INIT: bool>(output: &mut [u8], sources: [&[u8]; N]) {
    let vector_len = output.len() / 32 * 32;
    for offset in (0..vector_len).step_by(32) {
        // SAFETY: offset + 32 <= output.len(); inputs are at least that long.
        unsafe {
            let mut bits = if INIT {
                _mm256_set1_epi8(-1)
            } else {
                _mm256_loadu_si256(output.as_ptr().add(offset).cast())
            };
            for source in sources {
                bits =
                    _mm256_and_si256(bits, _mm256_loadu_si256(source.as_ptr().add(offset).cast()));
            }
            _mm256_storeu_si256(output.as_mut_ptr().add(offset).cast(), bits);
        }
    }
    for offset in vector_len..output.len() {
        let mut bits = if INIT { 255 } else { output[offset] };
        for source in sources {
            bits &= source[offset];
        }
        output[offset] = bits;
    }
}

#[target_feature(enable = "avx2")]
pub(super) unsafe fn rotate_offsets(offsets: &mut [u32; 8], first: usize) {
    let indices = _mm256_add_epi32(
        _mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7),
        _mm256_set1_epi32(first as i32),
    );
    // SAFETY: offsets supplies exactly one unaligned 256-bit load/store.
    // VPERMD uses each index's low three bits, giving a cyclic lane rotation.
    unsafe {
        let values = _mm256_loadu_si256(offsets.as_ptr().cast());
        _mm256_storeu_si256(
            offsets.as_mut_ptr().cast(),
            _mm256_permutevar8x32_epi32(values, indices),
        );
    }
}

#[target_feature(enable = "popcnt")]
pub(super) unsafe fn count_popcnt(bytes: &[u8]) -> usize {
    let mut count = 0;
    let mut chunks = bytes.chunks_exact(32);
    for block in &mut chunks {
        count += u64::from_ne_bytes(block[0..8].try_into().unwrap()).count_ones() as usize
            + u64::from_ne_bytes(block[8..16].try_into().unwrap()).count_ones() as usize
            + u64::from_ne_bytes(block[16..24].try_into().unwrap()).count_ones() as usize
            + u64::from_ne_bytes(block[24..32].try_into().unwrap()).count_ones() as usize;
    }
    count
        + chunks
            .remainder()
            .iter()
            .map(|b| b.count_ones() as usize)
            .sum::<usize>()
}

#[target_feature(enable = "avx2")]
pub(super) unsafe fn count_avx2(bytes: &[u8]) -> usize {
    let lookup = _mm256_broadcastsi128_si256(_mm_setr_epi8(
        0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4,
    ));
    let mask = _mm256_set1_epi8(15);
    let zero = _mm256_setzero_si256();
    let mut sums = zero;
    let mut chunks = bytes.chunks_exact(32);
    for chunk in &mut chunks {
        // SAFETY: each chunk contains exactly 32 bytes; unaligned loads are valid.
        let word = unsafe { _mm256_loadu_si256(chunk.as_ptr().cast()) };
        let lo = _mm256_shuffle_epi8(lookup, _mm256_and_si256(word, mask));
        let hi = _mm256_shuffle_epi8(lookup, _mm256_and_si256(_mm256_srli_epi16::<4>(word), mask));
        sums = _mm256_add_epi64(sums, _mm256_sad_epu8(_mm256_add_epi8(lo, hi), zero));
    }
    let mut lanes = [0u64; 4];
    // SAFETY: the four lanes provide exactly 32 writable bytes.
    unsafe { _mm256_storeu_si256(lanes.as_mut_ptr().cast(), sums) };
    lanes.iter().sum::<u64>() as usize
        + chunks
            .remainder()
            .iter()
            .map(|b| b.count_ones() as usize)
            .sum::<usize>()
}

pub(super) trait Output: Copy {
    fn number(n: usize) -> Self;
    /// Caller provides sixteen spare slots and enables AVX2.
    unsafe fn store(dst: *mut Self, number: usize, decoded: DecodedWord);
}

impl Output for usize {
    fn number(n: usize) -> Self {
        n
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn store(dst: *mut Self, number: usize, decoded: DecodedWord) {
        let base = _mm256_set1_epi64x(number as i64);
        let offsets = _mm_cvtsi64_si128(decoded.offsets as i64);
        // SAFETY: sixteen spare slots cover both vector stores and any dense
        // scalar tail. Wrapping padding values are never exposed by set_len.
        unsafe {
            _mm256_storeu_si256(
                dst.cast(),
                _mm256_add_epi64(base, _mm256_cvtepu8_epi64(offsets)),
            );
            if decoded.len > 4 {
                _mm256_storeu_si256(
                    dst.add(4).cast(),
                    _mm256_add_epi64(base, _mm256_cvtepu8_epi64(_mm_srli_si128::<4>(offsets))),
                );
            }
            store_remaining(dst, number, decoded);
        }
    }
}

impl Output for u32 {
    fn number(n: usize) -> Self {
        n as u32
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn store(dst: *mut Self, number: usize, decoded: DecodedWord) {
        let base = _mm256_set1_epi32(number as i32);
        let offsets = _mm_cvtsi64_si128(decoded.offsets as i64);
        // SAFETY: eight padded lanes and up to eight remaining valid seeds
        // fit the sixteen-slot reservation. Only valid values are exposed.
        unsafe {
            _mm256_storeu_si256(
                dst.cast(),
                _mm256_add_epi32(base, _mm256_cvtepu8_epi32(offsets)),
            );
            store_remaining(dst, number, decoded);
        }
    }
}

#[inline]
unsafe fn store_remaining<T: Output>(dst: *mut T, number: usize, decoded: DecodedWord) {
    let mut remaining = decoded.remaining;
    let mut written = 8;
    while remaining != 0 {
        let bit = remaining.trailing_zeros() as usize;
        // SAFETY: remaining contains at most eight additional entries; callers
        // supply sixteen writable slots. Each valid output fits its target type.
        unsafe { *dst.add(written) = T::number(number + 30 * (bit / 8) + RESIDUES[bit % 8]) };
        written += 1;
        remaining &= remaining - 1;
    }
}

#[target_feature(enable = "avx2")]
pub(super) unsafe fn append_avx2<T: Output>(primes: &mut Vec<T>, low: usize, sieve: &[u8]) {
    let mut blocks = sieve.chunks_exact(8);
    for (i, block) in blocks.by_ref().enumerate() {
        let bits = u64::from_le_bytes(block.try_into().unwrap());
        if bits == 0 {
            continue;
        }
        let a = DECODED_WORDS[(bits & 0xffff) as usize];
        let b = DECODED_WORDS[((bits >> 16) & 0xffff) as usize];
        let c = DECODED_WORDS[((bits >> 32) & 0xffff) as usize];
        let d = DECODED_WORDS[(bits >> 48) as usize];
        let len = primes.len();
        if primes.capacity() - len < 64 {
            primes.reserve(64);
        }
        let number = 30 * (low + 8 * i);
        let ab = a.len as usize + b.len as usize;
        let abc = ab + c.len as usize;
        // SAFETY: four records contribute at most sixteen values each; all
        // padded stores fit the reserved 64 slots. Expose only valid entries.
        unsafe {
            let dst = primes.as_mut_ptr().add(len);
            T::store(dst, number, a);
            T::store(dst.add(a.len as usize), number + 60, b);
            T::store(dst.add(ab), number + 120, c);
            T::store(dst.add(abc), number + 180, d);
            primes.set_len(len + abc + d.len as usize);
        }
    }
    let remainder = blocks.remainder();
    let tail_low = low + sieve.len() - remainder.len();
    let mut pairs = remainder.chunks_exact(2);
    for (i, pair) in pairs.by_ref().enumerate() {
        let word = u16::from_le_bytes([pair[0], pair[1]]);
        if word == 0 {
            continue;
        }
        let decoded = DECODED_WORDS[word as usize];
        let len = primes.len();
        if primes.capacity() - len < 16 {
            primes.reserve(16);
        }
        // SAFETY: a single record receives sixteen spare writable slots.
        unsafe {
            T::store(
                primes.as_mut_ptr().add(len),
                30 * (tail_low + 2 * i),
                decoded,
            );
            primes.set_len(len + decoded.len as usize);
        }
    }
    if let Some(&byte) = pairs.remainder().first() {
        append_byte(primes, 30 * (low + sieve.len() - 1), byte);
    }
}

const WORD_OFFSETS: [u8; 65] = {
    let mut offsets = [0; 65];
    let mut i = 0;
    while i < 64 {
        offsets[i] = (30 * (i / 8) + RESIDUES[i % 8]) as u8;
        i += 1;
    }
    offsets
};

#[target_feature(enable = "popcnt,bmi1")]
pub(super) unsafe fn append_popcnt<T: Output>(primes: &mut Vec<T>, low: usize, sieve: &[u8]) {
    let mut chunks = sieve.chunks_exact(8);
    for (i, chunk) in chunks.by_ref().enumerate() {
        let mut bits = u64::from_le_bytes(chunk.try_into().unwrap());
        let count = bits.count_ones() as usize;
        if count == 0 {
            continue;
        }
        let len = primes.len();
        if primes.capacity() - len < 64 {
            primes.reserve(64);
        }
        let base = 30 * (low + 8 * i);
        let mut written = 0;
        // SAFETY: round the count up to a group of four, never past 64 stores.
        // trailing_zeros(0) selects the initialized padding slot, not an output.
        unsafe {
            let dst = primes.as_mut_ptr().add(len);
            while written < count {
                for j in 0..4 {
                    *dst.add(written + j) =
                        T::number(base + WORD_OFFSETS[bits.trailing_zeros() as usize] as usize);
                    bits &= bits.wrapping_sub(1);
                }
                written += 4;
            }
            primes.set_len(len + count);
        }
    }
    let low = low + 8 * (sieve.len() / 8);
    for (i, &byte) in chunks.remainder().iter().enumerate() {
        append_byte(primes, 30 * (low + i), byte);
    }
}

fn append_byte<T: Output>(primes: &mut Vec<T>, number: usize, mut bits: u8) {
    while bits != 0 {
        primes.push(T::number(number + RESIDUES[bits.trailing_zeros() as usize]));
        bits &= bits - 1;
    }
}

// In-place canonical-wheel traversal follows primesieve's EratSmall design.
// Copyright (c) 2010-2026 Kim Walisch; BSD-2-Clause, see THIRD_PARTY_NOTICES.md.
// This implementation uses primeroute's own residue order and grouped state.
const DENSE_OFFSET_MASK: u32 = (1 << 29) - 1;
struct DensePrime {
    quotient: u32,
    next_wheel: u32,
}
pub(super) struct DenseGroups {
    groups: [Vec<DensePrime>; 8],
    larger: [Vec<DensePrime>; 8],
}
impl DenseGroups {
    pub(super) fn new() -> Self {
        Self {
            groups: std::array::from_fn(|_| Vec::new()),
            larger: std::array::from_fn(|_| Vec::new()),
        }
    }
    pub(super) fn activate(&mut self, state: super::SievingPrime) {
        let class = super::RESIDUE_INDEX[state.prime as usize % 30];
        let next = state.next[state.first as usize];
        debug_assert!(next <= DENSE_OFFSET_MASK);
        let groups = if state.prime as usize <= CACHE_DENSE_LIMIT {
            &mut self.groups
        } else {
            &mut self.larger
        };
        groups[class].push(DensePrime {
            quotient: state.prime / 30,
            next_wheel: next | ((state.first as u32) << 29),
        });
    }
    pub(super) fn mark(&mut self, sieve: &mut [u8]) {
        for bytes in sieve.chunks_mut(CACHE_BLOCK) {
            mark_dense_group::<0>(&mut self.groups[0], bytes);
            mark_dense_group::<1>(&mut self.groups[1], bytes);
            mark_dense_group::<2>(&mut self.groups[2], bytes);
            mark_dense_group::<3>(&mut self.groups[3], bytes);
            mark_dense_group::<4>(&mut self.groups[4], bytes);
            mark_dense_group::<5>(&mut self.groups[5], bytes);
            mark_dense_group::<6>(&mut self.groups[6], bytes);
            mark_dense_group::<7>(&mut self.groups[7], bytes);
        }
        // Few-hit primes advance once per outer window. Compact in-place
        // states avoid queue moves while the small-prime work stays in L1.
        mark_dense_group::<0>(&mut self.larger[0], sieve);
        mark_dense_group::<1>(&mut self.larger[1], sieve);
        mark_dense_group::<2>(&mut self.larger[2], sieve);
        mark_dense_group::<3>(&mut self.larger[3], sieve);
        mark_dense_group::<4>(&mut self.larger[4], sieve);
        mark_dense_group::<5>(&mut self.larger[5], sieve);
        mark_dense_group::<6>(&mut self.larger[6], sieve);
        mark_dense_group::<7>(&mut self.larger[7], sieve);
    }
}
#[inline(never)]
fn mark_dense_group<const CLASS: usize>(states: &mut [DensePrime], sieve: &mut [u8]) {
    if RECONSTRUCT_STREAMS && std::arch::is_x86_feature_detected!("avx2") {
        // SAFETY: AVX2 detected once for this group.
        unsafe { mark_stream_group::<CLASS>(states, sieve) };
    } else {
        mark_canonical_group::<CLASS>(states, sieve);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn mark_stream_group<const CLASS: usize>(states: &mut [DensePrime], sieve: &mut [u8]) {
    let steps = const { canonical_steps(CLASS) };
    let factors: [u32; 8] = std::array::from_fn(|i| steps[i].0 as u32);
    let carries: [u32; 8] = std::array::from_fn(|i| steps[i].1 as u32);
    // SAFETY: both coefficient arrays have exactly eight 32-bit elements.
    let (factors, carries) = unsafe {
        (
            _mm256_loadu_si256(factors.as_ptr().cast()),
            _mm256_loadu_si256(carries.as_ptr().cast()),
        )
    };
    let lanes = _mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7);
    for state in states {
        let packed = state.next_wheel;
        let first = (packed >> 29) as usize;
        let offset = packed & DENSE_OFFSET_MASK;
        if offset as usize >= sieve.len() {
            state.next_wheel = packed - sieve.len() as u32;
            continue;
        }
        let q = state.quotient;
        let prime = q * 30 + RESIDUES[CLASS] as u32;
        let origin = offset.wrapping_sub(q * steps[first].0 as u32 + steps[first].1 as u32);
        // Lanes before the minimum belong to the following period. All final
        // offsets are nonnegative and below offset + prime; intermediate
        // wrapping subtraction is intentional and canceled by vector addition.
        let wrap = _mm256_and_si256(
            _mm256_cmpgt_epi32(_mm256_set1_epi32(first as i32), lanes),
            _mm256_set1_epi32(prime as i32),
        );
        let offsets = _mm256_add_epi32(
            _mm256_add_epi32(
                _mm256_mullo_epi32(factors, _mm256_set1_epi32(q as i32)),
                carries,
            ),
            _mm256_add_epi32(_mm256_set1_epi32(origin as i32), wrap),
        );
        let mut wide = super::SievingPrime {
            prime,
            next: [0; 8],
            masks: super::WHEEL_MASKS[CLASS],
            first: first as u8,
        };
        // SAFETY: wide.next supplies exactly 32 writable bytes.
        unsafe { _mm256_storeu_si256(wide.next.as_mut_ptr().cast(), offsets) };
        mark_dense_class::<CLASS>(&mut wide, sieve);
        state.next_wheel = wide.next[wide.first as usize] | ((wide.first as u32) << 29);
    }
}

#[inline(never)]
fn mark_canonical_group<const CLASS: usize>(states: &mut [DensePrime], sieve: &mut [u8]) {
    let len = sieve.len();
    let steps = const { canonical_steps(CLASS) };
    'primes: for state in states {
        let packed = state.next_wheel;
        let mut i = (packed & DENSE_OFFSET_MASK) as usize;
        if i >= len {
            // Subtraction cannot borrow into the three wheel bits.
            state.next_wheel = packed - len as u32;
            continue;
        }
        let wheel = (packed >> 29) as usize;
        let q = state.quotient as usize;
        macro_rules! hit {
            ($index:expr) => {{
                if i >= len {
                    state.next_wheel = (i - len) as u32 | (($index as u32) << 29);
                    continue 'primes;
                }
                let step = super::WHEEL_STEPS[CLASS * 8 + $index];
                // SAFETY: the preceding check establishes i < len.
                unsafe {
                    *sieve.get_unchecked_mut(i) &= step.mask;
                }
                i += q * step.gap as usize + step.carry as usize;
            }};
        }
        // Finish the starting partial wheel. Bulk cycles always start at zero,
        // giving eight class-specialized loops instead of 64 wheel loops.
        // Labeled blocks provide fall-through entry without duplicating tails.
        'prefix_done: {
            'p7: {
                'p6: {
                    'p5: {
                        'p4: {
                            'p3: {
                                'p2: {
                                    'p1: {
                                        match wheel {
                                            0 => break 'prefix_done,
                                            1 => break 'p1,
                                            2 => break 'p2,
                                            3 => break 'p3,
                                            4 => break 'p4,
                                            5 => break 'p5,
                                            6 => break 'p6,
                                            _ => break 'p7,
                                        }
                                    }
                                    hit!(1);
                                }
                                hit!(2);
                            }
                            hit!(3);
                        }
                        hit!(4);
                    }
                    hit!(5);
                }
                hit!(6);
            }
            hit!(7);
        }
        let h = q * steps[7].0 + steps[7].1;
        if h < len {
            let a = q * steps[1].0 + steps[1].1;
            let b = q * steps[2].0 + steps[2].1;
            let c = q * steps[3].0 + steps[3].1;
            let d = q * steps[4].0 + steps[4].1;
            let e = q * steps[5].0 + steps[5].1;
            let f = q * steps[6].0 + steps[6].1;
            let p = q * 30 + RESIDUES[CLASS];
            let end = len - h;
            while i < end {
                // SAFETY: offsets are nondecreasing and h is their maximum.
                // i < len - h bounds all eight accesses; a full wheel adds p.
                unsafe {
                    *sieve.get_unchecked_mut(i) &= steps[0].2;
                    *sieve.get_unchecked_mut(i + a) &= steps[1].2;
                    *sieve.get_unchecked_mut(i + b) &= steps[2].2;
                    *sieve.get_unchecked_mut(i + c) &= steps[3].2;
                    *sieve.get_unchecked_mut(i + d) &= steps[4].2;
                    *sieve.get_unchecked_mut(i + e) &= steps[5].2;
                    *sieve.get_unchecked_mut(i + f) &= steps[6].2;
                    *sieve.get_unchecked_mut(i + h) &= steps[7].2;
                }
                i += p;
            }
        }
        hit!(0);
        hit!(1);
        hit!(2);
        hit!(3);
        hit!(4);
        hit!(5);
        hit!(6);
        assert!(i >= len, "full wheel was already handled");
        state.next_wheel = (i - len) as u32 | (7 << 29);
    }
}
const fn canonical_steps(class: usize) -> [(usize, usize, u8); 8] {
    let mut values = [(0, 0, 0); 8];
    let mut gap = 0;
    let mut carry = 0;
    let mut index = 0;
    while index < 8 {
        let step = super::WHEEL_STEPS[class * 8 + index];
        values[index] = (gap, carry, step.mask);
        gap += step.gap as usize;
        carry += step.carry as usize;
        index += 1;
    }
    values
}

#[derive(Clone, Copy)]
struct MediumPrime {
    next: u32,
    quotient: u32,
}

pub(super) struct MediumGroups {
    current: [Vec<MediumPrime>; 64],
    next: [Vec<MediumPrime>; 64],
}

impl MediumGroups {
    pub(super) fn new() -> Self {
        Self {
            current: std::array::from_fn(|_| Vec::new()),
            next: std::array::from_fn(|_| Vec::new()),
        }
    }
    pub(super) fn activate(&mut self, state: &super::SparsePrime) {
        // Only active medium primes enter: offsets are below one window plus
        // their next step (at most 2.6 windows), far below u32::MAX.
        self.current[state.wheel as usize].push(MediumPrime {
            next: u32::try_from(state.next).unwrap(),
            quotient: state.quotient,
        });
    }
    pub(super) fn mark(&mut self, sieve: &mut [u8]) {
        for (wheel, states) in self.current.iter_mut().enumerate() {
            match wheel {
                0 => mark_medium_group::<0>(states, &mut self.next, sieve),
                1 => mark_medium_group::<1>(states, &mut self.next, sieve),
                2 => mark_medium_group::<2>(states, &mut self.next, sieve),
                3 => mark_medium_group::<3>(states, &mut self.next, sieve),
                4 => mark_medium_group::<4>(states, &mut self.next, sieve),
                5 => mark_medium_group::<5>(states, &mut self.next, sieve),
                6 => mark_medium_group::<6>(states, &mut self.next, sieve),
                7 => mark_medium_group::<7>(states, &mut self.next, sieve),
                8 => mark_medium_group::<8>(states, &mut self.next, sieve),
                9 => mark_medium_group::<9>(states, &mut self.next, sieve),
                10 => mark_medium_group::<10>(states, &mut self.next, sieve),
                11 => mark_medium_group::<11>(states, &mut self.next, sieve),
                12 => mark_medium_group::<12>(states, &mut self.next, sieve),
                13 => mark_medium_group::<13>(states, &mut self.next, sieve),
                14 => mark_medium_group::<14>(states, &mut self.next, sieve),
                15 => mark_medium_group::<15>(states, &mut self.next, sieve),
                16 => mark_medium_group::<16>(states, &mut self.next, sieve),
                17 => mark_medium_group::<17>(states, &mut self.next, sieve),
                18 => mark_medium_group::<18>(states, &mut self.next, sieve),
                19 => mark_medium_group::<19>(states, &mut self.next, sieve),
                20 => mark_medium_group::<20>(states, &mut self.next, sieve),
                21 => mark_medium_group::<21>(states, &mut self.next, sieve),
                22 => mark_medium_group::<22>(states, &mut self.next, sieve),
                23 => mark_medium_group::<23>(states, &mut self.next, sieve),
                24 => mark_medium_group::<24>(states, &mut self.next, sieve),
                25 => mark_medium_group::<25>(states, &mut self.next, sieve),
                26 => mark_medium_group::<26>(states, &mut self.next, sieve),
                27 => mark_medium_group::<27>(states, &mut self.next, sieve),
                28 => mark_medium_group::<28>(states, &mut self.next, sieve),
                29 => mark_medium_group::<29>(states, &mut self.next, sieve),
                30 => mark_medium_group::<30>(states, &mut self.next, sieve),
                31 => mark_medium_group::<31>(states, &mut self.next, sieve),
                32 => mark_medium_group::<32>(states, &mut self.next, sieve),
                33 => mark_medium_group::<33>(states, &mut self.next, sieve),
                34 => mark_medium_group::<34>(states, &mut self.next, sieve),
                35 => mark_medium_group::<35>(states, &mut self.next, sieve),
                36 => mark_medium_group::<36>(states, &mut self.next, sieve),
                37 => mark_medium_group::<37>(states, &mut self.next, sieve),
                38 => mark_medium_group::<38>(states, &mut self.next, sieve),
                39 => mark_medium_group::<39>(states, &mut self.next, sieve),
                40 => mark_medium_group::<40>(states, &mut self.next, sieve),
                41 => mark_medium_group::<41>(states, &mut self.next, sieve),
                42 => mark_medium_group::<42>(states, &mut self.next, sieve),
                43 => mark_medium_group::<43>(states, &mut self.next, sieve),
                44 => mark_medium_group::<44>(states, &mut self.next, sieve),
                45 => mark_medium_group::<45>(states, &mut self.next, sieve),
                46 => mark_medium_group::<46>(states, &mut self.next, sieve),
                47 => mark_medium_group::<47>(states, &mut self.next, sieve),
                48 => mark_medium_group::<48>(states, &mut self.next, sieve),
                49 => mark_medium_group::<49>(states, &mut self.next, sieve),
                50 => mark_medium_group::<50>(states, &mut self.next, sieve),
                51 => mark_medium_group::<51>(states, &mut self.next, sieve),
                52 => mark_medium_group::<52>(states, &mut self.next, sieve),
                53 => mark_medium_group::<53>(states, &mut self.next, sieve),
                54 => mark_medium_group::<54>(states, &mut self.next, sieve),
                55 => mark_medium_group::<55>(states, &mut self.next, sieve),
                56 => mark_medium_group::<56>(states, &mut self.next, sieve),
                57 => mark_medium_group::<57>(states, &mut self.next, sieve),
                58 => mark_medium_group::<58>(states, &mut self.next, sieve),
                59 => mark_medium_group::<59>(states, &mut self.next, sieve),
                60 => mark_medium_group::<60>(states, &mut self.next, sieve),
                61 => mark_medium_group::<61>(states, &mut self.next, sieve),
                62 => mark_medium_group::<62>(states, &mut self.next, sieve),
                _ => mark_medium_group::<63>(states, &mut self.next, sieve),
            }
            states.clear();
        }
        std::mem::swap(&mut self.current, &mut self.next);
    }
}

#[inline(never)]
fn mark_medium_group<const WHEEL: usize>(
    states: &[MediumPrime],
    next: &mut [Vec<MediumPrime>; 64],
    sieve: &mut [u8],
) {
    for state in states {
        let (offset, wheel) = mark_medium_chain::<WHEEL>(*state, sieve);
        next[wheel].push(MediumPrime {
            next: offset,
            quotient: state.quotient,
        });
    }
}

#[inline(always)]
fn mark_medium_chain<const WHEEL: usize>(state: MediumPrime, sieve: &mut [u8]) -> (u32, usize) {
    let mut i = state.next as usize;
    let q = state.quotient as usize;
    let len = sieve.len();
    macro_rules! hit {
        ($offset:expr) => {{
            let wheel = (WHEEL / 8) * 8 + ((WHEEL + $offset) & 7);
            if i >= len {
                return ((i - len) as u32, wheel);
            }
            let step = super::WHEEL_STEPS[wheel];
            sieve[i] &= step.mask;
            i += q * step.gap as usize + step.carry as usize;
        }};
    }
    // One complete wheel advances by p. Repeat when smaller medium primes
    // have more than eight hits; each bounds check returns the exact next state.
    loop {
        hit!(0);
        hit!(1);
        hit!(2);
        hit!(3);
        hit!(4);
        hit!(5);
        hit!(6);
        hit!(7);
    }
}

#[inline(always)]
fn mark_dense_class<const CLASS: usize>(state: &mut super::SievingPrime, sieve: &mut [u8]) {
    let len = sieve.len();
    let p = state.prime as usize;
    // The predecessor of the minimum in cyclic order is the maximum.
    let mut last = state.next[(state.first as usize + 7) & 7] as usize;
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state.next.map(|n| n as usize);
    let [ma, mb, mc, md, me, mf, mg, mh] = super::WHEEL_MASKS[CLASS];
    // Interleave independent stores with one bounds check for all eight.
    while last < len {
        // SAFETY: every index <= last < len, and all eight indices
        // advance by the same positive p on each iteration.
        unsafe {
            *sieve.get_unchecked_mut(a) &= ma;
            *sieve.get_unchecked_mut(b) &= mb;
            *sieve.get_unchecked_mut(c) &= mc;
            *sieve.get_unchecked_mut(d) &= md;
            *sieve.get_unchecked_mut(e) &= me;
            *sieve.get_unchecked_mut(f) &= mf;
            *sieve.get_unchecked_mut(g) &= mg;
            *sieve.get_unchecked_mut(h) &= mh;
        }
        a += p;
        b += p;
        c += p;
        d += p;
        e += p;
        f += p;
        g += p;
        h += p;
        last += p;
    }
    state.next = [a, b, c, d, e, f, g, h].map(|n| n as u32);
    // Remaining hits form a prefix of that order. Advancing a stream by p
    // moves it past the other seven, so first rotates to the next minimum.
    let mut first = state.first as usize;
    while (state.next[first] as usize) < len {
        let i = state.next[first] as usize;
        sieve[i] &= super::WHEEL_MASKS[CLASS][first];
        state.next[first] += p as u32;
        first = (first + 1) & 7;
    }
    for next in &mut state.next {
        *next -= len as u32;
    }
    state.first = first as u8;
}

#[cfg(test)]
#[inline(never)]
fn mark_reference(states: &mut [super::SievingPrime], bitmap: &mut [u8]) {
    for state in states {
        super::mark_dense(state, bitmap);
    }
}

#[cfg(test)]
#[inline(never)]
fn mark_classes(states: &mut [super::SievingPrime], bitmap: &mut [u8]) {
    for state in states {
        match super::RESIDUE_INDEX[state.prime as usize % 30] {
            0 => mark_dense_class::<0>(state, bitmap),
            1 => mark_dense_class::<1>(state, bitmap),
            2 => mark_dense_class::<2>(state, bitmap),
            3 => mark_dense_class::<3>(state, bitmap),
            4 => mark_dense_class::<4>(state, bitmap),
            5 => mark_dense_class::<5>(state, bitmap),
            6 => mark_dense_class::<6>(state, bitmap),
            _ => mark_dense_class::<7>(state, bitmap),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_dense_state_has_disjoint_fields() {
        assert_eq!(std::mem::size_of::<DensePrime>(), 8);
        for wheel in 0..8u32 {
            for next in [0, 1, 32768, 131072, DENSE_OFFSET_MASK] {
                let packed = next | (wheel << 29);
                assert_eq!(packed & DENSE_OFFSET_MASK, next);
                assert_eq!(packed >> 29, wheel);
                if next >= 131072 {
                    assert_eq!((packed - 131072) & DENSE_OFFSET_MASK, next - 131072);
                    assert_eq!((packed - 131072) >> 29, wheel);
                }
            }
        }
    }

    #[test]
    fn expanded_fill_matches_cyclic_masks_with_unaligned_tails() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }
        let expanded = super::super::PATTERNS.map(|p| super::super::repeat_pattern(p, 131_072));
        for low in [0, 1, 17, 1_000_000_000_000usize / 30] {
            for len in [0, 1, 31, 32, 33, 127, 1023, 131_071, 131_072] {
                let mut expected = vec![0; len];
                super::super::fill_segment(low, &mut expected);
                let mut guarded = vec![0xa5; len + 63];
                // SAFETY: AVX2 checked; expanded patterns include every input byte.
                unsafe { fill_expanded(low, &mut guarded[31..31 + len], &expanded) };
                assert_eq!(&guarded[31..31 + len], expected);
                assert!(
                    guarded[..31]
                        .iter()
                        .chain(&guarded[31 + len..])
                        .all(|&b| b == 0xa5)
                );
                guarded.fill(0xa5);
                // SAFETY: AVX2 checked above; the helper uses bounded chunks.
                unsafe { fill_compact(low, &mut guarded[31..31 + len]) };
                assert_eq!(&guarded[31..31 + len], expected);
                assert!(
                    guarded[..31]
                        .iter()
                        .chain(&guarded[31 + len..])
                        .all(|&b| b == 0xa5)
                );
            }
        }
    }

    #[test]
    fn counts_match_scalar_for_unaligned_inputs_and_tails() {
        let mut data = vec![255u8; 131_072 + 32];
        for pattern in 0..2 {
            if pattern == 1 {
                for (i, byte) in data.iter_mut().enumerate() {
                    *byte = i.wrapping_mul(137).wrapping_add(i / 7) as u8;
                }
            }
            for offset in 0..32 {
                for len in [0, 1, 7, 8, 15, 16, 31, 32, 33, 255, 16_385, 131_072] {
                    let bytes = &data[offset..offset + len];
                    let expected: usize = bytes.iter().map(|b| b.count_ones() as usize).sum();
                    if std::arch::is_x86_feature_detected!("popcnt") {
                        // SAFETY: feature checked; inputs are bounded slices.
                        assert_eq!(unsafe { count_popcnt(bytes) }, expected);
                    }
                    if std::arch::is_x86_feature_detected!("avx2") {
                        // SAFETY: feature checked; inputs are bounded slices.
                        assert_eq!(unsafe { count_avx2(bytes) }, expected);
                    }
                }
            }
        }
    }

    #[test]
    fn every_decoder_matches_scalar_with_growth_and_endpoint_padding() {
        for word in 0..=u16::MAX {
            for copies in [1, 4] {
                let pair = word.to_le_bytes();
                let bitmap: Vec<_> = (0..copies).flat_map(|_| pair).collect();
                check_decoders(1_000, &bitmap);
            }
        }
        for len in 0..=65 {
            let bytes: Vec<_> = (0..len).map(|i| (i * 137) as u8).collect();
            check_decoders(1_000, &bytes);
        }
        check_decoders(u32::MAX as usize / 30 - 2, &[255, 255, 15]);
    }

    fn check_decoders(low: usize, bitmap: &[u8]) {
        let mut expected = vec![42];
        for (i, &byte) in bitmap.iter().enumerate() {
            for (bit, &residue) in RESIDUES.iter().enumerate() {
                if byte & (1 << bit) != 0 {
                    expected.push(30 * (low + i) + residue);
                }
            }
        }
        let expected_seeds: Vec<_> = expected.iter().map(|&n| n as u32).collect();
        if std::arch::is_x86_feature_detected!("avx2") {
            let mut actual = vec![42usize];
            let mut seeds = vec![42u32];
            // SAFETY: AVX2 checked, and valid endpoint seeds fit u32.
            unsafe {
                append_avx2(&mut actual, low, bitmap);
                append_avx2(&mut seeds, low, bitmap);
            }
            assert_eq!(actual, expected);
            assert_eq!(seeds, expected_seeds);
        }
        if std::arch::is_x86_feature_detected!("popcnt")
            && std::arch::is_x86_feature_detected!("bmi1")
        {
            let mut actual = vec![42usize];
            let mut seeds = vec![42u32];
            // SAFETY: both features checked, and valid endpoint seeds fit u32.
            unsafe {
                append_popcnt(&mut actual, low, bitmap);
                append_popcnt(&mut seeds, low, bitmap);
            }
            assert_eq!(actual, expected);
            assert_eq!(seeds, expected_seeds);
        }
    }

    #[test]
    #[ignore = "native kernel selection experiment; not a full-query benchmark"]
    fn profile_kernels() {
        use std::hint::black_box;
        use std::time::Instant;
        assert!(std::arch::is_x86_feature_detected!("avx2"));
        assert!(std::arch::is_x86_feature_detected!("popcnt"));
        assert!(std::arch::is_x86_feature_detected!("bmi1"));
        for start in [1_000_000_000usize, 1_000_000_000_000] {
            super::super::sieve_segments(start, start + 999_999, |low, bitmap| {
                let mut expected = Vec::<usize>::new();
                for (i, &byte) in bitmap.iter().enumerate() {
                    append_byte(&mut expected, 30 * (low + i), byte);
                }
                let mut output = Vec::<usize>::with_capacity(expected.len() + 64);
                for round in 0..9 {
                    for variant in [(round % 2), ((round + 1) % 2)] {
                        output.clear();
                        // SAFETY: all three required features asserted above.
                        unsafe {
                            if variant == 0 {
                                append_avx2(&mut output, low, bitmap);
                            } else {
                                append_popcnt(&mut output, low, bitmap);
                            }
                        }
                        assert_eq!(output, expected);
                        let timer = Instant::now();
                        for _ in 0..100 {
                            output.clear();
                            // SAFETY: features asserted above.
                            unsafe {
                                if variant == 0 {
                                    append_avx2(&mut output, low, bitmap);
                                } else {
                                    append_popcnt(&mut output, low, bitmap);
                                }
                            }
                            black_box(&output);
                        }
                        println!(
                            "decoder start={start} round={round} variant={variant} ns={}",
                            timer.elapsed().as_nanos() / 100
                        );
                    }
                }
            });
        }
    }
    #[test]
    fn class_marking_matches_stream_states_with_cache_chunks() {
        for start in [1_000_000_000usize, 1_000_000_000_000] {
            let low = start / 30;
            let states: Vec<_> = super::super::STATIC_SEEDS
                .iter()
                .copied()
                .filter(|&p| {
                    p > 167 && p as usize <= start.isqrt().min(super::super::DENSE_PRIME_LIMIT)
                })
                .map(|p| super::super::dense_prime(p as usize, low))
                .collect();
            let mut expected_states = states.clone();
            let mut expected = vec![255; super::super::SEGMENT_SIZE];
            for state in &mut expected_states {
                super::super::mark_dense(state, &mut expected);
            }
            for chunk in [32 * 1024, super::super::SEGMENT_SIZE] {
                let mut actual_states = states.clone();
                let mut actual = vec![255; expected.len()];
                for bitmap in actual.chunks_mut(chunk) {
                    mark_classes(&mut actual_states, bitmap);
                }
                assert_eq!(actual, expected);
                for (a, e) in actual_states.iter().zip(&expected_states) {
                    assert_eq!(a.next, e.next);
                    assert_eq!(a.first, e.first);
                }
            }
        }
    }

    #[test]
    #[ignore = "native dense-kernel selection experiment"]
    fn profile_dense() {
        use std::hint::black_box;
        use std::time::Instant;
        for start in [1_000_000_000usize, 1_000_000_000_000] {
            let low = start / 30;
            let initial: Vec<_> = super::super::STATIC_SEEDS
                .iter()
                .copied()
                .filter(|&p| {
                    p > 167 && p as usize <= start.isqrt().min(super::super::DENSE_PRIME_LIMIT)
                })
                .map(|p| super::super::dense_prime(p as usize, low))
                .collect();
            let mut states = initial.clone();
            let mut bitmap = vec![255; super::super::SEGMENT_SIZE];
            for round in 0..7 {
                for offset in 0..4 {
                    let variant = (round + offset) % 4;
                    let timer = Instant::now();
                    for _ in 0..50 {
                        states.copy_from_slice(&initial);
                        bitmap.fill(255);
                        let chunk = if variant >= 2 {
                            32 * 1024
                        } else {
                            bitmap.len()
                        };
                        for bytes in bitmap.chunks_mut(chunk) {
                            if variant % 2 == 0 {
                                mark_reference(&mut states, bytes);
                            } else {
                                mark_classes(&mut states, bytes);
                            }
                        }
                        black_box(&bitmap);
                    }
                    println!(
                        "dense start={start} round={round} variant={variant} ns={}",
                        timer.elapsed().as_nanos() / 50
                    );
                }
            }
        }
    }
    fn medium_states() -> Vec<super::super::SparsePrime> {
        let low = 1_000_000_000_000usize / 30;
        super::super::STATIC_SEEDS
            .iter()
            .copied()
            .filter(|&p| p as usize > DENSE_LIMIT && p as usize <= 1_000_000)
            .map(|p| {
                let mut state = super::super::SparsePrime::new(p as usize, low);
                state.next -= low;
                state
            })
            .collect()
    }

    #[test]
    fn grouped_medium_matches_packed_steps_across_windows() {
        let mut reference = medium_states();
        let mut groups = MediumGroups::new();
        for state in &reference {
            groups.activate(state);
        }
        for len in [131_072, 131_072, 131_072, 131_072, 31, 0, 131_072, 131_071] {
            let mut expected = vec![255; len];
            for state in &mut reference {
                super::super::mark_medium(state, &mut expected);
            }
            let mut actual = vec![255; len];
            groups.mark(&mut actual);
            assert_eq!(actual, expected);
            let mut states: Vec<_> = groups
                .current
                .iter()
                .enumerate()
                .flat_map(|(wheel, group)| {
                    group
                        .iter()
                        .map(move |s| (s.quotient, wheel / 8, wheel, s.next as usize))
                })
                .collect();
            states.sort_unstable();
            let mut target: Vec<_> = reference
                .iter()
                .map(|s| (s.quotient, s.wheel as usize / 8, s.wheel as usize, s.next))
                .collect();
            target.sort_unstable();
            assert_eq!(states, target);
        }
    }

    #[test]
    #[ignore = "native medium-kernel selection experiment"]
    fn profile_medium() {
        use std::hint::black_box;
        use std::time::Instant;
        for round in 0..7 {
            for variant in [round % 2, (round + 1) % 2] {
                let mut states = medium_states();
                let mut groups = MediumGroups::new();
                for state in &states {
                    groups.activate(state);
                }
                let mut bitmap = vec![255; 131_072];
                let timer = Instant::now();
                for _ in 0..32 {
                    bitmap.fill(255);
                    if variant == 0 {
                        for state in &mut states {
                            super::super::mark_medium(state, &mut bitmap);
                        }
                    } else {
                        groups.mark(&mut bitmap);
                    }
                    black_box(&bitmap);
                }
                println!(
                    "medium round={round} variant={variant} ns={}",
                    timer.elapsed().as_nanos() / 32
                );
            }
        }
    }
    #[test]
    fn medium_square_activation_matches_single_window_queries() {
        let p = *super::super::STATIC_SEEDS
            .iter()
            .find(|&&p| p as usize > DENSE_LIMIT)
            .unwrap() as usize;
        let start = p * p - 2_000_000;
        let end = start + 40_000_000;
        let mut expected = Vec::new();
        let mut low = start;
        while low <= end {
            let high = (low + 999_999).min(end);
            expected.extend(super::super::primes_in_range(low, high));
            low = high + 1;
        }
        assert_eq!(super::super::primes_in_range(start, end), expected);
        assert_eq!(
            super::super::count_primes_in_range(start, end),
            expected.len()
        );
    }
    #[test]
    fn short_multi_window_queries_match_single_window_chunks() {
        let start = 1_000_000_000_017usize;
        let end = start + 15_999_999;
        let mut expected = Vec::new();
        for low in (start..=end).step_by(1_000_000) {
            expected.extend(super::super::primes_in_range(low, (low + 999_999).min(end)));
        }
        assert_eq!(super::super::primes_in_range(start, end), expected);
        assert_eq!(
            super::super::count_primes_in_range(start, end),
            expected.len()
        );
    }
    #[test]
    fn grouped_dense_matches_independent_streams() {
        for start in [1_000_000_000usize, 1_000_000_000_000] {
            let low = start / 30;
            let initial: Vec<_> = super::super::STATIC_SEEDS
                .iter()
                .copied()
                .filter(|&p| {
                    p > 167 && p as usize <= start.isqrt().min(super::super::DENSE_PRIME_LIMIT)
                })
                .map(|p| super::super::dense_prime(p as usize, low))
                .collect();
            let mut reference = initial.clone();
            let mut groups = DenseGroups::new();
            let mut portable = DenseGroups::new();
            for state in initial {
                portable.activate(state);
                groups.activate(state);
            }
            for len in [131_072, 131_071, 31, 0, 131_072] {
                let mut expected = vec![255; len];
                mark_reference(&mut reference, &mut expected);
                let mut actual = vec![255; len];
                groups.mark(&mut actual);
                assert_eq!(actual, expected);
                let mut fallback = vec![255; len];
                let markers = [
                    mark_canonical_group::<0>,
                    mark_canonical_group::<1>,
                    mark_canonical_group::<2>,
                    mark_canonical_group::<3>,
                    mark_canonical_group::<4>,
                    mark_canonical_group::<5>,
                    mark_canonical_group::<6>,
                    mark_canonical_group::<7>,
                ];
                for (class, marker) in markers.into_iter().enumerate() {
                    for chunk in fallback.chunks_mut(CACHE_BLOCK) {
                        marker(&mut portable.groups[class], chunk);
                    }
                    marker(&mut portable.larger[class], &mut fallback);
                }
                assert_eq!(fallback, expected);
                // The packed state stores the minimum offset and wheel.
                let mut states: Vec<_> = groups
                    .groups
                    .iter()
                    .chain(groups.larger.iter())
                    .enumerate()
                    .flat_map(|(class, states)| {
                        let class = class % 8;
                        states.iter().map(move |s| {
                            (
                                30 * s.quotient + RESIDUES[class] as u32,
                                s.next_wheel & DENSE_OFFSET_MASK,
                                (s.next_wheel >> 29) as u8,
                            )
                        })
                    })
                    .collect();
                states.sort_unstable_by_key(|s| s.0);
                let target: Vec<_> = reference
                    .iter()
                    .map(|s| (s.prime, s.next[s.first as usize], s.first))
                    .collect();
                assert_eq!(states, target);
                let mut fallback_states: Vec<_> = portable
                    .groups
                    .iter()
                    .chain(portable.larger.iter())
                    .enumerate()
                    .flat_map(|(class, states)| {
                        states.iter().map(move |s| {
                            (
                                30 * s.quotient + RESIDUES[class % 8] as u32,
                                s.next_wheel & DENSE_OFFSET_MASK,
                                (s.next_wheel >> 29) as u8,
                            )
                        })
                    })
                    .collect();
                fallback_states.sort_unstable_by_key(|s| s.0);
                assert_eq!(fallback_states, target);
            }
        }
    }

    #[test]
    #[ignore = "native grouped dense-kernel selection experiment"]
    fn profile_grouped_dense() {
        use std::hint::black_box;
        use std::time::Instant;
        for start in [1_000_000_000usize, 1_000_000_000_000] {
            let low = start / 30;
            let initial: Vec<_> = super::super::STATIC_SEEDS
                .iter()
                .copied()
                .filter(|&p| {
                    p > 167 && p as usize <= start.isqrt().min(super::super::DENSE_PRIME_LIMIT)
                })
                .map(|p| super::super::dense_prime(p as usize, low))
                .collect();
            for round in 0..7 {
                for offset in 0..3 {
                    let variant = (round + offset) % 3;
                    let mut states = initial.clone();
                    let mut groups = DenseGroups::new();
                    for state in &initial {
                        groups.activate(*state);
                    }
                    let mut bitmap = vec![255; 131_072];
                    let timer = Instant::now();
                    for _ in 0..32 {
                        bitmap.fill(255);
                        match variant {
                            0 => mark_reference(&mut states, &mut bitmap),
                            1 => {
                                for bytes in bitmap.chunks_mut(32 * 1024) {
                                    mark_reference(&mut states, bytes);
                                }
                            }
                            _ => groups.mark(&mut bitmap),
                        }
                        black_box(&bitmap);
                    }
                    println!(
                        "grouped_dense start={start} round={round} variant={variant} ns={}",
                        timer.elapsed().as_nanos() / 32
                    );
                }
            }
        }
    }
}
