# Fresh range-query benchmarks

Measured on **2026-11-25, Apple M2 Pro**, with one thread, C++ primesieve 12.18
and the current Rust source. All twelve standard cases and twelve new high-start
cases completed; counts and generation checksums agree. Source and binary hashes
were frozen before each suite and checked again before publication.

## Standard range suite

Milliseconds per fresh inclusive range query, including setup, allocation,
generation/checksum and vector destruction. Generation materializes ordered
64-bit prime vectors; counting does not allocate the output list.

| Start | Numbers in interval | Previous Rust list | Current Rust list | primesieve list |
| ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | 0.212 | 0.146 | 0.121 |
| 1,000,000,000 | 100,000,000 | 10.669 | 10.960 | 11.100 |
| 100,000,000,000 | 1,000,000 | 0.487 | 0.475 | 0.360 |
| 100,000,000,000 | 100,000,000 | 18.690 | 18.325 | 14.828 |
| 1,000,000,000,000 | 1,000,000 | 0.852 | 0.982 | 0.883 |
| 1,000,000,000,000 | 100,000,000 | 26.510 | 25.356 | 16.828 |

Rust records a lower median in **1/6 list-generation cases**. Close results
should be read with the sample ranges in the JSON. The previous column is the
last published range snapshot, which predates the batch and bucket optimizations;
this is not an isolated measurement of the bucket change.

| Start | Numbers in interval | Rust count | primesieve count |
| ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | 0.110 | 0.096 |
| 1,000,000,000 | 100,000,000 | 6.216 | 7.008 |
| 100,000,000,000 | 1,000,000 | 0.441 | 0.332 |
| 100,000,000,000 | 100,000,000 | 14.768 | 11.404 |
| 1,000,000,000,000 | 1,000,000 | 0.938 | 0.887 |
| 1,000,000,000,000 | 100,000,000 | 22.535 | 13.873 |

The standard suite uses five warm-up batches, 30 measured batches, and three
fresh queries per timed sample. Each native process first makes one untimed
query to warm code pages. The seed data and marking states are rebuilt for
each query; no result or prepared-state cache persists between requests.

## New high-start cases

The standard cases stop at 10^12, below the new bucket threshold. These new
cases use starts of 10^14, 10^15 and 10^18, with intervals spanning one million
or forty million numbers. The wider intervals exercise bucket scheduling; the narrow
intervals fit in a single segment and exercise streaming seed preparation.

Seven measured batches follow one warm-up batch; each sample times one fresh
query after one untimed warm-up query inside the process. All values are ms.

| Start | Width | Rust count | primesieve count | Rust list | primesieve list |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 100,000,000,000,000 | 1,000,000 | 3.864 | 4.911 | 3.995 | 4.523 |
| 100,000,000,000,000 | 40,000,000 | 20.841 | 12.990 | 20.900 | 13.341 |
| 1,000,000,000,000,000 | 1,000,000 | 9.241 | 10.092 | 9.655 | 8.820 |
| 1,000,000,000,000,000 | 40,000,000 | 41.899 | 29.402 | 42.594 | 29.562 |
| 1,000,000,000,000,000,000 | 1,000,000 | 221.049 | 214.943 | 216.631 | 160.287 |
| 1,000,000,000,000,000,000 | 40,000,000 | 285.691 | 311.003 | 288.407 | 277.009 |

The bucket implementation improves the algorithmic handling of infrequent
marks, but it does not establish a general speed advantage. In this run Rust
wins the 40-million-number count near 10^18; primesieve still leads on that
interval for list generation and on several other wide cases. Narrow-case
advantages depend on both the starting magnitude and count/list operation.

## Memory

[high-range-memory.json](high-range-memory.json) records separate peak-RSS
observations near 10^18 for one-million- and forty-million-number count requests.
These are process high-water marks from macOS `/usr/bin/time -l`, including one
untimed and one timed fresh query. They are single observations, not medians,
and do not constitute a before/after memory comparison with the old code.

| Width | Rust peak RSS | primesieve default peak RSS |
| ---: | ---: | ---: |
| 1,000,000 | 4.81 MiB | 9.16 MiB |
| 40,000,000 | 31.47 MiB | 31.41 MiB |

## Implementation and limits

Standalone range requests stream sieving primes through sqrt(end), keeping a
decoded seed window instead of the entire seed-prime vector. This reduces
storage but still pays seed generation and alignment time at high starts.

Dense primes retain eight marking streams with compact 32-bit relative offsets.
Medium primes stay in a flat list. Primes above **1,048,576** wait in a ring of
buckets until the next segment they mark. Big-prime states occupy eight bytes
and are stored in reusable chunks; states with no multiple in the range are
discarded. Prime squares beyond the ring are held in a sorted pending list.

The mod-30 bitmap, compact static pre-sieve patterns and bounded NEON decoders
also serve counting, streaming callbacks and materialized lists. Portable scalar
fallbacks remain. The standalone range APIs have no configurable memory cap;
batch next-prime queries have separate resource limits and retain one shared
seed table for their sieve groups.

## Validation and comparison method

All 21 Rust tests pass in debug and release; formatting and Clippy pass. Coverage
includes all two-byte decoder masks for both output widths, endpoint padding,
compact wheel transitions and offsets, independent small-range sieves, known
counts, and bucketed windows compared with single-window chunks near 1.2e12
and 1e15. Range benchmarks check exact counts and wrapping-u64 sums, not every
individual output element; batch next-prime benchmarks compare full answer arrays.

C++ explicitly sets and reports one thread. Rust spawns no workers. The table
uses the faster C++ median of the default and 1024 KiB settings for each case.
Both list helpers use the same initial capacity estimate. No CPU affinity was
set; these are measurements on one ARM machine. Native cache detection was
available. Rust uses release LTO and one codegen unit; C++ uses Apple Clang -O3
and the static primesieve library.

## API and reproduction

```rust
assert_eq!(prime_numbers::count_primes_in_range(100, 110), 4);
assert_eq!(prime_numbers::primes_in_range(100, 110), vec![101, 103, 107, 109]);
prime_numbers::for_each_prime_in_range(100, 110, |p| println!("{p}"));
```

Both endpoints are included. Library APIs treat inverted intervals as empty;
the CLI rejects them. Build the static C++ library as described in
[README.md](README.md), then run:

```sh
cargo build --release --locked --example range_bench
clang++ -O3 -std=c++14 -I/path/to/primesieve/include \
  benchmarks/range_bench.cpp /path/to/primesieve/build/libprimesieve.a \
  -pthread -o /tmp/prime-numbers-cpp-range-bench
python3 benchmarks/ranges.py --cpp /tmp/prime-numbers-cpp-range-bench \
  --primesieve /path/to/primesieve/build/primesieve --runs 30 --warmups 5 --repeats 3
python3 benchmarks/ranges.py --cpp /tmp/prime-numbers-cpp-range-bench \
  --primesieve /path/to/primesieve/build/primesieve --runs 7 --warmups 1 --repeats 1 \
  --starts 100000000000000 1000000000000000 1000000000000000000 \
  --widths 1000000 40000000 --output benchmarks/high-range-results.json
```

[range-results.json](range-results.json) and [high-range-results.json](high-range-results.json)
contain all samples, bounds, counts/checksums, commands and source/binary hashes.
Drivers publish completed reports atomically; partial data uses `.progress.json`.
The [run manifest](measurement-2026-11-25.json) records the commands and source
snapshot. Previous reports remain in
[experiments/pre-buckets-2026-11-24](experiments/pre-buckets-2026-11-24/README.md).
