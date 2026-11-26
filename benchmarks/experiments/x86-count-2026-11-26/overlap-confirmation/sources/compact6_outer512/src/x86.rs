//! Runtime-selected x86 kernels. The crate's default target remains portable.
use super::{DECODED_WORDS, DecodedWord, RESIDUES};
use std::arch::x86_64::*;

pub(super) const SEGMENT_BYTES: usize = 512 * 1024;
const CACHE_BLOCK: usize = 32 * 1024;
const VECTOR_STATE_ADVANCE: bool = false;
const QUEUED_DENSE_MIN: usize = 6 * 1024;

pub(super) const COMPACT_PRE_SIEVE: bool = true;
const WIDE_PRE_SIEVE: bool = false;
const OVERLAP_PRE_SIEVE_TAIL: bool = true;
// Balance periods so each three-input pass keeps its cyclic tables in L1.
static COMPACT_0: [u8; 23 * 167] = super::pre_sieve([23, 167]);
static COMPACT_1: [u8; 13 * 17 * 19] = super::pre_sieve([13, 17, 19]);
static COMPACT_2: [u8; 29 * 163] = super::pre_sieve([29, 163]);
static COMPACT_3: [u8; 31 * 157] = super::pre_sieve([31, 157]);
static COMPACT_4: [u8; 37 * 151] = super::pre_sieve([37, 151]);
static COMPACT_5: [u8; 43 * 139] = super::pre_sieve([43, 139]);
static COMPACT_6: [u8; 41 * 149] = super::pre_sieve([41, 149]);
static COMPACT_7: [u8; 7 * 11 * 83] = super::pre_sieve([7, 11, 83]);
static COMPACT_8: [u8; 47 * 137] = super::pre_sieve([47, 137]);
static COMPACT_9: [u8; 61 * 113] = super::pre_sieve([61, 113]);
static COMPACT_10: [u8; 53 * 131] = super::pre_sieve([53, 131]);
static COMPACT_11: [u8; 67 * 109] = super::pre_sieve([67, 109]);
static COMPACT_12: [u8; 59 * 127] = super::pre_sieve([59, 127]);
static COMPACT_13: [u8; 73 * 103] = super::pre_sieve([73, 103]);
static COMPACT_14: [u8; 71 * 107] = super::pre_sieve([71, 107]);
static COMPACT_15: [u8; 79 * 101] = super::pre_sieve([79, 101]);
static COMPACT_16: [u8; 89 * 97] = super::pre_sieve([89, 97]);
const COMPACT_PATTERNS: [&[u8]; 17] = [
    &COMPACT_0,
    &COMPACT_1,
    &COMPACT_2,
    &COMPACT_3,
    &COMPACT_4,
    &COMPACT_5,
    &COMPACT_6,
    &COMPACT_7,
    &COMPACT_8,
    &COMPACT_9,
    &COMPACT_10,
    &COMPACT_11,
    &COMPACT_12,
    &COMPACT_13,
    &COMPACT_14,
    &COMPACT_15,
    &COMPACT_16,
];

#[target_feature(enable = "avx2")]
pub(super) unsafe fn fill_compact(low: usize, output: &mut [u8]) {
    if WIDE_PRE_SIEVE && std::arch::is_x86_feature_detected!("avx512f") {
        // SAFETY: AVX-512F detected; no BW operations are used.
        unsafe { fill_compact512(low, output) };
    } else {
        // SAFETY: AVX2 is enabled for this entry point.
        unsafe { fill_compact256(low, output) };
    }
}
#[target_feature(enable = "avx2")]
unsafe fn fill_compact256(low: usize, output: &mut [u8]) {
    // SAFETY: callers enable AVX2; cyclic helpers split at every input wrap.
    unsafe { compact_passes::<false>(low, output) };
}
#[target_feature(enable = "avx512f")]
unsafe fn fill_compact512(low: usize, output: &mut [u8]) {
    // SAFETY: callers enable AVX-512F; all 512-bit operations need only F.
    unsafe { compact_passes::<true>(low, output) };
}
#[inline(always)]
unsafe fn compact_passes<const WIDE: bool>(low: usize, output: &mut [u8]) {
    let p = COMPACT_PATTERNS;
    // SAFETY: the selected ISA is enabled by the caller, and groups have N inputs.
    unsafe {
        compact_cyclic::<3, true, WIDE>(low, output, p[..3].try_into().unwrap());
        compact_cyclic::<3, false, WIDE>(low, output, p[3..6].try_into().unwrap());
        compact_cyclic::<3, false, WIDE>(low, output, p[6..9].try_into().unwrap());
        compact_cyclic::<3, false, WIDE>(low, output, p[9..12].try_into().unwrap());
        compact_cyclic::<3, false, WIDE>(low, output, p[12..15].try_into().unwrap());
        compact_cyclic::<2, false, WIDE>(low, output, p[15..].try_into().unwrap());
    }
}
#[inline(always)]
unsafe fn compact_cyclic<const N: usize, const INIT: bool, const WIDE: bool>(
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
        // SAFETY: every input has chunk.len() bytes; caller enables the chosen ISA.
        unsafe {
            if WIDE {
                compact_combine512::<N, INIT>(chunk, inputs);
            } else {
                compact_combine256::<N, INIT>(chunk, inputs);
            }
        }
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
#[inline]
unsafe fn compact_combine256<const N: usize, const INIT: bool>(
    output: &mut [u8],
    inputs: [&[u8]; N],
) {
    let vectors = output.len() / 32 * 32;
    macro_rules! block {
        ($offset:expr) => {{
            let offset = $offset;
            // SAFETY: all slices contain offset + 32 bytes; AVX2 enabled.
            unsafe {
                let mut bits = if INIT {
                    _mm256_set1_epi8(-1)
                } else {
                    _mm256_loadu_si256(output.as_ptr().add(offset).cast())
                };
                for input in inputs {
                    bits = _mm256_and_si256(
                        bits,
                        _mm256_loadu_si256(input.as_ptr().add(offset).cast()),
                    );
                }
                _mm256_storeu_si256(output.as_mut_ptr().add(offset).cast(), bits);
            }
        }};
    }
    for offset in (0..vectors).step_by(32) {
        block!(offset);
    }
    if vectors < output.len() {
        if OVERLAP_PRE_SIEVE_TAIL && output.len() >= 32 {
            // INIT recomputes identical masks; later passes use idempotent AND.
            // The overlapping vector remains inside every input and output.
            block!(output.len() - 32);
        } else {
            compact_tail::<N, INIT>(output, inputs, vectors);
        }
    }
}
#[target_feature(enable = "avx512f")]
#[inline]
unsafe fn compact_combine512<const N: usize, const INIT: bool>(
    output: &mut [u8],
    inputs: [&[u8]; N],
) {
    let vectors = output.len() / 64 * 64;
    macro_rules! block {
        ($offset:expr) => {{
            let offset = $offset;
            // SAFETY: all slices contain offset + 64 bytes; only AVX-512F is used.
            unsafe {
                let mut bits = if INIT {
                    _mm512_set1_epi32(-1)
                } else {
                    _mm512_loadu_si512(output.as_ptr().add(offset).cast())
                };
                let mut source = 0;
                while source + 1 < N {
                    let a = _mm512_loadu_si512(inputs[source].as_ptr().add(offset).cast());
                    let b = _mm512_loadu_si512(inputs[source + 1].as_ptr().add(offset).cast());
                    bits = _mm512_ternarylogic_epi32::<0x80>(bits, a, b);
                    source += 2;
                }
                if source < N {
                    bits = _mm512_and_si512(
                        bits,
                        _mm512_loadu_si512(inputs[source].as_ptr().add(offset).cast()),
                    );
                }
                _mm512_storeu_si512(output.as_mut_ptr().add(offset).cast(), bits);
            }
        }};
    }
    for offset in (0..vectors).step_by(64) {
        block!(offset);
    }
    if vectors < output.len() {
        if OVERLAP_PRE_SIEVE_TAIL && output.len() >= 64 {
            // INIT recomputes identical masks; later passes use idempotent AND.
            // The overlapping vector remains inside every input and output.
            block!(output.len() - 64);
        } else {
            compact_tail::<N, INIT>(output, inputs, vectors);
        }
    }
}
#[inline]
fn compact_tail<const N: usize, const INIT: bool>(
    output: &mut [u8],
    inputs: [&[u8]; N],
    start: usize,
) {
    for offset in start..output.len() {
        let mut bits = if INIT { 255 } else { output[offset] };
        for input in inputs {
            bits &= input[offset];
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

#[derive(Clone)]
pub(super) struct DenseGroups {
    groups: [Vec<super::SievingPrime>; 8],
    larger: [Vec<super::SievingPrime>; 8],
    queued: MediumGroups,
}
impl DenseGroups {
    pub(super) fn new() -> Self {
        Self {
            groups: std::array::from_fn(|_| Vec::new()),
            larger: std::array::from_fn(|_| Vec::new()),
            queued: MediumGroups::new(),
        }
    }
    pub(super) fn activate(&mut self, state: super::SievingPrime) {
        let class = super::RESIDUE_INDEX[state.prime as usize % 30];
        if state.prime as usize > QUEUED_DENSE_MIN {
            self.queued.current[class * 8 + state.first as usize].push(MediumPrime {
                quotient: state.prime / 30,
                next: state.next[state.first as usize],
            });
            return;
        }
        if state.prime <= 32 * 1024 {
            self.groups[class].push(state);
        } else {
            self.larger[class].push(state);
        }
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
        // Larger primes have few hits per window. Visiting their state four
        // times costs more than the cache locality saves, so advance them once.
        mark_dense_group::<0>(&mut self.larger[0], sieve);
        mark_dense_group::<1>(&mut self.larger[1], sieve);
        mark_dense_group::<2>(&mut self.larger[2], sieve);
        mark_dense_group::<3>(&mut self.larger[3], sieve);
        mark_dense_group::<4>(&mut self.larger[4], sieve);
        mark_dense_group::<5>(&mut self.larger[5], sieve);
        mark_dense_group::<6>(&mut self.larger[6], sieve);
        mark_dense_group::<7>(&mut self.larger[7], sieve);
        self.queued.mark(sieve);
    }
    #[cfg(test)]
    fn normalized_states(&self) -> Vec<(u32, u32, u8)> {
        let mut states: Vec<_> = self
            .groups
            .iter()
            .chain(self.larger.iter())
            .flatten()
            .map(|s| (s.prime, s.next[s.first as usize], s.first))
            .collect();
        states.extend(
            self.queued
                .current
                .iter()
                .enumerate()
                .flat_map(|(wheel, states)| {
                    states.iter().map(move |s| {
                        (
                            s.quotient * 30 + RESIDUES[wheel / 8] as u32,
                            s.next,
                            (wheel % 8) as u8,
                        )
                    })
                }),
        );
        states.sort_unstable_by_key(|s| s.0);
        states
    }
}
#[inline(never)]
fn mark_dense_group<const CLASS: usize>(states: &mut [super::SievingPrime], sieve: &mut [u8]) {
    if VECTOR_STATE_ADVANCE && std::arch::is_x86_feature_detected!("avx2") {
        // SAFETY: AVX2 detected for this group.
        unsafe { mark_dense_group_avx2::<CLASS>(states, sieve) };
        return;
    }
    for state in states {
        mark_dense_class::<CLASS>(state, sieve);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn mark_dense_group_avx2<const CLASS: usize>(
    states: &mut [super::SievingPrime],
    sieve: &mut [u8],
) {
    let lanes = _mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7);
    let seven = _mm256_set1_epi32(7);
    let len = sieve.len();
    let [ma, mb, mc, md, me, mf, mg, mh] = super::WHEEL_MASKS[CLASS];
    for state in states {
        let p = state.prime as usize;
        let first = (state.first as usize) & 7;
        let offsets = state.next;
        // SAFETY: offsets contains exactly eight 32-bit entries.
        let original = unsafe { _mm256_loadu_si256(offsets.as_ptr().cast()) };
        let [a, b, c, d, e, f, g, h] = offsets.map(|n| n as usize);
        let max = offsets[(first + 7) & 7] as usize;
        let end = len.saturating_sub(max);
        let mut advance = 0usize;
        while advance < end {
            // SAFETY: the predecessor of first is the maximum stream offset.
            // advance < len - max bounds all eight accesses simultaneously.
            unsafe {
                *sieve.get_unchecked_mut(advance + a) &= ma;
                *sieve.get_unchecked_mut(advance + b) &= mb;
                *sieve.get_unchecked_mut(advance + c) &= mc;
                *sieve.get_unchecked_mut(advance + d) &= md;
                *sieve.get_unchecked_mut(advance + e) &= me;
                *sieve.get_unchecked_mut(advance + f) &= mf;
                *sieve.get_unchecked_mut(advance + g) &= mg;
                *sieve.get_unchecked_mut(advance + h) &= mh;
            }
            advance += p;
        }
        let mut next = first;
        let mut marked = 0;
        while offsets[next] as usize + advance < len {
            let index = offsets[next] as usize + advance;
            sieve[index] &= super::WHEEL_MASKS[CLASS][next];
            next = (next + 1) & 7;
            marked += 1;
        }
        // The failed full-wheel condition guarantees at least one unmarked
        // stream, so the tail never revisits a lane. The marked lanes are the
        // cyclic prefix starting at first; add p only to those lanes.
        debug_assert!(marked < 8);
        let distances = _mm256_and_si256(
            _mm256_sub_epi32(lanes, _mm256_set1_epi32(first as i32)),
            seven,
        );
        let marked_lanes = _mm256_cmpgt_epi32(_mm256_set1_epi32(marked), distances);
        let delta = (advance as u32).wrapping_sub(len as u32);
        let adjusted = _mm256_add_epi32(
            _mm256_add_epi32(original, _mm256_set1_epi32(delta as i32)),
            _mm256_and_si256(marked_lanes, _mm256_set1_epi32(p as i32)),
        );
        // SAFETY: state.next provides 32 writable bytes. Intermediate vector
        // subtraction wraps intentionally; every exposed next offset is < p.
        unsafe { _mm256_storeu_si256(state.next.as_mut_ptr().cast(), adjusted) };
        state.first = next as u8;
    }
}

#[derive(Clone, Copy)]
struct MediumPrime {
    next: u32,
    quotient: u32,
}

#[derive(Clone)]
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
        // their next step (< 210 KiB), far below u32::MAX.
        self.current[state.wheel as usize].push(MediumPrime {
            next: u32::try_from(state.next).unwrap(),
            quotient: state.quotient,
        });
    }
    pub(super) fn mark(&mut self, sieve: &mut [u8]) {
        mark_medium_class::<0>(&self.current, &mut self.next, sieve);
        mark_medium_class::<1>(&self.current, &mut self.next, sieve);
        mark_medium_class::<2>(&self.current, &mut self.next, sieve);
        mark_medium_class::<3>(&self.current, &mut self.next, sieve);
        mark_medium_class::<4>(&self.current, &mut self.next, sieve);
        mark_medium_class::<5>(&self.current, &mut self.next, sieve);
        mark_medium_class::<6>(&self.current, &mut self.next, sieve);
        mark_medium_class::<7>(&self.current, &mut self.next, sieve);
        for states in &mut self.current {
            states.clear();
        }
        std::mem::swap(&mut self.current, &mut self.next);
    }
}

// Sorted wheel entry points share one full-cycle body per residue class.
// Canonical traversal follows primesieve EratSmall; BSD-2-Clause notice applies.
#[inline(never)]
fn mark_medium_class<const CLASS: usize>(
    current: &[Vec<MediumPrime>; 64],
    next: &mut [Vec<MediumPrime>; 64],
    sieve: &mut [u8],
) {
    for first in 0..8 {
        for state in &current[CLASS * 8 + first] {
            let (offset, wheel) = medium_class_chain::<CLASS>(*state, first, sieve);
            next[CLASS * 8 + wheel].push(MediumPrime {
                next: offset,
                quotient: state.quotient,
            });
        }
    }
}
#[inline(always)]
fn medium_class_chain<const CLASS: usize>(
    state: MediumPrime,
    first: usize,
    sieve: &mut [u8],
) -> (u32, usize) {
    let mut i = state.next as usize;
    let q = state.quotient as usize;
    let len = sieve.len();
    let steps = const { medium_canonical_steps(CLASS) };
    if i >= len {
        return ((i - len) as u32, first);
    }
    macro_rules! hit {
        ($index:expr) => {{
            if i >= len {
                return ((i - len) as u32, $index);
            }
            let step = super::WHEEL_STEPS[CLASS * 8 + $index];
            // SAFETY: the preceding check establishes i < len.
            unsafe {
                *sieve.get_unchecked_mut(i) &= step.mask;
            }
            i += q * step.gap as usize + step.carry as usize;
        }};
    }
    'prefix_done: {
        'p7: {
            'p6: {
                'p5: {
                    'p4: {
                        'p3: {
                            'p2: {
                                'p1: {
                                    match first {
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
        let prime = q * 30 + RESIDUES[CLASS];
        let end = len - h;
        while i < end {
            // SAFETY: offsets are nondecreasing and h is their maximum.
            // i < len - h bounds all eight accesses simultaneously.
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
            i += prime;
        }
    }
    hit!(0);
    hit!(1);
    hit!(2);
    hit!(3);
    hit!(4);
    hit!(5);
    hit!(6);
    debug_assert!(i >= len);
    ((i - len) as u32, 7)
}
const fn medium_canonical_steps(class: usize) -> [(usize, usize, u8); 8] {
    let mut values = [(0, 0, 0); 8];
    let (mut gap, mut carry, mut index) = (0, 0, 0);
    while index < 8 {
        let step = super::WHEEL_STEPS[class * 8 + index];
        values[index] = (gap, carry, step.mask);
        gap += step.gap as usize;
        carry += step.carry as usize;
        index += 1;
    }
    values
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
    fn balanced_compact_masks_match_original_tables_and_guard_bytes() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }
        for low in [0, 1, 3839, 10_000_003, usize::MAX / 30 - 131_072] {
            for len in [
                0, 1, 31, 32, 33, 63, 64, 65, 511, 4095, 32_767, 131_071, 524_288,
            ] {
                let mut expected = vec![0; len];
                super::super::fill_segment(low, &mut expected);
                let mut storage = vec![0xa5; 17 + len + 33];
                // SAFETY: AVX2 checked and the output slice has exactly len bytes.
                unsafe {
                    fill_compact256(low, &mut storage[17..17 + len]);
                }
                assert_eq!(&storage[17..17 + len], expected);
                assert!(
                    storage[..17]
                        .iter()
                        .chain(storage[17 + len..].iter())
                        .all(|&b| b == 0xa5)
                );
                if std::arch::is_x86_feature_detected!("avx512f") {
                    storage.fill(0xa5);
                    // SAFETY: AVX-512F checked; guards must remain unchanged.
                    unsafe {
                        fill_compact512(low, &mut storage[17..17 + len]);
                    }
                    assert_eq!(&storage[17..17 + len], expected);
                    assert!(
                        storage[..17]
                            .iter()
                            .chain(storage[17 + len..].iter())
                            .all(|&b| b == 0xa5)
                    );
                }
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
    fn dense_tails_match_scalar_for_every_starting_wheel() {
        for class in 0..8 {
            for floor in [173, 100_000] {
                let p = *super::super::STATIC_SEEDS
                    .iter()
                    .find(|&&p| {
                        p as usize >= floor && super::super::RESIDUE_INDEX[p as usize % 30] == class
                    })
                    .unwrap() as usize;
                for first in 0..8 {
                    for delta in [0, p - 1] {
                        let mut initial = super::super::SievingPrime {
                            prime: p as u32,
                            next: [0; 8],
                            masks: super::super::WHEEL_MASKS[class],
                            first: first as u8,
                        };
                        let mut offset = delta;
                        for n in 0..8 {
                            let index = (first + n) & 7;
                            initial.next[index] = offset as u32;
                            let step = super::super::WHEEL_STEPS[class * 8 + index];
                            offset += (p / 30) * step.gap as usize + step.carry as usize;
                        }
                        for len in [0, 1, 31, p - 1, p, p + 1, 32_768, 131_072] {
                            let (mut actual, mut expected) = (initial, initial);
                            let mut a = vec![255; len];
                            let mut b = a.clone();
                            mark_classes(std::slice::from_mut(&mut actual), &mut a);
                            super::super::mark_dense(&mut expected, &mut b);
                            assert_eq!(
                                a, b,
                                "class={class}, p={p}, first={first}, delta={delta}, len={len}"
                            );
                            assert_eq!(actual.next, expected.next);
                            assert_eq!(actual.first, expected.first);
                            let mut groups = DenseGroups::new();
                            groups.activate(initial);
                            let mut grouped = vec![255; len];
                            groups.mark(&mut grouped);
                            assert_eq!(grouped, b);
                            assert_eq!(
                                groups.normalized_states(),
                                vec![(
                                    expected.prime,
                                    expected.next[expected.first as usize],
                                    expected.first
                                )]
                            );
                        }
                    }
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
            .filter(|&p| {
                p as usize > QUEUED_DENSE_MIN.min(super::super::DENSE_PRIME_LIMIT)
                    && p as usize <= 1_000_000
            })
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
        let start = 131_101usize * 131_101 - 2_000_000;
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
            for state in initial {
                groups.activate(state);
            }
            for len in [131_072, 131_071, 31, 0, 131_072] {
                let mut expected = vec![255; len];
                mark_reference(&mut reference, &mut expected);
                let mut actual = vec![255; len];
                groups.mark(&mut actual);
                assert_eq!(actual, expected);
                let states = groups.normalized_states();
                let target: Vec<_> = reference
                    .iter()
                    .map(|s| (s.prime, s.next[s.first as usize], s.first))
                    .collect();
                assert_eq!(states, target);
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
