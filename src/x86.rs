//! Runtime-selected x86 kernels. The crate's default target remains portable.
use super::{DECODED_WORDS, DecodedWord, RESIDUES};
use std::arch::x86_64::*;

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

#[cfg(test)]
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
}
