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
Rust **primal 0.3.3**. Values are median **milliseconds for the entire batch**:
sorting, deduplication, routing, computation, and restoring input order are
included. Parsing, process startup, and result serialization are excluded.
Seven measured runs follow one discarded warm-up.

| Workload | Queries | Ours | primesieve | primal |
| --- | ---: | ---: | ---: | ---: |
| Scattered near 10⁸ | 2,048 | 0.702 | 1.033 | 1.262 |
| Clustered near 10⁸ | 8,192 | 0.344 | 0.546 | 4.466 |
| Scattered near 10¹² | 2,048 | 3.114 | 2.132 | 7.802 |
| Clustered near 10¹² | 8,192 | 1.348 | 1.383 | 27.167 |
| Scattered near 10¹⁴ | 2,048 | 5.801 | 8.874 | 14.204 |
| Clustered near 10¹⁴ | 8,192 | 4.587 | 5.123 | 45.415 |
| Packed near 10¹⁴ | 8,192 | 1.629 | 2.110 | 3.721 |
| Mixed near 10¹² | 14,336 | 4.663 | 4.651 | 32.032 |
| Spaced near 10¹⁴ | 1,024 | 2.837 | 27.998 | 7.022 |
| Isolated near 10¹⁴ | 1,024 | 2.929 | 1544.077 | 7.101 |

Scattered inputs cover ten million numbers; clustered inputs cover one million;
packed inputs cover 16,384. Spaced and isolated inputs are approximately 100,003
and 1,000,000,003 apart, respectively. The mixed batch combines three narrow
clusters with a scattered band. Inputs are shuffled and starting values differ
between queries.

Ours has a lower median in **8/10 workloads against primesieve** and **10/10
against primal**. The clustered result near 10¹² is effectively a tie.
Primesieve remains faster on the 10¹² scattered band and slightly faster on the
mixed batch. A second query seed confirms the low/high clustered advantages and
the close middle result. These measurements cover one ARM machine, with no CPU
affinity set; they do not establish performance on x86 or other workloads.

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

The [prefix-counting report](benchmarks/README.md) and
[range report](benchmarks/RANGES.md) also measure the current source, including
counts through 100 billion and narrow/wide ranges near 10¹⁸. Their timing methods
differ from the batch benchmarks. Earlier reports remain in
[the pre-bucket archive](benchmarks/experiments/pre-buckets-2026-11-24/README.md).

For the new 40-million-number range near 10¹⁸, counting takes **285.691 ms**
versus primesieve's **311.003 ms**. Generating the list takes **288.407 ms**
versus **277.009 ms**, so the counting lead does not extend to list generation.
A separate count-process memory check measured about **31.5 MiB peak RSS** for
both implementations; details and the narrow-range results are in the range report.

## How it works

Direct search skips multiples of 2, 3, and 5, filters additional small-prime
multiples, then runs deterministic Miller–Rabin. The witness sets cover every
`u64`; larger candidates use Montgomery modular arithmetic. Checked increments
return `None` when no larger representable prime exists.

The sieve uses a mod-30 bitmap and segments of up to 128 KiB. Compact static
patterns remove multiples of primes 7 through 167. Dense primes use eight marking
streams; medium primes use a flat wheel-state list, and primes above 1,048,576
are scheduled in buckets for the next segment they hit. Single-segment requests
mark large primes directly without retaining their state vector.

Batch sieve groups share one `u32` seed table per call and read requested answers
directly from the bitmap, without generating a full interval prime list. Dense
stream offsets also use `u32`. Narrow batches can use radix sorting. Counting
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

All **21 tests** pass in debug and release on the measured ARM machine. Coverage
includes independent sieve comparisons, exact next-prime comparisons across
segment boundaries, pseudoprimes and `u64` endpoints, Montgomery arithmetic,
ordering/duplicates, router limits, and tail fallback. Both decoder widths are
checked against all 65,536 two-byte masks, including buffer growth and tails.
Formatting and Clippy pass. The benchmark helper also cross-checks primality and
next-prime results against primal; details are in the
[batch report](benchmarks/BATCH_NEXT.md#validation).
