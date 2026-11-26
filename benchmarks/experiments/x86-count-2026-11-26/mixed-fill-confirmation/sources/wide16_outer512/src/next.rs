//! Exact u64 primality and single-threaded batch next-prime queries.

use crate::candidates::FILTER_PRIMES;

/// Montgomery arithmetic with R=2^64, for odd moduli > 1.
struct Montgomery {
    modulus: u64,
    negative_inverse: u64,
    r2: u64,
    one: u64,
}

impl Montgomery {
    fn new(modulus: u64) -> Self {
        let mut inverse = 1u64;
        for _ in 0..6 {
            inverse = inverse.wrapping_mul(2u64.wrapping_sub(modulus.wrapping_mul(inverse)));
        }
        let one = ((1u128 << 64) % modulus as u128) as u64;
        let r2 = (one as u128 * one as u128 % modulus as u128) as u64;
        Self {
            modulus,
            negative_inverse: inverse.wrapping_neg(),
            r2,
            one,
        }
    }

    #[inline]
    fn multiply(&self, a: u64, b: u64) -> u64 {
        let product = a as u128 * b as u128;
        let correction = (product as u64).wrapping_mul(self.negative_inverse);
        let (sum, overflow) = product.overflowing_add(correction as u128 * self.modulus as u128);
        let high = (sum >> 64) as u64;
        // The mathematical quotient is < 2*modulus. If the 128-bit addition
        // overflowed, its quotient has an extra 2^64; wrapping subtraction
        // removes the modulus and represents the correct reduced result.
        if overflow {
            high.wrapping_sub(self.modulus)
        } else if high >= self.modulus {
            high - self.modulus
        } else {
            high
        }
    }

    fn encode(&self, value: u64) -> u64 {
        self.multiply(value % self.modulus, self.r2)
    }

    fn power(&self, base: u64, mut exponent: u64) -> u64 {
        let mut base = self.encode(base);
        let mut result = self.one;
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = self.multiply(result, base);
            }
            exponent >>= 1;
            if exponent != 0 {
                base = self.multiply(base, base);
            }
        }
        result
    }
}

fn miller_rabin(n: u64) -> bool {
    if n <= u32::MAX as u64 {
        return miller_rabin32(n);
    }
    // Complete deterministic witness set for n < 2^64:
    // https://miller-rabin.appspot.com/ (Jim Sinclair's seven-base set).
    const BASES: [u64; 7] = [2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022];
    const SMALL_BASES: [u64; 3] = [2, 7, 61];
    let bases: &[u64] = if n < 4_759_123_141 {
        &SMALL_BASES
    } else {
        &BASES
    };
    let powers_of_two = (n - 1).trailing_zeros();
    let odd_part = (n - 1) >> powers_of_two;
    let arithmetic = Montgomery::new(n);
    let minus_one = n - arithmetic.one;
    'witness: for &base in bases {
        if base.is_multiple_of(n) {
            continue;
        }
        let mut x = arithmetic.power(base, odd_part);
        if x == arithmetic.one || x == minus_one {
            continue;
        }
        for _ in 1..powers_of_two {
            x = arithmetic.multiply(x, x);
            if x == minus_one {
                continue 'witness;
            }
            if x == arithmetic.one {
                return false;
            }
        }
        return false;
    }
    true
}

fn miller_rabin32(n: u64) -> bool {
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'witness: for base in [2, 7, 61] {
        let mut exponent = d;
        let mut base = base % n;
        let mut x = 1u64;
        while exponent != 0 {
            // Both operands are < n <= u32::MAX, so products fit in u64.
            if exponent & 1 != 0 {
                x = x * base % n;
            }
            exponent >>= 1;
            if exponent != 0 {
                base = base * base % n;
            }
        }
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = x * x % n;
            if x == n - 1 {
                continue 'witness;
            }
            if x == 1 {
                return false;
            }
        }
        return false;
    }
    true
}

/// Exact deterministic primality test over the entire u64 domain.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in FILTER_PRIMES {
        if n == p {
            return true;
        }
        if n.is_multiple_of(p) {
            return false;
        }
    }
    miller_rabin(n)
}

/// Returns the smallest prime strictly greater than n, or None if it would
/// exceed u64::MAX. Uses a wheel, small-prime filtering and exact Miller–Rabin.
pub fn next_prime(n: u64) -> Option<u64> {
    crate::candidates::next_by(n, miller_rabin)
}

/// Selection policy. Sieve still respects the span, seed-root and memory limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BatchStrategy {
    #[default]
    Auto,
    Direct,
    Sieve,
}

#[derive(Clone, Copy, Debug)]
pub struct BatchOptions {
    pub strategy: BatchStrategy,
    pub max_cluster_span: u64,
    pub max_cluster_gap: u64,
    pub max_sieve_root: u64,
    /// Extra interval coverage; unanswered tails use exact direct search.
    pub tail_padding: u64,
    /// Conservative working-set estimate, not an allocator-enforced RSS limit.
    pub max_sieve_bytes: usize,
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self {
            strategy: BatchStrategy::Auto,
            max_cluster_span: 16_000_000,
            max_cluster_gap: 65_536,
            max_sieve_root: 10_000_000,
            tail_padding: 512,
            max_sieve_bytes: 192 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BatchStats {
    pub unique_queries: usize,
    pub direct_groups: usize,
    pub sieve_groups: usize,
    pub direct_queries: usize,
    pub sieve_queries: usize,
    pub max_estimated_sieve_bytes: usize,
    pub tail_fallbacks: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BatchResult {
    pub answers: Vec<Option<u64>>,
    pub stats: BatchStats,
}

/// Returns strict next primes in the original order, deduplicating identical
/// inputs internally. No worker threads, persistent cache, or random tests.
pub fn batch_next_primes(numbers: &[u64]) -> Vec<Option<u64>> {
    batch_next_primes_with_options(numbers, BatchOptions::default()).answers
}

fn working_set(root: u64) -> usize {
    // Every prime >5 occupies one of eight residues per 30 numbers. Allow
    // growth headroom for both seed-vector and state-vector capacities, plus
    // the dense states, bitmap and recursive seed workspace. No full interval
    // prime list is materialized; query/output storage is accounted separately.
    let candidates = |n: u64| (n as u128 / 30 + 1) * 8 + 3;
    let bytes = 48 * candidates(root) + 160 * candidates(root.min(131_072)) + 8 * 1024 * 1024;
    bytes.min(usize::MAX as u128) as usize
}

fn select_sieve(first: u64, last: u64, count: usize, options: BatchOptions) -> (bool, usize, u64) {
    let end = last.saturating_add(options.tail_padding);
    let root = end.isqrt();
    let span = end.saturating_sub(first);
    let bytes = working_set(root);
    if options.strategy == BatchStrategy::Direct
        || first == u64::MAX
        || usize::try_from(end).is_err()
        || root > options.max_sieve_root
        || bytes > options.max_sieve_bytes
        || count < 2
    {
        return (false, bytes, end);
    }
    if options.strategy == BatchStrategy::Sieve {
        return (true, bytes, end);
    }

    // Portable heuristic defaults, deliberately visible and tested rather than
    // a claimed universal crossover. Root preparation and range traversal are
    // charged once; direct search accounts for reuse within the same prime gap.
    let log = (last.max(3) as f64).ln();
    let prime_gaps = ((last - first) as f64 / log).max(1.0);
    let searches =
        (1.0 + prime_gaps * (1.0 - (-(count as f64) / prime_gaps).exp())).min(count as f64);
    let word_cost = if last <= u32::MAX as u64 { 0.6 } else { 1.0 };
    let direct_ns = searches * (250.0 + 2.5 * log * log) * word_cost;
    let seed_primes = if root < 17 {
        root as f64
    } else {
        1.25506 * root as f64 / (root as f64).ln()
    };
    let sieve_ns =
        50_000.0 + 12.0 * seed_primes + span as f64 * (0.02 + 0.008 * (root.max(2) as f64).ln());
    // Avoid paying seed setup when a close cost prediction could go either way.
    (1.25 * sieve_ns < direct_ns, bytes, end)
}

// Answer sorted queries straight from the wheel bitmap. A range may contain
// hundreds of thousands of primes but only a few thousand requested answers;
// decoding and allocating the entire prime list is unnecessary here.
fn sieve_answers(queries: &[u64], answers: &mut [Option<u64>], stop: u64, seeds: &[u32]) -> usize {
    let mut cursor = 0;
    while cursor < queries.len() && queries[cursor] < 167 {
        let p = next_prime(queries[cursor]).unwrap();
        if p > stop {
            break;
        }
        answers[cursor] = Some(p);
        cursor += 1;
    }
    crate::sieve_segments_with_seeds(
        (queries[0] + 1) as usize,
        stop as usize,
        Some(seeds),
        |low, sieve| {
            while cursor < queries.len() {
                let Some(target) = queries[cursor]
                    .checked_add(1)
                    .and_then(|n| usize::try_from(n).ok())
                else {
                    break;
                };
                let target_byte = target / 30;
                let mut offset = target_byte.saturating_sub(low);
                if offset >= sieve.len() {
                    break;
                }
                let mut bits = sieve[offset];
                if target_byte >= low {
                    let skip = crate::RESIDUES.partition_point(|&r| r < target % 30);
                    bits &= (255u16 << skip) as u8;
                }
                if bits == 0 {
                    let Some(next) = sieve[offset + 1..].iter().position(|&byte| byte != 0) else {
                        break;
                    };
                    offset += next + 1;
                    bits = sieve[offset];
                }
                let p =
                    ((low + offset) * 30 + crate::RESIDUES[bits.trailing_zeros() as usize]) as u64;
                // All inputs in this prime gap share the same strict next prime.
                while cursor < queries.len() && queries[cursor] < p {
                    answers[cursor] = Some(p);
                    cursor += 1;
                }
            }
        },
    );
    let mut fallbacks = 0;
    let mut previous = None;
    for i in cursor..queries.len() {
        answers[i] = if previous.is_some_and(|p| p > queries[i]) {
            previous
        } else {
            fallbacks += 1;
            next_prime(queries[i])
        };
        previous = answers[i];
    }
    fallbacks
}

/// Batch engine with a configurable sparse/dense router and aggregate decisions.
/// Sorting, deduplication, route selection and original-order restoration are
/// all performed inside this call. Estimated-memory limits apply to sieving;
/// input/output storage remains O(number of queries).
pub fn batch_next_primes_with_options(numbers: &[u64], options: BatchOptions) -> BatchResult {
    let order = crate::batch_order::sorted_queries(numbers);
    let mut unique: Vec<_> = order.iter().map(|&(n, _)| n).collect();
    unique.dedup();
    let mut found = vec![None; unique.len()];
    let mut stats = BatchStats {
        unique_queries: unique.len(),
        ..BatchStats::default()
    };
    let mut groups = Vec::new();
    let mut seed_root = 0;
    let mut begin = 0;
    while begin < unique.len() {
        let mut end = begin + 1;
        while end < unique.len()
            && unique[end] - unique[end - 1] <= options.max_cluster_gap
            && unique[end] - unique[begin] <= options.max_cluster_span
        {
            end += 1;
        }
        let (sieve, bytes, stop) =
            select_sieve(unique[begin], unique[end - 1], end - begin, options);
        stats.max_estimated_sieve_bytes = stats.max_estimated_sieve_bytes.max(bytes);
        if sieve {
            seed_root = seed_root.max(stop.isqrt());
        }
        groups.push((begin, end, sieve, stop));
        begin = end;
    }
    // Prepared only for this batch. Nearby clusters share the seed table, but
    // each still initializes its own positions; nothing persists across calls.
    let seeds = if seed_root == 0 {
        Vec::new()
    } else {
        crate::seed_primes(seed_root as usize)
    };
    for (begin, end, sieve, stop) in groups {
        if sieve {
            stats.sieve_groups += 1;
            stats.sieve_queries += end - begin;
            stats.tail_fallbacks +=
                sieve_answers(&unique[begin..end], &mut found[begin..end], stop, &seeds);
        } else {
            stats.direct_groups += 1;
            stats.direct_queries += end - begin;
            let mut previous = None;
            for i in begin..end {
                found[i] = if previous.is_some_and(|p| p > unique[i]) {
                    previous
                } else {
                    next_prime(unique[i])
                };
                previous = found[i];
            }
        }
    }
    let mut answers = vec![None; numbers.len()];
    let mut cursor = 0;
    for (n, original) in order {
        while unique[cursor] != n {
            cursor += 1;
        }
        answers[original] = found[cursor];
    }
    BatchResult { answers, stats }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn montgomery_matches_wide_remainders() {
        let mut random = 42u64;
        for modulus in [
            3,
            67,
            73,
            1_000_000_007,
            1 << 63 | 1,
            u64::MAX - 58,
            u64::MAX,
        ] {
            let arithmetic = Montgomery::new(modulus);
            for _ in 0..2_000 {
                random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = random % modulus;
                random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
                let b = random % modulus;
                let encoded = arithmetic.multiply(arithmetic.encode(a), arithmetic.encode(b));
                assert_eq!(
                    arithmetic.multiply(encoded, 1),
                    (a as u128 * b as u128 % modulus as u128) as u64
                );
            }
            for a in [0, 1, modulus - 1] {
                for b in [0, 1, modulus - 1] {
                    let encoded = arithmetic.multiply(arithmetic.encode(a), arithmetic.encode(b));
                    assert_eq!(
                        arithmetic.multiply(encoded, 1),
                        (a as u128 * b as u128 % modulus as u128) as u64
                    );
                }
            }
        }
    }

    #[test]
    fn primality_matches_sieve_and_rejects_pseudoprimes() {
        let primes = crate::primes_in_range(0, 100_000);
        for n in 0..=100_000 {
            assert_eq!(
                is_prime(n),
                primes.binary_search(&(n as usize)).is_ok(),
                "n={n}"
            );
        }
        for n in [
            561,
            1_105,
            1_729,
            2_047,
            3_215_031_751,
            4_759_123_141,
            341_550_071_728_321,
            3_825_123_056_546_413_051,
            4_294_967_291u64 * 4_294_967_291,
            u64::MAX,
        ] {
            assert!(!is_prime(n), "composite {n}");
        }
        for n in [
            73,
            193,
            4_294_967_291,
            4_294_967_311,
            18_446_744_073_709_551_557,
        ] {
            assert!(is_prime(n), "prime {n}");
        }
    }

    #[test]
    fn strict_next_prime_and_overflow() {
        let primes = crate::primes_in_range(0, 101_000);
        for n in 0..100_000 {
            assert_eq!(
                next_prime(n),
                Some(primes[primes.partition_point(|&p| p <= n as usize)] as u64)
            );
        }
        let last = 18_446_744_073_709_551_557;
        assert_eq!(next_prime(last - 1), Some(last));
        assert_eq!(next_prime(last), None);
        assert_eq!(next_prime(u64::MAX), None);
    }

    #[test]
    fn batch_preserves_order_duplicates_and_resource_fallback() {
        let inputs = [100, 2, 0, 100, 101, u64::MAX, 18_446_744_073_709_551_556];
        let expected: Vec<_> = inputs.iter().map(|&n| next_prime(n)).collect();
        for strategy in [
            BatchStrategy::Auto,
            BatchStrategy::Direct,
            BatchStrategy::Sieve,
        ] {
            let result = batch_next_primes_with_options(
                &inputs,
                BatchOptions {
                    strategy,
                    ..BatchOptions::default()
                },
            );
            assert_eq!(result.answers, expected);
            assert_eq!(result.stats.unique_queries, 6);
        }
        assert!(batch_next_primes(&[]).is_empty());
        let inputs: Vec<_> = (100_000_000..100_000_008).collect();
        let result = batch_next_primes_with_options(
            &inputs,
            BatchOptions {
                strategy: BatchStrategy::Sieve,
                max_sieve_bytes: 0,
                ..BatchOptions::default()
            },
        );
        assert_eq!(result.stats.sieve_groups, 0);
        assert_eq!(
            result.answers,
            inputs.iter().map(|&n| next_prime(n)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn router_uses_both_engines_and_forced_sieve_agrees() {
        let mut inputs: Vec<_> = (100_000_000..100_000_000 + 8_000).collect();
        inputs.extend([1_000_000_000_000_000_000, u64::MAX, 9_000_000_000_000_000]);
        inputs.reverse();
        let result = batch_next_primes_with_options(&inputs, BatchOptions::default());
        assert!(result.stats.direct_groups > 0);
        assert!(result.stats.sieve_groups > 0);
        let direct = batch_next_primes_with_options(
            &inputs,
            BatchOptions {
                strategy: BatchStrategy::Direct,
                ..BatchOptions::default()
            },
        );
        assert_eq!(result.answers, direct.answers);
        let forced = batch_next_primes_with_options(
            &inputs,
            BatchOptions {
                strategy: BatchStrategy::Sieve,
                ..BatchOptions::default()
            },
        );
        assert_eq!(result.answers, forced.answers);
    }

    #[test]
    fn bitmap_answers_match_exact_search_across_segments_and_prime_boundaries() {
        for base in [0, 105_000_000, 1_000_005_000_000, 100_000_005_000_000] {
            if usize::try_from(base + 9_000_000).is_err() {
                continue;
            }
            let mut inputs: Vec<_> = (0..384).map(|i| base + i * 72_511 % 9_000_000).collect();
            let boundary = (base / 30 + crate::SEGMENT_SIZE as u64) * 30;
            inputs.extend((boundary - 50)..=boundary + 50);
            if base == 0 {
                inputs.extend(0..200);
            }
            inputs.extend_from_within(..32);
            inputs.reverse();
            let expected: Vec<_> = inputs.iter().map(|&n| next_prime(n)).collect();
            let span = inputs.iter().max().unwrap() - inputs.iter().min().unwrap();
            for tail_padding in [0, 512] {
                let result = batch_next_primes_with_options(
                    &inputs,
                    BatchOptions {
                        strategy: BatchStrategy::Sieve,
                        max_cluster_gap: span,
                        max_cluster_span: span,
                        max_sieve_root: inputs
                            .iter()
                            .max()
                            .unwrap()
                            .saturating_add(tail_padding)
                            .isqrt(),
                        tail_padding,
                        ..BatchOptions::default()
                    },
                );
                assert_eq!(
                    result.answers, expected,
                    "base={base}, padding={tail_padding}"
                );
                assert_eq!(result.stats.sieve_groups, 1);
                if tail_padding == 0 {
                    assert!(result.stats.tail_fallbacks > 0);
                }
            }
        }
    }

    #[test]
    fn sieve_tail_has_no_prime_gap_assumption() {
        let result = batch_next_primes_with_options(
            &[100, 101],
            BatchOptions {
                strategy: BatchStrategy::Sieve,
                tail_padding: 0,
                ..BatchOptions::default()
            },
        );
        assert_eq!(result.answers, [Some(101), Some(103)]);
        assert_eq!(result.stats.sieve_groups, 1);
        assert_eq!(result.stats.tail_fallbacks, 1);
    }
}
