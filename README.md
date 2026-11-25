# primeroute

A single-threaded Rust library and CLI for counting primes, listing primes in
ranges, and finding the next prime after each number in a batch.

The aim is to compete with primesieve on one thread. Results are checked against
other implementations. Benchmark reports retain the samples, source hashes and
binary hashes.

The library has no external dependencies. C++ primesieve and primal are used
for benchmarks only.

## Run it

Rust edition 2024. The recorded builds use rustc 1.94.1.

```sh
cargo build --release --locked

# Count primes <= 100. Output: 25.
./target/release/primeroute 100

# Count primes in [1000, 1100].
./target/release/primeroute 1000 1100

# Print the primes in that interval.
./target/release/primeroute 1000 1100 --print

# Next prime after each input. Output: 101, 103, 101.
./target/release/primeroute next 100 101 100

# Read a batch from stdin.
printf '100 101 100\n' | ./target/release/primeroute next
```

Range endpoints are inclusive. Next-prime answers are strictly greater than the
input. Order and duplicates are preserved. The CLI prints `none` when no larger
prime fits in `u64`.

Place `--print` last. Results go to stdout. Timings and errors go to stderr.
Invalid inputs and inverted ranges exit with code 2. Use `--help` for syntax.

## Use it from Rust

Add a local path dependency:

```toml
[dependencies]
primeroute = { path = "../prime-numbers" }
```

```rust
use primeroute::{
    batch_next_primes, count_primes, count_primes_in_range,
    is_prime, next_prime, primes_in_range,
};

assert!(is_prime(101));
assert_eq!(next_prime(101), Some(103));
assert_eq!(next_prime(u64::MAX), None);

assert_eq!(
    batch_next_primes(&[100, 101, 100]),
    vec![Some(101), Some(103), Some(101)],
);

assert_eq!(count_primes(100), 25);
assert_eq!(count_primes_in_range(100, 110), 4);
assert_eq!(primes_in_range(100, 110), vec![101, 103, 107, 109]);
```

Primality and next-prime APIs accept `u64`. Range APIs use `usize` for bounds,
counts and output values. Inverted library ranges return an empty result.

Use `for_each_prime` or `for_each_prime_in_range` to stream results through a
callback. Use `primes_in_range` when you need the full ordered vector.

`batch_next_primes_with_options` exposes `Auto`, `Direct` and `Sieve` strategies.
It returns answers and route statistics. `Sieve` still respects resource limits
and can fall back to direct testing. The [batch report](benchmarks/BATCH_NEXT.md)
covers the options and routing policy.

## Measured results

**25 November 2026. Apple M2 Pro. One thread.** Compared with C++ primesieve
12.18 and Rust primal 0.3.3. All tables show medians.

### Short ranges

Each interval contains one million numbers. Times are milliseconds per fresh
query. Setup, allocation, list checksums and vector destruction are included.
There are 30 measured samples after five warm-ups, with three fresh queries per
sample. C++ gets the best median of its default, 64, 256 and 1024 KiB settings.

| Start | Rust count | C++ count | Rust list | C++ list | Primal adapter list |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 0.068 | 0.092 | 0.110 | 0.117 | 0.925 |
| 100,000,000,000 | 0.255 | 0.310 | 0.282 | 0.335 | 1.398 |
| 1,000,000,000,000 | 0.670 | 0.870 | 0.700 | 0.882 | 2.226 |

All six count/list cases above beat primesieve. The additional short suite wins
**24/24 cases** across widths from 1,000 to one million. An unaligned holdout,
including intervals around four million numbers wide, wins **19/24**.

Primal has no public range-sieve API. The adapter independently segments the
interval using primal's public seed sieve. Its setup is included. The public
`primal::is_prime` scanning path is also benchmarked separately.

The [range report](benchmarks/RANGES.md) includes before/after results, high
starts, memory measurements and every remaining loss.

### Counting through 100 billion

These timings include process startup, setup, computation and output capture.
Through ten billion: five warm-ups and 30 measurements. At 100 billion: one
warm-up and five measurements. C++ gets the faster default/tuned median.

| Limit | Ours | primesieve | primal |
| --- | ---: | ---: | ---: |
| 100,000,000 | 8.212 ms | 8.070 ms | 12.895 ms |
| 1,000,000,000 | 52.668 ms | 65.050 ms | 132.417 ms |
| 10,000,000,000 | 637.159 ms | 763.521 ms | 1759.883 ms |
| 100,000,000,000 | 9.132 s | 10.061 s | 24.757 s |

At 100 billion, Rust takes **9.2% less time than primesieve**. Every Rust sample
beats every C++ sample in that run. All runs return **4,118,054,813 primes**.
The previous Rust executable takes 9.247 s in the same interleaved comparison.

The 100-million process benchmark still favors C++ slightly. See the
[counting report](benchmarks/README.md) and
[raw 100-billion results](benchmarks/results-100b.json).

### Batch next-prime queries

Times are milliseconds for the entire batch. Sorting, deduplication, routing,
computation and restoring input order are included. Parsing, process startup
and serialization are excluded. Seven measurements follow one warm-up.

| Workload | Queries | Ours | primesieve | primal |
| --- | ---: | ---: | ---: | ---: |
| Scattered near 10⁸ | 2,048 | 0.674 | 1.041 | 1.284 |
| Clustered near 10⁸ | 8,192 | 0.327 | 0.542 | 4.389 |
| Scattered near 10¹² | 2,048 | 2.827 | 2.257 | 7.957 |
| Clustered near 10¹² | 8,192 | 1.270 | 1.384 | 26.490 |
| Scattered near 10¹⁴ | 2,048 | 5.709 | 8.626 | 13.881 |
| Clustered near 10¹⁴ | 8,192 | 4.185 | 4.979 | 44.990 |
| Packed near 10¹⁴ | 8,192 | 1.583 | 2.088 | 3.615 |
| Mixed near 10¹² | 14,336 | 4.222 | 4.493 | 31.927 |
| Spaced near 10¹⁴ | 1,024 | 2.889 | 27.931 | 6.931 |
| Isolated near 10¹⁴ | 1,024 | 2.908 | 1530.674 | 6.985 |

Rust wins **9/10 workloads against primesieve** and **10/10 against primal**.
Primesieve still leads on the scattered band near 10¹².

C++ shares continuing iterators across queries. Its faster adaptive/continuous
median is shown where practical. Primal uses its public exact primality API with
the same candidate filter, sorting and answer reuse as our direct path.
Every full answer array agrees.

The [batch report](benchmarks/BATCH_NEXT.md) defines the workloads and includes
a second query seed, direct/sieve controls and reproduction commands.

These results cover one ARM machine. Wider ranges and some high-start list
queries still favor primesieve. Other CPUs need their own measurements.

## How it works

Sparse next-prime queries test candidates directly. Dense query groups share a
segmented sieve. The router estimates the cost of each approach per group.

**Direct testing.** Skip multiples of 2, 3 and 5. Filter more small factors.
Run deterministic Miller–Rabin on the survivors. The test covers every `u64`.
Larger candidates use Montgomery arithmetic. Checked increments handle the
end of the integer range.

**Range sieving.** Store eight candidates per byte using a mod-30 bitmap.
Process windows of up to 128 KiB. Static patterns remove factors 7 through 167.
Dense primes use eight marking streams. Medium primes use rotating wheel steps.
Large primes wait in buckets until the next window they hit.

**Short requests.** Fuse the pre-sieve masks into one pass. Align dense streams
using fixed seed metadata and exact integer reciprocal corrections. On Apple
Silicon, NEON table lookups rotate the streams into increasing order. Final-window
marking discards state updates that no later window will use.

**Long requests.** Prepare expanded masks once when the request spans at least
eight windows. The prepared set is about 2.4 MB. Reuse it across the request.
Counting uses NEON population counts where supported. List and seed decoding
also use NEON. Portable fallbacks remain.

Batch sieve groups read answers directly from the bitmap and share one seed
table per call. They do not materialize the full interval prime list.

## Tradeoffs and limits

Fixed seed and alignment metadata occupies **1.99 MiB** before linker alignment,
on top of the existing mask and decoder tables. It moves bounded preprocessing
to compilation and increases binary size. Every requested interval is still
marked and counted. Query answers are not cached.

Standalone ranges use fixed seeds for roots through 2²⁰ and stream larger roots.
Very high starts still pay seed-generation and alignment costs. Standalone range
APIs have no configurable memory cap.

Default batch limits:

| Setting | Default |
| --- | ---: |
| Maximum adjacent query gap | 65,536 |
| Maximum group span | 16 million numbers |
| Maximum seed root | 10 million |
| Estimated sieve working set | 192 MiB |
| Extra sieve tail | 512 numbers |

The memory limit is an estimate. Input, sorting and output storage also grow
with batch size. It is not an allocator-enforced RSS cap.

Direct fallback handles prime gaps beyond the extra sieve tail. Correctness
does not depend on the padding. The router's cost model is tuned on the M2 Pro
and can choose a slower route on other workloads. `BatchOptions` exposes the
strategy and limits.

## Validation and reproduction

```sh
cargo test --locked
cargo test --release --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

All **28 tests** pass in debug and release on the measured ARM machine.
Formatting and Clippy pass for the main crate and benchmark crate.

Coverage includes independent sieve comparisons, known counts, deterministic
primality checks, prime-gap and integer endpoints, batch ordering and limits,
bucket scheduling, static seeds, exact reciprocal quotients and NEON stream
rotation. Both decoder widths are checked against all 65,536 two-byte masks.
Unsafe marking and decoding paths have documented bounds and targeted tests.

Range benchmarks compare counts and wrapping-u64 checksums. Batch benchmarks
compare full answer arrays. A separate helper cross-checks primality and
next-prime results against primal.

The code matches the completed [benchmark manifest](benchmarks/measurement-2026-11-25.json).
It records source and binary hashes, commands and the source snapshot. Previous
executables, reports and experiments are retained in the
[archive](benchmarks/experiments/short-range-2026-11-25/README.md).

Reproduce the [counting](benchmarks/README.md#reproduce),
[range](benchmarks/RANGES.md#validation-and-reproduction) and
[batch](benchmarks/BATCH_NEXT.md#reproduce) comparisons using their report commands.
