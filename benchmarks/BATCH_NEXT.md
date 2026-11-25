# Batch next-prime engine

Exact next-prime queries over `u64`, preserving input order and duplicates. All code runs on one thread. The batch API chooses direct deterministic Miller–Rabin or a shared range sieve.

## Results

Measured on **2026-11-25, Apple M2 Pro**, against primesieve 12.18 and primal
0.3.3, using the frozen short-range-optimized source. Times are milliseconds for
the whole batch, including routing and order restoration. Seven measurements
follow one warm-up. C++ is the faster adaptive/continuous iterator median.

| Workload | Queries | Our router | Direct control | primal | primesieve | Routes D/S |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.674 | 1.224 | 1.284 | 1.041 | 0/1 |
| clustered_1e8 | 8,192 | 0.327 | 4.398 | 4.389 | 0.542 | 0/1 |
| scattered_1e12 | 2,048 | 2.827 | 4.650 | 7.957 | 2.257 | 0/1 |
| clustered_1e12 | 8,192 | 1.270 | 15.395 | 26.490 | 1.384 | 0/1 |
| scattered_1e14 | 2,048 | 5.709 | 5.743 | 13.881 | 8.626 | 1/0 |
| clustered_1e14 | 8,192 | 4.185 | 18.491 | 44.990 | 4.979 | 0/1 |
| packed_1e14 | 8,192 | 1.583 | 1.546 | 3.615 | 2.088 | 1/0 |
| mixed_1e12 | 14,336 | 4.222 | 18.474 | 31.927 | 4.493 | 0/4 |
| spaced_1e14 | 1,024 | 2.889 | 2.850 | 6.931 | 27.931 | 1024/0 |
| isolated_1e14 | 1,024 | 2.908 | 2.901 | 6.985 | 1530.674 | 1024/0 |

The router has lower medians than C++ in **9/10 workloads** and primal
in **10/10**. Close medians should be read with their sample ranges; these
are measurements on one ARM machine, not a guarantee for every input or CPU.

### Previous kernel

The previous frozen source and reports are retained in
[the archive](experiments/short-range-2026-11-25/README.md). Query and expected
answer arrays are identical. These batch comparisons are separate runs, so small
differences include system variation. The range suite remeasures the old range
executable in the same rounds.

| Workload | Previous Rust | Current Rust |
| --- | ---: | ---: |
| clustered_1e8 | 0.302 | 0.327 |
| clustered_1e12 | 1.252 | 1.270 |
| clustered_1e14 | 4.281 | 4.185 |
| mixed_1e12 | 4.207 | 4.222 |

### Second query seed

Seed 12648432 uses different query values, seven measurements and one warm-up.

| Workload | Our router | primal | primesieve |
| --- | ---: | ---: | ---: |
| scattered_1e8 | 0.692 | 1.278 | 1.049 |
| clustered_1e8 | 0.329 | 4.354 | 0.544 |
| clustered_1e12 | 1.272 | 26.637 | 1.383 |
| clustered_1e14 | 4.320 | 45.129 | 4.986 |
| mixed_1e12 | 4.125 | 31.782 | 4.493 |

All full answer arrays agree across Rust, C++ and primal.
[batch-next-results.json](batch-next-results.json) and
[batch-next-cluster-holdout-results.json](batch-next-cluster-holdout-results.json)
contain all inputs, answers, samples, route stats and source/binary hashes.
The [manifest](measurement-2026-11-25.json) records the frozen source snapshot
and commands. [RANGES.md](RANGES.md) measures the shared range backend separately.

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
Code/data pages can be cached. Computed answers and per-call seed/marking
states are discarded; fixed compiler-generated seed metadata is shared read-only. No CPU affinity was set. All C++ runs explicitly set
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

The dense path answers directly from each mod-30 bitmap and shares one `u32`
seed table across sieve groups within the call. No full interval prime list is
materialized. Per-call prepared data is discarded; the fixed compiler-generated
seed metadata remains read-only.

Dense offsets and their bounded prime use `u32`. A tracked first-stream index
keeps the marking offsets in cyclic order, giving the maximum via its predecessor
and walking only the remaining tail hits. Dense state remains 48 bytes on the
64-bit target. The hot loop is outlined to keep its register use separate from
range setup. Single-segment requests still avoid a retained sparse-state vector.

Medium-prime masks, gaps and carry bytes rotate in registers, avoiding a dependent
transition-table load per hit. Big primes above 1,048,576 retain the bucket ring.
Standalone range calls use static seed primes through 2^20 and stream larger
roots. Fixed seeds, reciprocals and dense templates occupy about 1.99 MiB.
Batch groups retain one per-call prepared table. Single-window marking uses
rotated sorted offsets and discards final state updates; details are in [RANGES.md](RANGES.md).

Single-window requests fuse the compact cyclic masks in one pass. Other short
requests apply them cyclically. Requests spanning at
least eight segments prepare 17 expanded mask buffers once, about 2.4 MB per
prepared set, amortizing setup across segments. The memory estimate includes
workspace allowance; it is not an allocator-enforced RSS cap. Narrow batches
of at least 4,096 inputs can use radix sorting, also shared with the primal
benchmark wrapper. Quotient arithmetic remains integer arithmetic.

Default limits are a seed root of 10,000,000 and a conservative **192 MiB** sieve
working-set estimate. The estimate covers seed/state storage, the bounded bitmap
and recursive seed workspace; it no longer charges for an interval prime list.
These limits prevent routing huge isolated values into multi-GB seed tables.
The estimate is a planning bound, not an allocator-enforced RSS cap;
input/order/output storage still grows with the number of queries. The cost
coefficients were tuned on the measured M2 and can make suboptimal decisions on
other workloads or CPUs. `BatchOptions` exposes strategy and resource limits.

## API and CLI

```rust
use primeroute::{batch_next_primes, is_prime, next_prime};

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

All 28 tests pass in debug and release. Formatting and Clippy with warnings
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
- Bucket-ring scheduling and deferred prime-square activation compared with
  single-window range chunks near 1.2e12 and 1e15.
- Ordered tail state and bitmaps compared with independent marking streams.
- Packed medium steps compared with the original transition table, including
  large-prime bounds, and expanded masks compared with cyclic masks.
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
