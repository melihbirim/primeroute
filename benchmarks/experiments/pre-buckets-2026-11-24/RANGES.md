# Fresh range-query benchmarks

This is the completed pre-batch-next range-generator snapshot identified by the
source hashes in `range-results.json`. For the latest scattered/clustered
next-prime workloads, see [BATCH_NEXT.md](BATCH_NEXT.md).

Measured on 2026-11-24, Apple M2 Pro, using one thread. These tests materialize actual ordered prime vectors from fresh inclusive range requests.

Our optimized generator now wins **two of six** tested intervals against the faster median of primesieve's default and 1024 KiB settings. Primesieve remains faster in the other four. The gains are local measurements for these builds and this machine.

All times below are milliseconds per fresh list-generation query. “Previous Rust” is the version from the preceding range benchmark, preserved in `range-previous-results.json`.

| Start | Numbers in interval | Previous Rust | Current Rust | C++ primesieve |
| ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | 0.292 | 0.212 | 0.121 |
| 1,000,000,000 | 100,000,000 | 13.583 | **10.669** | 11.032 |
| 100,000,000,000 | 1,000,000 | 1.337 | 0.487 | 0.351 |
| 100,000,000,000 | 100,000,000 | 24.258 | 18.690 | 15.120 |
| 1,000,000,000,000 | 1,000,000 | 4.250 | **0.852** | 0.905 |
| 1,000,000,000,000 | 100,000,000 | 41.411 | 26.510 | 17.688 |

The wide interval near one billion is `[1_000_000_000, 1_099_999_999]`.
Ours takes **3.3% less time** there and wins 27 of 30 paired batches. The narrow
interval near one trillion is `[1_000_000_000_000, 1_000_000_999_999]`; ours takes
**5.8% less time**, wins 24 of 30 batches, and is about **5x faster than our previous
version**. Timing ranges overlap in these close cases, so these are modest local
leads rather than a claim of universal superiority.

## What changed

- Larger seed-prime tables use the packed sieve itself, recursively shrinking the
  bound by sqrt until the small odd-only base case. Each query rebuilds its table.
- Dense primes keep the eight interleaved marking streams. Primes larger than a
  sieve window use a compact 16-byte wheel state instead of the 80-byte dense
  state on this 64-bit target.
- Wheel masks and multiplier transitions use compile-time tables. Binary search
  locates the active prime prefix, avoiding repeated square calculations while
  initializing every state.
- NEON list decoding handles two sieve bytes at once. A 65,536-entry, roughly
  1 MiB lookup packs candidate offsets; four independent lookups are batched
  before writing output. This is a bit-decoding table, not a cached prime list.
- The decoder stays in a separate function to reduce register pressure in the
  large marking loop. A scalar fallback remains available for other CPUs.

All six generation cases improved compared with the preceding version. The
larger high-start intervals still expose a substantial gap to C++.

## Method

Each case has five warm-up batches and **30 measured batches** per implementation.
Each sample averages three freshly constructed range queries. Rust and C++
batches are interleaved in alternating order. An untimed request inside each
process warms code pages. Process startup is excluded; per-query base-prime
preparation, sieve initialization, allocation, generation, checksum, and
vector deallocation are included. There is no retained sieve or result cache
between queries.

Generation compares `primes_in_range` with `primesieve::generate_primes`,
materializing ordered 64-bit prime vectors and computing their sums. Both use
the same initial capacity estimate. C++ explicitly uses
`primesieve::set_num_threads(1)` and reports the setting to the driver; our code
spawns no workers. Every count and generation checksum agrees across all cases.

C++ is primesieve 12.18, Apple Clang 21.0.0, Release mode; Rust is rustc 1.94.1,
release LTO and one codegen unit. CPU cache detection was allowed outside the
Codex sandbox. No CPU affinity was set.

[range-results.json](range-results.json) contains every sample, bound, count,
checksum, command, source/executable hash, and CPU metadata. The hashes were
verified against the final source and binaries. Historical stages remain in
[range-previous-results.json](range-previous-results.json) (15 measured batches)
and [range-before-results.json](range-before-results.json) (the first range
implementation). These native-API metrics differ from the historical prefix CLI
benchmarks, which include process startup.

## Correctness and bounds

Debug and release tests, plus Clippy with warnings denied, pass. Tests cover:

- All 65,536 two-byte masks, including dense masks, four-word batches, vector
  growth, zero blocks, and odd tails.
- Compact wheel transitions checked against ordinary integer multiplication
  across all residue classes and different starting positions.
- Full lists against an independent sieve for small, singleton, inverted,
  randomized, square-adjacent, and segment-crossing ranges.
- A high-start range checked by independent trial division, and known prime
  counts through 100 million.

The bulk decoder reserves 64 output slots for four words. Each word writes at
most 16 valid values or padding values; only valid entries are exposed through
`Vec::set_len`. Padding stays outside the vector's logical length. The remaining
pairs reserve 16 slots, and a final odd byte uses safe scalar pushes.

## Count-only results

Counting remains a separate workload, without materializing the list:


| Start | Numbers in interval | Rust count | C++ count |
| ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | 0.163 | 0.096 |
| 1,000,000,000 | 100,000,000 | 5.950 | 7.117 |
| 100,000,000,000 | 1,000,000 | 0.461 | 0.325 |
| 100,000,000,000 | 100,000,000 | 14.489 | 11.291 |
| 1,000,000,000,000 | 1,000,000 | 0.806 | 0.889 |
| 1,000,000,000,000 | 100,000,000 | 22.367 | 14.081 |

## Use the range APIs

```sh
cargo run --release -- 1000 1100
cargo run --release -- 1000 1100 --print
```

```rust
let count = prime_numbers::count_primes_in_range(1_000, 2_000);
let primes = prime_numbers::primes_in_range(1_000, 2_000);
prime_numbers::for_each_prime_in_range(1_000, 2_000, |p| println!("{p}"));
```

Both endpoints are included. Library APIs treat inverted intervals as empty;
the CLI rejects them. Single-bound calls start at zero. The callback API streams
values; the ordered-vector API uses the optimized bulk decoder.

## Reproduce

After building the static C++ library as described in [README.md](README.md):

```sh
cargo build --release --example range_bench
clang++ -O3 -std=c++14 -I/path/to/primesieve/include \
  benchmarks/range_bench.cpp /path/to/primesieve/build/libprimesieve.a \
  -pthread -o /tmp/prime-numbers-cpp-range-bench
python3 benchmarks/ranges.py --cpp /tmp/prime-numbers-cpp-range-bench \
  --primesieve /path/to/primesieve/build/primesieve --runs 30 --warmups 5
```

Run where CPU cache information is accessible. The driver also accepts
`--repeats` and `--output`. Its default run/warm-up counts are the older 15/3;
the explicit flags above reproduce the latest 30/5 comparison.
