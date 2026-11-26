# Single-thread 100-billion counting

Rust now counts through 100 billion in **10.754 seconds** on the measured
EPYC 9V74 VM, down from **13.718 seconds**. That is **21.6% faster than the
preserved Rust baseline**. Primesieve takes **10.695 seconds**, leaving Rust
**0.54% behind**. This is inside the shared 3% tie band. The goal of a clear
100-billion win over primesieve was not achieved.

Every timed implementation uses one thread and sieves the complete interval.
Times are medians in seconds:

| CPU | Previous Rust | Selected Rust | Fastest primesieve | Primal |
| --- | ---: | ---: | ---: | ---: |
| AMD EPYC 9V74 VM | 13.718 | 10.754 | 10.695 | 24.959 |
| Apple M2 Pro | 8.982 | 8.992 | 9.730 | 23.924 |

The M2 retains its 100-billion win: **7.6% less time than primesieve**.
All programs return 4,118,054,813 primes.

The selected source is [commit `8f1606f`](https://github.com/melihbirim/primeroute/commit/8f1606f0ac34089425d6fad9325d1a7633a9a989).
Its source hashes match the selected snapshot in the completed
[independent native run](https://github.com/melihbirim/primeroute/actions/runs/45848350089).
The [raw Linux report](experiments/x86-count-2026-11-26/mixed-fill-confirmation/paired-results.json)
and [raw M2 report](experiments/x86-count-2026-11-26/apple-final-paired-results.json)
retain every sample, command, compiler version and source/binary hash.

## Whole-suite results

The same 26 cases cover four prefix counts, twelve count/list ranges and ten
next-prime batches. Nothing is removed from the score. The shared band is
below 0.97× for a win, above 1.03× for a loss, otherwise a tie.

| Platform | vs primesieve: wins / ties / losses | vs primal | vs previous Rust |
| --- | --- | --- | --- |
| Linux EPYC 9V74 | 10 / 2 / 14 | 26 / 0 / 0 | 14 / 11 / 1 |
| Apple M2 Pro | 19 / 3 / 4 | 26 / 0 / 0 | 2 / 23 / 1 |

The Linux scattered-100-million batch regresses by **7.5%** against the
baseline, while still beating both competitors. Primesieve continues to lead
many range cases. This implementation does not win every workload.

M2 short checks retain **24/24** C++ wins; the unaligned holdout retains **19/24**.
The original five-sample scattered-10¹⁴ batch moves from a C++ win to a tie at
the threshold: baseline 9.264 ms, candidate 9.293 ms, C++ 9.553 ms. The Rust
builds differ by only 0.3%. A fixed [31-sample repeat](experiments/x86-count-2026-11-26/apple-scattered-stability.json)
returns 5.582 / 5.631 / 8.513 ms respectively, with both Rust builds ahead.
The original 26-case score is unchanged. The large absolute timing variation
is a reason to treat close macOS batch results cautiously; M2 runs are not CPU-pinned.

The final x86 constants differ from the ARM-timed template only in the
cfg-excluded x86 source. A forced rebuild confirms all three timed ARM
executables are [byte-identical after final selection](experiments/x86-count-2026-11-26/arm-selected-equivalence.json).
Baseline ARM executables were captured before editing at the same project
location, avoiding a different dependency build identity in the paired comparison.

## What helped

Small primes retain marking in L1-sized chunks. Most other active primes use
eight-byte queued state and advance once per outer window. Their queues group
the starting wheel position. Eight shared class kernels replace 64 separate
kernels, and full cycles cross out eight multiples with one bounds check.
Squares still control prime activation; larger primes retain the bucket sieve.

The selected pre-sieve uses 108,497 bytes of balanced cyclic tables instead of
allocating seventeen expanded buffers. At a 512 KiB window, those buffers would
contain 9,079,353 bytes. AVX2 and runtime-detected AVX-512F paths are checked
against the original tables, including unaligned buffers and guard bytes.
Overlapping final vectors are valid because repeated AND with the same mask is
idempotent. The selected path uses six compact passes. The attempted five-pass layout was
not selected; its complete results remain in the archive. Each concrete variant
has its own source hashes and measurements.

Counting still sieves the complete interval. There is no combinatorial counting
shortcut, stored count or query-answer cache. The known answer, 4,118,054,813,
checks every timed invocation.

## What did not help

Compact in-place canonical state, queued dense state, reconstructed AVX2 stream
offsets and constant-mask tail unrolling all had losing configurations. They are
retained in the [complete experiment table](experiments/x86-count-2026-11-26/confirmed-100b.md)
and [all-case paired tables](experiments/x86-count-2026-11-26/paired-summary.md).
Absolute times from different VMs are not comparable.

One compact-table candidate beat primesieve's default setting: 12.012 versus
12.400 seconds on a Xeon 6973P. Primesieve's 256 KiB setting took 11.232 seconds,
so that candidate did not beat the fastest measured C++ configuration.

## Measurement

Linux runs natively on GitHub Actions VMs, with every timed process pinned to
one logical CPU. All three implementations use one thread. Compilation is
outside timing. Counter times include process startup and output capture.

Screening chooses a candidate. A separate confirmation then takes five measured
100-billion samples after one warm-up. Execution order rotates and reverses.
C++ is measured with default, 256 KiB and 1024 KiB sieve settings; the comparison
uses the fastest configuration median, never a selected individual sample.
Rust uses its default target with runtime ISA detection. Builds use rustc 1.94.1.

Each full confirmation also checks three smaller prefix counts, twelve inclusive
count/list ranges and ten next-prime batches. Range counts/checksums and complete
batch answer arrays must match. Sources and executables are hashed before and
after timing. The [audit](experiments/x86-count-2026-11-26/report.py) recomputes
medians and checks retained source snapshots and executable hashes.

Historical reports and their hashes remain unchanged. A
[240-file manifest](experiments/x86-count-2026-11-26/historical-results-sha256.json)
covers the existing result files and previous report generator.

## Validation and reproduction

The [final-source CI run](https://github.com/melihbirim/primeroute/actions/runs/45851475285)
passes tests, formatting, Clippy and the reduced benchmark suite on Linux and
macOS. Native x86 passes 38 correctness tests in debug and release, with four
separate ignored kernel profiles. ARM passes 29 in both profiles. Each screened
configuration also passes release tests before timing. Both crates pass fmt
and Clippy; Python summary tests and actionlint pass.

The [experiment workflow](../.github/workflows/x86-optimization.yml) builds the
preserved baseline and the candidate matrix, uploads screening evidence, then
runs an independent full confirmation. The [runner](counting26.py) and
[confirmation driver](confirm_count26.py) preserve exact generated sources.
Use a fresh output directory when reproducing. Existing raw reports are frozen.
The kernel design attribution is in [THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).
