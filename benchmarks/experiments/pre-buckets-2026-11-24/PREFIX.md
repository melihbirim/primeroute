# Single-thread benchmark

For the new batch next-prime API and its current benchmark snapshot, see
[BATCH_NEXT.md](BATCH_NEXT.md). The prefix results below are historical.

Measured on 2026-11-24, Apple M2 Pro, macOS 26.5.1, ARM64. Count primes up to an
inclusive limit, with no prime printing.

For fresh `[start, end]` queries, including actual prime-list generation, see
[RANGES.md](RANGES.md). Those use native-API timings and the newer range-capable
implementation; the historical prefix measurements below retain their original
source/executable hashes.

| Implementation | 100 million | 1 billion | 10 billion |
| --- | ---: | ---: | ---: |
| Our optimized Rust sieve | 8.805 ms | 53.614 ms | 674.837 ms |
| C++ primesieve 12.18, default cache setting | 8.381 ms | 65.446 ms | 767.677 ms |
| C++ primesieve 12.18, 1024 KiB sieve | 8.584 ms | 65.347 ms | 765.783 ms |
| Rust primal 0.3.3 (primal-sieve 0.3.7) | 13.212 ms | 132.450 ms | 1775.098 ms |

These are median process wall times. Against the faster of the two C++ settings,
ours takes **18.0% less time at one billion** and **11.9% less at ten billion**.
At 100 million ours takes **5.1% more time**. At ten billion ours beat the tuned
C++ executable in 29 of 30 measured rounds. This is a result for these builds and
this machine, rather than a general claim about every CPU or upper bound.

At **100 billion**, the unchanged Rust version falls slightly behind C++:

| Implementation | Median |
| --- | ---: |
| Our optimized Rust sieve | 10.733 s |
| C++ primesieve, default cache setting | 10.390 s |
| C++ primesieve, 1024 KiB sieve | 10.441 s |

Ours takes **3.3% more time** than the faster C++ setting in this run. These
longer measurements use **one warm-up and five measured runs** per executable,
with the same alternating order and one thread. Primal was not included at this
limit. Every run returned **4,118,054,813** primes. The timing ranges overlap, so
this small lead should be read as a local five-run result. Source and executable
hashes match the preceding ten-billion benchmark; the Rust implementation was
unchanged.

All counts agree: **5,761,455**, **50,847,534**, and **455,052,511**, respectively.
Every implementation is single-threaded: C++ explicitly uses `--threads=1`; our
code and primal's streaming sieve do not spawn workers. All timed runs compute
the count from the requested bound. The benchmark script's known counts are only
used to validate the answers.

## Method

Through ten billion: five warm-ups and 30 measured runs per implementation, interleaved in alternating
order. Times include process startup, initialization, counting, and output
capture. Compilation is excluded. No CPU affinity was set. Runs execute outside
the Codex sandbox so C++ can detect CPU caches correctly.

Our sieve uses 128 KiB windows, previously selected by testing 16, 32, 64, 128 and
256 KiB. C++ selected 512 KiB by default. We also swept C++ sieve sizes of 32, 64,
128, 256, 512 and 1024 KiB at ten billion, with one warm-up and three measured
runs per setting. 1024 KiB had the lowest median in that exploratory sweep, very
close to the default. Both settings are included in the final 30-run comparison.
Primal uses its defaults.

Raw samples, commands, source and executable hashes, CPU cache information, and
versions are in:

- [results.json](results.json): 100 million.
- [results-1b.json](results-1b.json): one billion.
- [results-10b.json](results-10b.json): ten billion.
- [results-100b.json](results-100b.json): 100 billion, one warm-up and five measured runs.
- [tuning-results.json](tuning-results.json): exploratory variant/cache sweep.

Both Rust executables use Cargo release builds with LTO and one codegen unit,
rustc 1.94.1. C++ uses Apple Clang 21.0.0, CMake Release, and static linking to
libprimesieve. No custom CPU targeting flags were added. Parallel build jobs do
not affect runtime thread counts.

## Implementation

A mod-30 wheel packs eight candidates per byte, skipping multiples of 2, 3 and 5.
Seventeen compile-time repeating masks eliminate multiples of primes 7 through
167. Eight independent marking streams handle the remaining sieving primes.
Primes become active when their square reaches the current window; subsequent
positions stay relative to that window. After the bulk loop, each stream has at
most one remaining hit, handled with a single conditional store.

Counting uses packed population counts instead of enumerating every prime. On
AArch64, NEON processes 16 bytes per vector with two bounded accumulators. Other
CPUs use a portable fallback. Printing extracts primes in order from the same
sieve. The small-prime table grows through sqrt(limit), so the windowing approach
continues to work as the upper bound increases.

Profiling the previous version at ten billion showed about 802 ms in marking,
9.7 ms in mask initialization, and 8.7 ms in population counts. This motivated
extending the masks and reducing marking-loop bookkeeping.

The marking loop has a documented unsafe block: one maximum-index check guards
all eight stores. NEON loads are bounded to complete chunks, and accumulator
sizes are capped to prevent overflow. Debug and release tests compare full prime
lists with an independent sieve, cover segment boundaries and small inclusive
limits, check known counts through 100 million, and compare vector counts with
scalar counts for full buffers and tails. All benchmark counts through 100
billion match the known answers. Clippy passes with warnings denied.

[Primal's author describes it as a Rust port of primesieve](https://huonw.github.io/blog/2015/06/announcing-primal/).
We use its exact streaming counter, `StreamingSieve::prime_pi`. The separately
published [primesieve.rs](https://github.com/pthariensflame/primesieve.rs) is a C++
binding.

## Reproduce

Build [C++ primesieve v12.18](https://github.com/kimwalisch/primesieve/releases/tag/v12.18)
from its extracted source directory:

```sh
cmake -S /path/to/primesieve -B /path/to/primesieve/build \
  -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF
cmake --build /path/to/primesieve/build --parallel 4
```

Then, from this project's root, in a terminal with CPU cache information available:

```sh
cargo build --release --locked
cargo build --release --locked --manifest-path benchmarks/primal/Cargo.toml
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 1000000000 --output benchmarks/results-1b.json
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --progress --limit 10000000000 --output benchmarks/results-10b.json
python3 benchmarks/compare.py --primesieve /path/to/primesieve/build/primesieve \
  --cpp-size 1024 --skip-primal --progress --limit 100000000000 \
  --warmups 1 --runs 5 --output benchmarks/results-100b.json
```

`--cpp-size` adds a second C++ comparator; it preserves the default comparator.
`--skip-primal` restricts the comparison to our implementation and C++ primesieve.
Use `--limit`, `--warmups`, `--runs`, or `--output` to change the defaults. An
optional `--baseline /path/to/older-executable` adds another comparator. The
benchmark dependency is a separate Cargo project; the main project has no
dependencies.

Historical measurements are preserved in `baseline-results.json`,
`previous-results.json`, `previous-results-1b.json`, and `previous-results-10b.json`.
The historical ten-billion measurement used three runs; the latest uses thirty.
