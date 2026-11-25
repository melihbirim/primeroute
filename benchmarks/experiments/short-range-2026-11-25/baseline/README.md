# Single-threaded prefix-counting benchmarks

Measured on **2026-11-25, Apple M2 Pro**, using the current bucket-sieve source.
Each executable counts all primes up to the same inclusive limit, with printing
disabled. Times are **whole-process wall times**, including startup, setup,
computation and output capture; compilation is excluded. These differ from the
native API timings in the [range](RANGES.md) and [batch](BATCH_NEXT.md) reports.

| Implementation | 100 million | 1 billion | 10 billion |
| --- | ---: | ---: | ---: |
| Our Rust sieve | 6.908 ms | 51.909 ms | 626.010 ms |
| C++ primesieve 12.18, default | 6.939 ms | 64.348 ms | 755.881 ms |
| C++ primesieve 12.18, selected cache setting | 6.891 ms | 64.336 ms | 755.001 ms |
| Rust primal 0.3.3 | 11.686 ms | 130.941 ms | 1739.826 ms |

Against the faster C++ median, Rust uses:

- **0.2% more time** at 100,000,000.
- **19.3% less time** at 1,000,000,000.
- **17.1% less time** at 10,000,000,000.

## 100 billion

| Implementation | Median |
| --- | ---: |
| Our Rust sieve | 8.959 s |
| C++ default | 9.730 s |
| C++ selected cache setting | 9.771 s |
| Rust primal 0.3.3 | 23.859 s |
| Previous Rust kernel | 11.315 s |

Rust takes **7.9% less time** than the faster C++ median.
Primal is included at this limit in the current run. The original Rust
executable is also measured in the same interleaved rounds.

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

C++ explicitly uses `--threads=1`; Rust and primal spawn no workers. A cache
sweep tested 64, 128, 256, 512, 1024 and 2048 KiB before the final runs. The extra
setting is 256 KiB at 100 million and 1024 KiB at larger limits; the default is
also measured, and the faster median is used. No CPU affinity was set. Native
CPU-cache detection was available. Rust uses release LTO and one
codegen unit; C++ is the static Release primesieve build. Versions, CPU metadata
and per-command timing ranges are recorded in the JSON. Small median differences
and overlapping timing ranges do not establish universal performance advantages.

The 100-million case is effectively tied (C++ has the slightly lower median).
The one-billion, ten-billion and 100-billion cases favor Rust in this run;
this is not a universal win across workloads or CPUs. Range/list losses remain
visible in the other reports.

## What changed

The algorithm still constructs a full segmented sieve and crosses out multiples
through the requested bound. Known counts occur only in validation; there is no
stored answer for a benchmark limit and no persistent result cache.

- Long requests prepare expanded pre-sieve masks once; short requests keep the
  compact cyclic fill. Each prepared set is about 2.4 MB.
- Dense streams track their cyclic first index, so the maximum is its predecessor
  and the tail visits only actual hits. The bounded prime now uses `u32`, keeping
  the state at 48 bytes on this target. The marking loop remains outlined.
- Medium-prime masks, gaps and carries rotate in registers instead of loading a
  dependent transition-table record at each hit.
- Seed roots up to 65,536 use a bounded odd sieve, avoiding bulk-decoder startup.

All 24 tests pass in debug and release, with new independent checks for ordered
stream state, packed transitions and expanded masks. The roots through 100 billion
remain below the big-prime bucket threshold; this win comes from the dense and
medium kernels and pre-sieve handling.

Exploratory variants and all pilot samples are retained under
[experiments/count-100b-2026-11-25](experiments/count-100b-2026-11-25/README.md).
The C++ calibration selected settings for fresh final runs; final samples were
not selected from the pilots. Previous Rust is measured again in the same
100-billion rounds. It uses source SHA `3bd6266b…` and the preserved executable;
its source and the final source are both retained.

## Raw data and source identity

- [results.json](results.json): 100 million.
- [results-1b.json](results-1b.json): one billion.
- [results-10b.json](results-10b.json): ten billion.
- [results-100b.json](results-100b.json): 100 billion.

All four reports measure `src/lib.rs` SHA-256 `09abee545c4ac726fcc68c98cb12c0fe3514c7558b760f0bd4500f71db9f906f`.
Sources and binaries were frozen before timing and verified before completed
reports were published atomically. The [run manifest](measurement-2026-11-25.json)
also records the source snapshot and commands. Previous measurements and report
text are preserved under
[experiments/pre-count-optimization-2026-11-25](experiments/pre-count-optimization-2026-11-25/README.md).
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
  --cpp-size 256 --progress --limit 100000000 --warmups 5 --runs 30 \
  --output benchmarks/results.json
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 1000000000 --warmups 5 --runs 30 \
  --output benchmarks/results-1b.json
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 10000000000 --warmups 5 --runs 30 \
  --output benchmarks/results-10b.json
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 100000000000 --warmups 1 --runs 5 --baseline /path/to/previous-rust-counter \
  --output benchmarks/results-100b.json
```

`--cpp-size` adds a comparator and preserves the default C++ run.
`--baseline` adds an older executable. The separate benchmark Cargo project uses
primal; the main library remains dependency-free.
