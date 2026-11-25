# Single-threaded prefix-counting benchmarks

Measured on **2026-11-25, Apple M2 Pro**, using the current bucket-sieve source.
Each executable counts all primes up to the same inclusive limit, with printing
disabled. Times are **whole-process wall times**, including startup, setup,
computation and output capture; compilation is excluded. These differ from the
native API timings in the [range](RANGES.md) and [batch](BATCH_NEXT.md) reports.

| Implementation | 100 million | 1 billion | 10 billion |
| --- | ---: | ---: | ---: |
| Our Rust sieve | 7.403 ms | 59.717 ms | 746.628 ms |
| C++ primesieve 12.18, default | 6.888 ms | 64.398 ms | 759.071 ms |
| C++ primesieve 12.18, 1024 KiB | 6.909 ms | 64.394 ms | 759.959 ms |
| Rust primal 0.3.3 | 11.770 ms | 131.436 ms | 1752.443 ms |

Against the faster C++ median, Rust uses:

- **7.5% more time** at 100,000,000.
- **7.3% less time** at 1,000,000,000.
- **1.6% less time** at 10,000,000,000.

## 100 billion

| Implementation | Median |
| --- | ---: |
| Our Rust sieve | 11.574 s |
| C++ default | 10.052 s |
| C++ 1024 KiB | 10.224 s |

Rust takes **15.1% more time** than the faster C++ median.
Primal was not measured at this limit, matching the earlier comparison scope.

## Method and interpretation

Through ten billion, each implementation receives five warm-up rounds and
30 measured rounds. At 100 billion there is one warm-up and five measurements.
Run order is interleaved, rotated and reversed between rounds. All timed
programs compute their answers; known counts are used only for validation.
Every run agrees on the following inclusive counts:

| Limit | Prime count |
| ---: | ---: |
| 100,000,000 | 5,761,455 |
| 1,000,000,000 | 50,847,534 |
| 10,000,000,000 | 455,052,511 |
| 100,000,000,000 | 4,118,054,813 |

C++ explicitly uses `--threads=1`; Rust and primal spawn no workers. C++ default
and 1024 KiB sieve settings are both reported; the latter is retained from the
previous cache sweep and was not retuned on this source revision. No CPU affinity
was set. Native CPU-cache detection was available. Rust uses release LTO and one
codegen unit; C++ is the static Release primesieve build. Versions, CPU metadata
and per-command timing ranges are recorded in the JSON. Small median differences
and overlapping timing ranges do not establish universal performance advantages.

These runs replace earlier prefix results from an older source version. Historical
18% and 11.9% advantages at one and ten billion belong to that earlier version;
use the current table above for this implementation. This rerun includes several
intervening optimizations, not an isolated bucket-sieve comparison. In particular,
roots through the 100-billion limit remain below the large-prime bucket threshold;
the new [high-start range cases](RANGES.md#new-high-start-cases) exercise that path.

## Raw data and source identity

- [results.json](results.json): 100 million.
- [results-1b.json](results-1b.json): one billion.
- [results-10b.json](results-10b.json): ten billion.
- [results-100b.json](results-100b.json): 100 billion.

All four reports measure `src/lib.rs` SHA-256 `3bd6266b9b3e9a41ae0b532bfae004619e25f4e9869346a5604067f2f3ff237a`.
Sources and binaries were frozen before timing and verified before completed
reports were published atomically. The [run manifest](measurement-2026-11-25.json)
also records the source snapshot and commands. Previous measurements and report
text are preserved under
[experiments/pre-buckets-2026-11-24](experiments/pre-buckets-2026-11-24/README.md).
Earlier tuning files retain their original historical identities.

## Reproduce

Build C++ primesieve 12.18 as a static Release library:

```sh
cmake -S /path/to/primesieve -B /path/to/primesieve/build \
  -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF
cmake --build /path/to/primesieve/build --parallel 4
```

Build the Rust executables and run the comparisons from the project root:

```sh
cargo build --release --locked
cargo build --release --locked --manifest-path benchmarks/primal/Cargo.toml --bins
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 100000000 --warmups 5 --runs 30 \
  --output benchmarks/results.json
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 1000000000 --warmups 5 --runs 30 \
  --output benchmarks/results-1b.json
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 10000000000 --warmups 5 --runs 30 \
  --output benchmarks/results-10b.json
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 100000000000 --skip-primal --warmups 1 --runs 5 \
  --output benchmarks/results-100b.json
```

`--cpp-size` adds a comparator and preserves the default C++ run.
`--baseline` adds an older executable. The separate benchmark Cargo project uses
primal; the main library remains dependency-free.
