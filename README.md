# primeroute

Version **0.1.1**. See the [release notes](CHANGELOG.md).

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

These are the original frozen M2 measurements. The current code has separate
[100-billion platform results](benchmarks/X86_COUNT_100B.md). The earlier
[platform report](benchmarks/X86.md) remains available with its original data.

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

### Current platform checks — 26 November

At **100 billion**, with one thread and complete interval sieving:

| CPU | Previous Rust | Current Rust | primesieve | primal |
| --- | ---: | ---: | ---: | ---: |
| AMD EPYC 9V74 VM | 13.718 s | 10.754 s | 10.695 s | 24.959 s |
| Apple M2 Pro | 8.982 s | 8.992 s | 9.730 s | 23.924 s |

Rust counting improves **21.6%** on x86. It remains **0.54% slower than
primesieve**, within the shared 3% tie band. We have not beaten primesieve
at 100 billion on x86. C++ gets the fastest measured median of default, 256 KiB and
1024 KiB settings. The M2 retains its 100-billion win, taking **7.6% less time**
than primesieve.

The Linux 26-case suite has **10 wins, 2 ties and 14 losses** against primesieve,
and **26/26 wins against primal**. Fourteen cases improve against the previous
Rust, eleven tie, and one scattered batch regresses by 7.5% while retaining its
C++ win. M2 has **19 wins, 3 ties and 4 losses**, and **26/26 wins against primal**.
The final short checks retain **24/24** wins and the unaligned holdout **19/24**.

The [report](benchmarks/X86_COUNT_100B.md) includes the remaining losses, a
borderline ARM batch repeat, every rejected experiment, source identity and raw
samples. Original benchmark results and hashes remain unchanged.

### v0.1.1 CI results — 26 November

The [release tag run](https://github.com/melihbirim/primeroute/actions/runs/45855473920)
and [matching main run](https://github.com/melihbirim/primeroute/actions/runs/45855475507)
both measure release commit `74548f1`. All test and benchmark jobs pass on native
x86-64 and Apple Silicon ARM64. A successful job means the checks and benchmarks
completed; it does not mean Rust wins every comparison.

Each run has **20 cases**: two prefix counts, eight range count/list cases and
ten batch workloads. All comparators use **one thread**. The tables count a win
when Rust takes less than 97% of the comparator's time, a loss above 103%, and a
tie in between. Primesieve uses the fastest configuration median measured for
each case; primal ranges use the segmented adapter described above.

| Run | CPU reported by runner | vs primesieve: wins / ties / losses | vs primal: wins / ties / losses |
| --- | --- | ---: | ---: |
| Release tag | AMD EPYC 7763, AVX2 | 8 / 2 / 10 | 18 / 1 / 1 |
| Main | AMD EPYC 9V74, AVX-512 available | 10 / 1 / 9 | 20 / 0 / 0 |
| Release tag | Apple M2 Pro (Virtual) | 16 / 0 / 4 | 20 / 0 / 0 |
| Main | Apple M1 (Virtual) | 13 / 1 / 6 | 20 / 0 / 0 |

The runs use the same release source but different CPUs. Hardware differences
and shared-runner variability affect timings, so these totals are not a
before/after code comparison.

On Apple Silicon, counting to one billion beats primesieve in both runs, as do
most short ranges and batches. Counting to 100 million, the 100-million-wide
range near 10¹² in both count and list modes, and scattered batches near 10¹²
lose in both runs. On x86, primesieve leads most range cases and several dense
batches. Scattered, packed, spaced and isolated batches near 10¹⁴ beat primesieve
on both x86 CPUs.

Primal is slower in most cases, with two exceptions in the EPYC 7763 tag run:
scattered batches near 10⁸ tie, and scattered batches near 10¹² take **9.1% more
time** than primal. The **26/26** primal wins above describe the dedicated
26-case experiment on its recorded machines, rather than every CI runner.

CI counts only to **100 million and one billion**. It does not remeasure
10 billion, 100 billion, the 24-case short suite or its unaligned holdout. The
original 26-case reports and 100-billion measurements above remain separate,
with their original results and hashes unchanged. These release CI results
support the broad workload differences; they do not reproduce every published
timing or independently validate the 100-billion result.

### Earlier platform checks — 25–26 November

The tables above retain the original M2 measurements and their original hashes.
The earlier [platform report](benchmarks/X86.md) measures revision `4daa474`
against its preserved baseline, primesieve and primal. At **100 billion**, one thread:

| CPU | Previous Rust | Current Rust | primesieve | primal |
| --- | ---: | ---: | ---: | ---: |
| AMD EPYC 9V74 VM, AVX2 | 22.883 s | 17.668 s | 13.887 s | 31.934 s |
| Apple M2 Pro | 9.385 s | 9.375 s | 10.369 s | 24.794 s |

On x86, counting improves 22–34% and range listing improves 33–46% against the
baseline. Primesieve still leads most full-sieve workloads. The paired 26-case
suite gives Rust **6 wins, 1 tie and 19 losses** against primesieve, and **25/26
wins against primal**. The one-billion count is within the 3% tie band.

The M2 retains all twenty baseline wins: **20 wins, 2 ties and 4 losses** against
primesieve, and **26/26 wins against primal**. Final short checks retain **24/24**
and the unaligned holdout **19/24**. A few M2 cases are modestly slower than the
baseline; the report lists them.

Main paired measurements were made on 25 November; final M2 short checks on
26 November. These are separate experiments with their own hashes. The
[CI workflow](.github/workflows/benchmark.yml) runs a reduced benchmark suite
of twenty cases on both platforms for pushes and pull requests. Its total is
different from the 26-case paired suite. Shared runners are useful for
finding regressions; close timings need repeated measurements.

## How it works

Sparse next-prime queries test candidates directly. Dense query groups share a
segmented sieve. The router estimates the cost of each approach per group.

**Direct testing.** Skip multiples of 2, 3 and 5. Filter more small factors.
Run deterministic Miller–Rabin on the survivors. The test covers every `u64`.
Larger candidates use Montgomery arithmetic. Checked increments handle the
end of the integer range.

**Range sieving.** Store eight candidates per byte using a mod-30 bitmap.
Process windows of up to 128 KiB on the portable path and 512 KiB on x86-64.
Static patterns remove factors 7 through 167.
Dense primes use eight marking streams. Medium primes use rotating wheel steps.
Large primes wait in buckets until the next window they hit.

**Short requests.** Fuse the pre-sieve masks into one pass. Align dense streams
using fixed seed metadata and exact integer reciprocal corrections. On Apple
Silicon, NEON table lookups rotate the streams into increasing order. Final-window
marking discards state updates that no later window will use.

**Long requests.** The portable path prepares about 2.4 MB of expanded masks
once per long call. On AVX2-capable x86 CPUs, balanced cyclic tables use 108,497
bytes of static pattern data and avoid those expanded allocations. Runtime
detection can also select AVX-512F filling. Overlapping final vector stores use
idempotent AND to avoid scalar cleanup at most pattern boundaries.

Counting and decoding retain NEON on Apple Silicon. On x86-64, runtime detection
selects AVX2 population counts, stream rotation and list decoding, with POPCNT
and portable alternatives. Default builds require no explicit CPU flag.

The x86 long-range path marks primes through 8,192 in 32 KiB chunks. Larger
active primes use compact wheel queues and eight shared class kernels, crossing
off eight multiples per full cycle. Outer windows are 512 KiB. ARM marking,
NEON and router algorithms retain their existing paths. Short calls avoid the
extra grouping setup.

Batch sieve groups read answers directly from the bitmap and share one seed
table per call. Roots through 2²⁰ copy a prefix of the existing static seed
metadata; larger tables are generated for the call. They do not materialize
the full interval prime list.

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
cargo fmt --manifest-path benchmarks/primal/Cargo.toml --check
cargo clippy --all-targets --locked -- -D warnings
cargo clippy --manifest-path benchmarks/primal/Cargo.toml --all-targets --locked -- -D warnings
python3 -m unittest discover -s benchmarks -p 'test_*.py'
```

All **29 tests** pass in debug and release on the measured ARM machine.
The native x86 jobs pass **38 correctness tests** in both profiles; four ignored
kernel profiles run separately in the experiment workflow.
Formatting and Clippy pass for the main crate and benchmark crate.
Summary regression tests check threshold boundaries and consistent win totals
across count, range and batch reports.

Coverage includes independent sieve comparisons, known counts, deterministic
primality checks, prime-gap and integer endpoints, batch ordering and limits,
bucket scheduling, static seeds, exact reciprocal quotients and NEON stream
rotation. Both decoder widths are checked against all 65,536 two-byte masks.
Unsafe marking and decoding paths have documented bounds and targeted tests.

Range benchmarks compare counts and wrapping-u64 checksums. Batch benchmarks
compare full answer arrays. A separate helper cross-checks primality and
next-prime results against primal.

The published M2 Pro measurements come from the pre-rename `prime-numbers`
build in the [benchmark manifest](benchmarks/measurement-2026-11-25.json).
The x86 additions and subsequent package, CLI and benchmark changes are not part
of that historical build. Its original source and binary hashes remain intact.
The current [counting report](benchmarks/X86_COUNT_100B.md) records its own
source and binary hashes. Previous executables,
reports and experiments are retained in the
[archive](benchmarks/experiments/short-range-2026-11-25/README.md).

Reproduce the [counting](benchmarks/README.md#reproduce),
[range](benchmarks/RANGES.md#validation-and-reproduction) and
[batch](benchmarks/BATCH_NEXT.md#reproduce) comparisons using their report commands.
