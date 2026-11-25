# Single-threaded prefix-counting benchmarks

Measured on **2026-11-25, Apple M2 Pro**, with the frozen short-range-optimized source.
All programs perform full sieving to the same inclusive limit. Times include
process startup, setup, computation and output capture; compilation is excluded.

| Implementation | 100 million | 1 billion | 10 billion |
| --- | ---: | ---: | ---: |
| Our Rust | 8.212 ms | 52.668 ms | 637.159 ms |
| C++ default | 8.070 ms | 65.050 ms | 763.521 ms |
| C++ selected cache size | 8.135 ms | 65.066 ms | 764.615 ms |
| Rust primal 0.3.3 | 12.895 ms | 132.417 ms | 1759.883 ms |

## 100 billion

| Implementation | Median |
| --- | ---: |
| Our Rust | 9.132 s |
| C++ default | 10.061 s |
| C++ selected cache size | 10.099 s |
| Rust primal 0.3.3 | 24.757 s |
| Previous Rust (before short-range changes) | 9.247 s |

Rust uses **9.2% less time than the faster C++ median**.
The preserved previous Rust executable is measured in the same rounds. These
changes target short ranges; the 100-billion comparison checks that their
long-counting advantage is retained.

## Method and validation

Through ten billion, five warm-up rounds precede 30 measured rounds. At
100 billion there is one warm-up and five measurements. Commands rotate and
reverse order between rounds. C++ explicitly uses `--threads=1`; both Rust
programs spawn no workers. No CPU affinity is set, and native CPU/cache detection
is available. Both C++ default and an explicit cache size are timed: 256 KiB at
100 million, 1024 KiB at larger limits. Comparisons use the faster **median**,
not the fastest individual sample.

| Inclusive limit | Verified prime count |
| --- | ---: |
| 100,000,000 | 5,761,455 |
| 1,000,000,000 | 50,847,534 |
| 10,000,000,000 | 455,052,511 |
| 100,000,000,000 | 4,118,054,813 |

Known counts are validation data only. No timed implementation returns a stored
count. The new library embeds approximately **1.99 MiB of fixed seed/alignment
metadata** (primes through 2^20, exact integer reciprocals, and dense templates
through 2^18). This moves bounded seed preprocessing to compilation and enlarges
the binary. Runtime marking, result production, and all allocations are timed.
There is no computed-answer cache. See [RANGES.md](RANGES.md) for this tradeoff and
the short-range implementation; the long dense/medium loops retain the previous
optimized design.

All 28 tests pass in debug and release; formatting and Clippy pass for both
crates. Close medians on one ARM machine do not establish an all-input or
all-platform win. Several wide range/list cases still favor C++; the full range
report keeps those results visible.

## Raw data and source identity

- [results.json](results.json): 100 million.
- [results-1b.json](results-1b.json): one billion.
- [results-10b.json](results-10b.json): ten billion.
- [results-100b.json](results-100b.json): 100 billion.

All four measure `src/lib.rs` SHA-256 `c23c4a67eb8eac3d168047f30082c61d21de982237563510e6d75d83204ff1c3`.
The [manifest](measurement-2026-11-25.json) records every command and the complete
source snapshot. Sources and binaries are frozen and checked before completed
reports are published atomically. The immediate previous source, executable,
reports and documentation are retained in
[the short-range experiment archive](experiments/short-range-2026-11-25/README.md).

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
