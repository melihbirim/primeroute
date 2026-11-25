# prime-numbers

A dependency-free Rust library and CLI for exact `u64` primality testing,
finding the next prime after each number in a batch, and counting or generating
primes in inclusive ranges. All computation runs on one thread.

Batch queries use deterministic Miller–Rabin for sparse inputs and a shared
segmented sieve for dense inputs. The router sorts and deduplicates queries,
reuses answers within prime gaps, and restores the original order and duplicates.

## Build and run

The project uses Rust edition 2024. The recorded benchmarks used rustc 1.94.1.
C++ and primal are needed only for comparative benchmarks.

```sh
cargo build --release --locked

# Strict next prime for each input; prints 101, 103, 101 on separate lines.
./target/release/prime-numbers next 100 101 100

# With no number arguments, read whitespace-separated inputs from stdin.
printf '100 101 100\n' | ./target/release/prime-numbers next

# Count primes <= 100; prints 25.
./target/release/prime-numbers 100

# Count or print primes in the inclusive range [1000, 1100].
./target/release/prime-numbers 1000 1100
./target/release/prime-numbers 1000 1100 --print
```

CLI forms:

```text
prime-numbers next [numbers ...]
prime-numbers [start] <limit> [--print]
prime-numbers --help
```

Next-prime answers are **strictly greater** than their inputs. The CLI prints
`none` when no larger prime fits in `u64`. Range endpoints are inclusive; one
bound means a range starting at zero. Place `--print` after the numeric bounds.
Counts, prime lists, or next-prime answers go to stdout; timing summaries go to
stderr. The CLI rejects inverted ranges and invalid inputs with exit code 2.

## Library usage

To use this library from another local crate, add a path dependency, adjusting
the path to this folder:

```toml
[dependencies]
prime-numbers = { path = "../prime-numbers" }
```

```rust
use prime_numbers::{
    batch_next_primes, batch_next_primes_with_options, count_primes,
    is_prime, next_prime, primes_in_range, BatchOptions, BatchStrategy,
};

assert!(is_prime(101));
assert_eq!(next_prime(101), Some(103));
assert_eq!(next_prime(u64::MAX), None);
assert_eq!(
    batch_next_primes(&[100, 101, 100]),
    vec![Some(101), Some(103), Some(101)],
);

let batch = batch_next_primes_with_options(
    &[100, 101, 100],
    BatchOptions {
        strategy: BatchStrategy::Direct,
        ..BatchOptions::default()
    },
);
assert_eq!(batch.answers, vec![Some(101), Some(103), Some(101)]);
assert_eq!(batch.stats.unique_queries, 2);

assert_eq!(count_primes(100), 25);
assert_eq!(primes_in_range(100, 110), vec![101, 103, 107, 109]);
```

`is_prime`, `next_prime`, and both batch functions accept `u64` values.
Range APIs use `usize` for bounds, counts, and prime values.
`count_primes_in_range` counts an interval; `for_each_prime` and
`for_each_prime_in_range` stream primes through a callback without collecting
the full list. `primes_in_range` returns an ordered vector. Library range APIs
treat inverted intervals as empty.

`BatchOptions` selects `Auto` (the default), `Direct`, or `Sieve` and exposes
grouping and resource limits. `Sieve` requests sieving within those limits; it
can still fall back to direct search. `BatchStats` reports the chosen routes,
unique-query count, estimated sieve memory, and tail fallbacks.

## Current single-threaded benchmarks

Measured on **2026-11-25, Apple M2 Pro**, against C++ **primesieve 12.18** and
Rust **primal 0.3.3**.

### Batch next-prime

Values are median **milliseconds for the entire batch**:
sorting, deduplication, routing, computation, and restoring input order are
included. Parsing, process startup, and result serialization are excluded.
Seven measured runs follow one discarded warm-up.

| Workload | Queries | Ours | primesieve | primal |
| --- | ---: | ---: | ---: | ---: |
| Scattered near 10⁸ | 2,048 | 0.631 | 1.018 | 1.212 |
| Clustered near 10⁸ | 8,192 | 0.302 | 0.510 | 4.216 |
| Scattered near 10¹² | 2,048 | 2.613 | 2.167 | 7.628 |
| Clustered near 10¹² | 8,192 | 1.252 | 1.358 | 25.940 |
| Scattered near 10¹⁴ | 2,048 | 5.588 | 8.500 | 13.529 |
| Clustered near 10¹⁴ | 8,192 | 4.281 | 4.885 | 43.864 |
| Packed near 10¹⁴ | 8,192 | 1.534 | 2.071 | 3.610 |
| Mixed near 10¹² | 14,336 | 4.207 | 4.452 | 30.919 |
| Spaced near 10¹⁴ | 1,024 | 2.758 | 27.510 | 6.842 |
| Isolated near 10¹⁴ | 1,024 | 2.811 | 1522.719 | 6.931 |

Scattered inputs cover ten million numbers; clustered inputs cover one million;
packed inputs cover 16,384. Spaced and isolated inputs are approximately 100,003
and 1,000,000,003 apart, respectively. The mixed batch combines three narrow
clusters with a scattered band. Inputs are shuffled and starting values differ
between queries.

Ours records lower medians in **9/10 workloads against primesieve** and
**10/10 against primal**. Primesieve still leads on the scattered 10¹² band.
A second query seed also measures the clustered and mixed gains. These results
cover one ARM machine with no CPU affinity set; they do not establish a win
for every input, CPU, or workload.

C++ explicitly uses one thread and shares continuing iterators across queries.
The table takes the faster median of its adaptive and continuous iterator
baselines where both were practical. Primal uses its exact `is_prime` API with
the same candidate filter, sorting, and answer reuse as our direct path. Both
Rust implementations spawn no workers. No computed answers or prepared seed
tables persist between batches. Every measured answer agrees across all three.

See [the full batch report](benchmarks/BATCH_NEXT.md) for workload definitions,
direct/sieve controls, before-and-after results, the second query seed, and
reproduction commands. [Raw results](benchmarks/batch-next-results.json) include
all inputs, expected answers, samples, build commands, and source/binary hashes.

### Counting primes up to N

These are whole-process median times, including startup and output capture.
There are five warm-ups and 30 measured runs through ten billion; at 100 billion,
one warm-up and five measured runs. C++ is the faster default/tuned median.

| Limit | Ours | primesieve | primal |
| ---: | ---: | ---: | ---: |
| 100,000,000 | 6.908 ms | 6.891 ms | 11.686 ms |
| 1,000,000,000 | 51.909 ms | 64.336 ms | 130.941 ms |
| 10,000,000,000 | 626.010 ms | 755.001 ms | 1739.826 ms |
| 100,000,000,000 | 8.959 s | 9.730 s | 23.859 s |

At 100 billion, ours uses **7.9% less time than primesieve** and
**20.8% less than the previous Rust executable**, measured in the
same interleaved run. Every new-Rust sample is faster than every C++ sample
in that run. At 100 million the implementations are effectively tied, with
C++ having a slightly lower median; this is not an all-case win.

See the [full counting report](benchmarks/README.md) and
[raw 100-billion samples](benchmarks/results-100b.json) for the complete results.
The [range report](benchmarks/RANGES.md) also measures the current source,
including narrow/wide ranges near 10¹⁸. Earlier results are preserved in
[the pre-optimization archive](benchmarks/experiments/pre-count-optimization-2026-11-25/README.md).

For a 40-million-number range near 10¹⁸, counting takes **280.255 ms**
versus primesieve's **309.333 ms**. List generation takes **280.225 ms**
versus **273.209 ms**, so the counting lead does not extend to list generation.
A separate count-process RSS observation measured **44.05 MiB** for Rust
and **42.77 MiB** for C++ default; these are single observations.

## How it works

Direct search skips multiples of 2, 3, and 5, filters additional small-prime
multiples, then runs deterministic Miller–Rabin. The witness sets cover every
`u64`; larger candidates use Montgomery modular arithmetic. Checked increments
return `None` when no larger representable prime exists.

The sieve uses a mod-30 bitmap and segments of up to 128 KiB. Compact static
patterns remove multiples of primes 7 through 167. Requests spanning at least
eight segments prepare expanded masks once, amortizing up to about 2.4 MB per
prepared set; shorter requests cycle directly through the compact patterns.
Dense primes use eight marking streams, tracking their cyclic order for the
remaining tail hits; medium primes use a flat wheel-state list, and primes above 1,048,576
are scheduled in buckets for the next segment they hit. Single-segment requests
mark large primes directly without retaining their state vector.

Batch sieve groups share one `u32` seed table per call and read requested answers
directly from the bitmap, without generating a full interval prime list. Dense
stream offsets and bounded primes use `u32`, keeping each state at 48 bytes on
64-bit targets despite the additional order index. Narrow batches can use radix
sorting. Counting
uses NEON population counts on supported AArch64 CPUs, and vector/seed generation
uses NEON decoding, with portable scalar fallbacks. The marking loop and NEON
decoders contain documented unsafe code.

## Limits and tradeoffs

- Default batch grouping allows a maximum adjacent gap of **65,536** and a
  maximum span of **16 million** numbers per group.
- By default, batch sieving is limited to a seed root of **10 million** and a
  conservative **192 MiB** working-set estimate. This is not an allocator-enforced RSS cap;
  query, sorting, and answer storage additionally grow with input size.
- Sieve groups extend 512 numbers beyond the last query. Exact direct fallback
  handles longer prime gaps, so correctness does not depend on that padding.
- The router uses a cost heuristic tuned on the M2 Pro; it can choose a slower
  route on other inputs or CPUs.
- The standalone range APIs do not apply the batch router's resource limits.
  They stream sieving primes through `sqrt(limit)`, so very high starts still
  pay that setup time even for narrow ranges. Primes above about one million
  wait in per-window buckets, and primes with no multiple in the range are
  dropped, so memory follows the primes that actually mark the range.

## Validation

```sh
cargo test --locked
cargo test --release --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

All **24 tests** pass in debug and release on the measured ARM machine. Coverage
includes independent sieve comparisons, exact next-prime comparisons across
segment boundaries, pseudoprimes and `u64` endpoints, Montgomery arithmetic,
ordering/duplicates, router limits, and tail fallback. The new kernels are
checked against independent marking streams and the wheel transition table;
expanded masks are compared with cyclic masks at offsets and tails. Both decoder
widths are
checked against all 65,536 two-byte masks, including buffer growth and tails.
Formatting and Clippy pass. The benchmark helper also cross-checks primality and
next-prime results against primal; details are in the
[batch report](benchmarks/BATCH_NEXT.md#validation).
