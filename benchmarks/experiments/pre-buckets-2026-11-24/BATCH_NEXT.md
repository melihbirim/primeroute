# Batch next-prime engine

Exact next-prime queries over `u64`, preserving input order and duplicates. All code runs on one thread. The batch API chooses direct deterministic Miller–Rabin or a shared range sieve.

## Results

Measured on 2026-11-24, Apple M2 Pro, against primesieve 12.18 and primal 0.3.3. These are **milliseconds for the entire batch**, including sorting, deduplication, routing, computation and restoring input order. Seven measured runs follow one discarded warm-up. C++ is the faster median of the adaptive and continuous iterator baselines where both were practical.

| Workload | Queries | Our router | Direct control | primal | primesieve | Routes D/S |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| scattered_1e8 | 2,048 | 0.696 | 1.226 | 1.255 | 1.042 | 0/1 |
| clustered_1e8 | 8,192 | 0.338 | 4.404 | 4.484 | 0.546 | 0/1 |
| scattered_1e12 | 2,048 | 3.203 | 4.597 | 7.968 | 2.242 | 0/1 |
| clustered_1e12 | 8,192 | 1.324 | 15.234 | 26.566 | 1.343 | 0/1 |
| scattered_1e14 | 2,048 | 5.660 | 5.717 | 14.135 | 8.649 | 1/0 |
| clustered_1e14 | 8,192 | 4.532 | 18.474 | 44.718 | 4.971 | 0/1 |
| packed_1e14 | 8,192 | 1.512 | 1.551 | 3.712 | 2.025 | 1/0 |
| mixed_1e12 | 14,336 | 4.556 | 18.521 | 32.093 | 4.502 | 0/4 |
| spaced_1e14 | 1,024 | 2.890 | 2.876 | 6.951 | 28.065 | 1024/0 |
| isolated_1e14 | 1,024 | 2.870 | 2.853 | 7.063 | 1530.976 | 1024/0 |

The router has a lower median than primesieve in **8 of ten** workloads and
primal in **10 of ten**. The 10^12 clustered result is effectively a tie: the
roughly 1% median advantage is too small to claim a reliable win. Primesieve
retains a clear advantage on the 10^12 scattered band; the mixed batch is also
slightly faster with primesieve.

### Improvement over the previous version

Same seed and identical input/answer arrays. These are separate recorded runs
(five samples before, seven after), so small differences can include system
variation. The before-run snapshot is [batch-next-previous-results.json](batch-next-previous-results.json).

| Workload | Before | Now | Less time |
| --- | ---: | ---: | ---: |
| clustered_1e8 | 0.643 | 0.338 | 47.5% |
| clustered_1e12 | 1.520 | 1.324 | 12.9% |
| clustered_1e14 | 6.782 | 4.532 | 33.2% |
| mixed_1e12 | 6.454 | 4.556 | 29.4% |

### Second query seed

Seed **12648432**, seven measurements and one warm-up, gives different input
values. No answers or prepared tables persist between runs.

| Workload | Our router | primal | primesieve |
| --- | ---: | ---: | ---: |
| scattered_1e8 | 0.688 | 1.316 | 1.036 |
| clustered_1e8 | 0.336 | 4.403 | 0.537 |
| clustered_1e12 | 1.387 | 27.243 | 1.406 |
| clustered_1e14 | 4.689 | 46.702 | 5.166 |
| mixed_1e12 | 4.752 | 32.298 | 4.608 |

The low and high clusters retain their advantages; the middle cluster remains
close and the mixed batch is about 3% behind primesieve on this seed.
These measurements cover one ARM machine; they do not establish x86 speed.

[batch-next-results.json](batch-next-results.json) and
[batch-next-cluster-holdout-results.json](batch-next-cluster-holdout-results.json)
contain full inputs, expected answer arrays, all samples, router decisions, source
and executable hashes, build commands, versions and timestamps. Every answer agrees
across all measured implementations. Source and binary identities were frozen and
checked before/after each run; both reports match the final source files.

Intermediate experiments are archived under
[experiments/clustered-2026-11-24](experiments/clustered-2026-11-24/README.md).
The floating-point quotient and cache-blocked marking experiments were discarded.

## Workloads

All regular workloads start around 10^8, 10^12 or 10^14 plus 5,000,000. Inputs are
unique and shuffled, requiring original-order restoration:

- `scattered`: 2,048 values across a ten-million-number band.
- `clustered`: 8,192 values across a one-million-number band.
- `packed_1e14`: 8,192 values across only 16,384 numbers.
- `mixed_1e12`: three 4,096-query clusters, each 100,000 numbers wide, plus 2,048
  queries spread across another ten-million-number band.
- `spaced_1e14`: 1,024 queries approximately 100,003 apart, across about 102 million
  numbers. C++ can profitably retain one iterator across this whole span.
- `isolated_1e14`: 1,024 queries approximately 1,000,000,003 apart. C++ restarts
  instead of sieving billions of irrelevant numbers between queries.

Seven measured batches follow one discarded warm-up batch. Input parsing,
process startup and result serialization are excluded. Output vectors remain
available after timing; final answer-buffer destruction is excluded consistently.
Code/data pages can be cached, but no computed answers or prepared seed tables
persist between batches. No CPU affinity was set. All C++ runs explicitly set
and report one thread; the Rust implementations spawn no workers.

The initial router pilot used seed 20261124 and early validation used 12648430.
The final comparison reuses seed **12648431** to compare the optimized engine on
identical inputs, with **12648432** as a second query set. Earlier model results
remain in `batch-next-calibration.json` and `batch-next-validation.json`;
`batch-next-previous-results.json` records the version before clustered-query
optimization. They are historical snapshots with their own source hashes.

## Fair baselines

**Primal:** its published `is_prime` API is wrapped in the exact same candidate
wheel and small-prime filter source as ours. It also sorts, deduplicates, reuses
an answer while it remains greater than the next input, and restores input order.
Both Rust engines also share the same narrow-batch radix sorter; comparison
sorting remains the fallback. This compares deterministic primality implementations
with the same candidate filter and batching rather than unfiltered tests or a
sieve from zero to a huge bound.

**Primesieve:** sorted inputs share a continuing `primesieve::iterator`, with a
stop hint beyond the last query. The adaptive baseline restarts only when a gap
exceeds `max(1,000,000, 4*sqrt(cluster_start))`. It has no materialized-span cap.
A second baseline retains one iterator across the entire batch whenever its span
is at most 200 million; the table uses the faster median. Streaming across the
roughly trillion-number isolated span was excluded as impractical. C++ therefore
gets its batching advantage rather than being forced to initialize once per
query in ordinary clusters.

The `ours_sieve` control requests sieving but still honors all policy limits,
including the minimum of two unique queries per group. It can fall back to direct
search; consult its recorded route stats. It should not be interpreted as an
unconditionally forced, potentially unbounded sieve benchmark.

## Implementation

`is_prime` is deterministic for every `u64`. Above the small-number fast path it
uses witnesses `2, 325, 9375, 28178, 450775, 9780504, 1795265022`, from the
[deterministic Miller–Rabin witness records](https://miller-rabin.appspot.com/).
The documented three-base set `2, 7, 61` handles smaller numbers. Products for
32-bit candidates fit in `u64`; larger candidates use Montgomery arithmetic,
including the extra carry bit needed for moduli near `u64::MAX`.

`next_prime` skips multiples of 2, 3 and 5, uses a precomputed 7/11/13 filter,
then trial-divides by primes 17 through 61. Survivors get the exact test. There
is no redundant trial division by the primes already covered by the table.
Checked candidate increments return `None` when no larger u64 prime exists.

The batch engine sorts and deduplicates inputs, then forms groups using a
65,536 maximum adjacent gap and a **16-million-number** maximum group span. It
estimates direct searches, allowing answer reuse within a prime gap, against
seed preparation and sieve traversal. Sieving must predict at least a 20% time
saving (`1.25 * sieve_cost < direct_cost`). Sieve tails get 512 extra numbers as
an optimization; exact direct fallback handles any longer prime gap.

The dense path answers directly from each mod-30 bitmap: it finds the first set
bit after the current query and reuses that answer for subsequent inputs in the
same prime gap. It never constructs a full interval prime list. The seed table
is generated once for all sieve groups within a batch and discarded afterwards.

Sieving primes use `u32` storage, with a specialized 32-bit NEON decoder on
AArch64 and a portable scalar fallback. Dense stream offsets also use `u32`;
the fixed 128 KiB segment bounds their initial offsets below 2^32. This reduces
each dense state from 80 to 48 bytes on this 64-bit machine. Single-segment
ranges initialize and mark large sparse primes directly, without retaining
their state vector, and overlap independent initial divisions in groups of eight.
All quotient arithmetic uses integer division.

The 17 compact static pre-sieve patterns are applied cyclically. No per-query
expanded pattern buffers are allocated (previously up to about 2.4 MB). Narrow
batches of at least 4,096 inputs use byte-wise radix sorting when their total
span fits in `u32`; other batches retain comparison sorting. This optimization
is shared with the primal benchmark wrapper.

Default limits are a seed root of 10,000,000 and a conservative **192 MiB** sieve
working-set estimate. The estimate covers seed/state storage, the bounded bitmap
and recursive seed workspace; it no longer charges for an interval prime list.
These limits prevent routing huge isolated values into multi-GB seed tables. The estimate is a planning bound, not an allocator-enforced RSS cap;
input/order/output storage still grows with the number of queries. The cost
coefficients were tuned on the measured M2 and can make suboptimal decisions on
other workloads or CPUs. `BatchOptions` exposes strategy and resource limits.

## API and CLI

```rust
use prime_numbers::{batch_next_primes, is_prime, next_prime};

assert!(is_prime(101));
assert_eq!(next_prime(101), Some(103)); // strictly greater
assert_eq!(batch_next_primes(&[100, 101, 100]), vec![Some(101), Some(103), Some(101)]);
assert_eq!(next_prime(u64::MAX), None);
```

`batch_next_primes_with_options` returns both answers and `BatchStats`, including
route counts, deduplicated input counts, estimated sieve memory and tail fallbacks.
`BatchStrategy::Direct` disables sieving; `BatchStrategy::Sieve` requests it within
policy limits. Default `Auto` uses the cost model.

```sh
cargo run --release -- next 100 101 100000005000000
printf '100 101 100\n' | cargo run --release -- next
```

The CLI prints one answer per input in the original order. `none` means no larger
prime is representable. Existing range/count/printing commands remain available.

## Validation

All 20 tests pass in debug and release. Formatting and Clippy with warnings
denied pass for the main crate and the benchmark crate. Tests include:

- Montgomery multiplication checked against u128 remainders, including high
  moduli and carry-overflow cases.
- Primality and strict next-prime results checked against a sieve for 100,000
  small values, plus Carmichael numbers, strong pseudoprimes and u64 endpoints.
- Duplicate/order restoration, empty batches, both router paths, memory-policy
  fallback, and sieve-tail fallback with zero padding.
- Bitmap next-prime lookups across segment boundaries, small primes, duplicates,
  high starts, and zero-padding tails, compared with exact direct search.
- Radix ordering against tuple sorting, including duplicates and u64 endpoints.
- Compact dense offsets checked with u128 arithmetic; both 32-bit and 64-bit
  decoders checked for all 65,536 masks, growth, tails, and u32 endpoint padding.
- The previous packed-sieve, wheel and range tests.

The independent primal verification executable checks 200,000 small values,
100,000 random u64 values, 10,000 random u32 values, and 2,500 high next-prime
queries, including the area near u64::MAX. All agree. Full answer vectors are
also compared with primal and C++ during every benchmark batch.

## Reproduce

Build static primesieve as described in [README.md](README.md), then populate the
Rust dependency cache and run:

```sh
cargo build --release --manifest-path benchmarks/primal/Cargo.toml --bin batch_next
benchmarks/primal/target/release/batch_next verify
python3 benchmarks/batch_next.py --primesieve-dir /path/to/primesieve \
  --seed 12648431 --runs 7 --warmups 1 --include-sieve
```

The driver rebuilds both helpers, using locked/offline Cargo dependencies and
Clang -O3, freezes source/executable identities, and checks that they remain
unchanged. The final JSON is published atomically only after all cases pass;
a separate `.progress.json` is explicitly marked as running. On failure the
previous completed report remains intact. The script supports dataset filtering,
separate output paths, run counts and warm-ups via `--help`.
