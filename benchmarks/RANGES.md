# Fresh range-query benchmarks

Measured on **2026-11-25, Apple M2 Pro**, single threaded, against primesieve
12.18 and primal 0.3.3. Each query creates its own bitmap and marking state.
The tables show milliseconds per fresh query, including setup, allocation,
result checksums and vector destruction. Compilation and process startup are
excluded from these native API timings.

## Standard ranges

The previous Rust executable is remeasured in the same interleaved rounds, so
these before/after columns do not reuse historical timing numbers.

### Materialized prime lists

| Start | Width | Previous Rust | New Rust | Best C++ | Primal adapter |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | 0.189 | 0.110 | 0.117 | 0.925 |
| 1,000,000,000 | 100,000,000 | 10.044 | 10.037 | 10.854 | 91.092 |
| 100,000,000,000 | 1,000,000 | 0.451 | 0.282 | 0.335 | 1.398 |
| 100,000,000,000 | 100,000,000 | 14.821 | 14.666 | 14.208 | 129.658 |
| 1,000,000,000,000 | 1,000,000 | 0.957 | 0.700 | 0.882 | 2.226 |
| 1,000,000,000,000 | 100,000,000 | 22.209 | 22.123 | 16.860 | 176.094 |

### Counts

| Start | Width | Previous Rust | New Rust | Best C++ | Primal adapter |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | 0.150 | 0.068 | 0.092 | 0.540 |
| 1,000,000,000 | 100,000,000 | 5.434 | 5.348 | 6.920 | 52.051 |
| 100,000,000,000 | 1,000,000 | 0.415 | 0.255 | 0.310 | 1.075 |
| 100,000,000,000 | 100,000,000 | 11.145 | 11.191 | 10.877 | 95.725 |
| 1,000,000,000,000 | 1,000,000 | 0.933 | 0.670 | 0.870 | 1.894 |
| 1,000,000,000,000 | 100,000,000 | 18.669 | 18.723 | 13.495 | 144.097 |

Rust has lower medians in **8/12 standard cases**: **4/6 counts** and **4/6 lists**.
The three million-number intervals account for 6/6 wins across count and list operations.
The earlier short-range regressions are compared with the preserved kernel in
the tables above. Wide intervals remain a separate limitation; all losses are
retained here. Small gaps should be interpreted with the raw sample ranges.

Thirty measurements follow five discarded warm-up batches. Each sample times
three fresh queries, after one untimed query inside its process. C++ default,
64, 256 and 1024 KiB settings are all timed; **Best C++** is the fastest median
among those settings. The benchmark does not select the fastest sample.

## Primal comparison

Primal 0.3.3 has no public start/end segmented-sieve API. The **Primal adapter**
is an independent conventional odd-only segmented range implementation that
uses `primal::Sieve::new(sqrt(end))` to generate seeds. Its seed generation,
alignment and marking are all timed. It is benchmark code, not a native primal
range-sieve feature or a claim about the best possible primal-based adapter.
The adapter is limited to roots at most 2^24 to bound benchmark resources.

The separate short suite also scans candidates using the public exact
`primal::is_prime` API, skipping multiples of 2, 3 and 5. It uses seven measured
samples after one discarded warm-up, one fresh query per sample, and the same
interleaving and count/checksum validation. This exposes both the public-API
comparison and the faster segmented adapter instead of conflating them.

| Start | Width | Rust count | C++ count | Rust list | C++ list | Adapter list | Primal direct list |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000 | 0.011 | 0.020 | 0.011 | 0.020 | 0.022 | 0.049 |
| 1,000,000,000 | 10,000 | 0.020 | 0.041 | 0.021 | 0.043 | 0.043 | 0.467 |
| 1,000,000,000 | 100,000 | 0.041 | 0.057 | 0.037 | 0.056 | 0.112 | 4.680 |
| 1,000,000,000 | 1,000,000 | 0.076 | 0.101 | 0.113 | 0.127 | 0.930 | 45.927 |
| 100,000,000,000 | 1,000 | 0.059 | 0.093 | 0.061 | 0.071 | 0.094 | 0.238 |
| 100,000,000,000 | 10,000 | 0.080 | 0.135 | 0.080 | 0.125 | 0.138 | 2.162 |
| 100,000,000,000 | 100,000 | 0.161 | 0.298 | 0.168 | 0.287 | 0.304 | 21.992 |
| 100,000,000,000 | 1,000,000 | 0.268 | 0.331 | 0.293 | 0.353 | 1.455 | 216.568 |
| 1,000,000,000,000 | 1,000 | 0.167 | 0.248 | 0.170 | 0.183 | 0.322 | 0.208 |
| 1,000,000,000,000 | 10,000 | 0.184 | 0.299 | 0.192 | 0.244 | 0.380 | 2.012 |
| 1,000,000,000,000 | 100,000 | 0.311 | 0.562 | 0.306 | 0.526 | 0.591 | 20.805 |
| 1,000,000,000,000 | 1,000,000 | 0.680 | 0.899 | 0.736 | 0.907 | 2.277 | 206.049 |

Rust has lower medians than C++ in **24/24 short-suite cases**.
All direct primal and adapter count/list samples agree with both sieves.

## Unaligned holdout

Different starts and widths (137, 65,537, 1,000,033 and 4,194,319) check both
small intervals and the transition beyond one bitmap segment. These use fifteen
measurements after three warm-ups, with two fresh queries per sample. C++
default, 64 and 1024 KiB settings are measured.

Rust has lower medians in **19/24 holdout cases**.

All holdout cases still behind C++:

| Start | Width | Operation | Rust | Best C++ |
| --- | ---: | ---: | ---: | ---: |
| 1,001,234,567 | 4,194,319 | generate | 0.485 | 0.468 |
| 100,007,654,321 | 4,194,319 | count | 0.771 | 0.599 |
| 100,007,654,321 | 4,194,319 | generate | 0.900 | 0.710 |
| 1,000,023,456,789 | 4,194,319 | count | 1.320 | 1.047 |
| 1,000,023,456,789 | 4,194,319 | generate | 1.484 | 1.166 |

## High-start regression check

Seven measurements follow one warm-up; one fresh query is timed per sample.
C++ default and 1024 KiB settings are compared. These include the bucketed path
as well as single-window queries. Primal is omitted here because the adapter
resource limit does not cover all starts, and scanning forty million numbers
with primality tests is a different workload.

| Start | Width | Rust count | Best C++ count | Rust list | Best C++ list |
| --- | ---: | ---: | ---: | ---: | ---: |
| 100,000,000,000,000 | 1,000,000 | 3.557 | 4.875 | 3.493 | 4.416 |
| 100,000,000,000,000 | 40,000,000 | 18.621 | 12.397 | 20.031 | 13.758 |
| 1,000,000,000,000,000 | 1,000,000 | 8.143 | 9.991 | 8.295 | 8.530 |
| 1,000,000,000,000,000 | 40,000,000 | 39.186 | 28.943 | 40.114 | 29.678 |
| 1,000,000,000,000,000,000 | 1,000,000 | 188.225 | 214.250 | 187.827 | 160.890 |
| 1,000,000,000,000,000,000 | 40,000,000 | 286.956 | 314.623 | 289.444 | 277.891 |

These measurements keep high-start and wide-range losses visible; they do not
establish an advantage at every magnitude or width.

## Implementation and memory tradeoff

The long counting kernel remains the segmented mod-30 sieve with 128 KiB
windows, expanded masks for requests of at least eight windows, ordered dense
streams, packed medium wheel steps and bucketed large primes.

The single-window path adds:

- A fused pass over the seventeen compact pre-sieve patterns.
- Compiler-generated seed primes through 2^20 and exact integer reciprocal
  metadata. Multiply-high estimates plus remainder corrections replace repeated
  seed alignment divisions; no floating-point approximation is used.
- Dense-prime templates through 2^18, aligned per query. On AArch64, two NEON
  table lookups rotate the eight offsets into increasing order; portable slice
  rotation remains available. Short requests retain more dense streams where
  that is cheaper than walking sparse transitions.
- Read-only final-window marking: sorted tails are unrolled, and the state is
  not advanced for a nonexistent next segment. Large primes with at most one or
  two remaining hits avoid preparing further transitions.

The fixed seed/alignment arrays occupy **2,088,300 bytes (1.99 MiB)** before
linker alignment, in addition to existing mask and decoder tables. They enlarge
the executable and move bounded preprocessing to compilation. They contain seed
metadata, not benchmark answers. Bitmap construction, prime elimination, counting
and list generation still run for every call. Roots above 2^20 still stream
seeds. No computed answer or mutable prepared-state cache persists between calls.

Separate macOS `/usr/bin/time -l` observations near 10^18 measure process peak
RSS. Each process makes one untimed query and one timed fresh count; these are
single observations, not medians or allocator caps.

| Width | Rust peak RSS | C++ default peak RSS |
| --- | ---: | ---: |
| 1,000,000 | 9.67 MiB | 10.11 MiB |
| 40,000,000 | 38.00 MiB | 34.83 MiB |

## Validation and reproduction

All **28 tests** pass in debug and release, with formatting and Clippy passing
for both crates. Tests compare static seeds to an independent odd sieve, check
reciprocal quotients against integer division, compare aligned/rotated offsets
and final-window bitmaps with independent streams, and use u128 products to
check marking near usize::MAX. Existing prefix, bucket, range, decoder and batch
tests remain. Native range benchmarks compare exact counts and wrapping-u64
checksums, not every output element; batch benchmarks compare full answer arrays.

C++ explicitly sets and reports one thread; Rust and primal spawn no workers.
Rust uses release LTO and one codegen unit. The main crate remains dependency-free.
No affinity is set; only this ARM machine was measured. Both list helpers use
the same capacity estimate. Primal adapter code is at
[primal/src/bin/range_bench.rs](primal/src/bin/range_bench.rs).

Build the helpers as described in [README.md](README.md), then run:

```sh
cargo build --release --locked --example range_bench
cargo build --release --locked --manifest-path benchmarks/primal/Cargo.toml --bin range_bench
clang++ -O3 -std=c++14 -I/path/to/primesieve/include \
  benchmarks/range_bench.cpp /path/to/primesieve/build/libprimesieve.a \
  -pthread -o /tmp/prime-numbers-cpp-range-bench
python3 benchmarks/ranges.py --cpp /tmp/prime-numbers-cpp-range-bench \
  --primesieve /path/to/primesieve/build/primesieve --primal benchmarks/primal/target/release/range_bench \
  --baseline /path/to/previous-range-helper --cpp-sizes 64 256 1024 \
  --runs 30 --warmups 5 --repeats 3
```

- [range-results.json](range-results.json)
- [short-range-results.json](short-range-results.json)
- [short-range-holdout-results.json](short-range-holdout-results.json)
- [high-range-results.json](high-range-results.json)
- [high-range-memory.json](high-range-memory.json)

All reports measure `src/lib.rs` SHA-256 `c23c4a67eb8eac3d168047f30082c61d21de982237563510e6d75d83204ff1c3`.
The [manifest](measurement-2026-11-25.json) records exact commands, versions,
source snapshot and validation. Completed reports publish atomically only after
source and binary hashes are rechecked. Previous reports and exploratory
variants are retained in [the archive](experiments/short-range-2026-11-25/README.md).
