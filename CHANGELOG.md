# Changelog

## 0.1.1 — 26 November 2026

- Integrate the validated x86 experiments with runtime CPU detection, compact
  pre-sieve tables, vectorized fills and more efficient medium-prime marking.
- Retain single-threaded full sieving and the Apple Silicon implementation.
- Record paired comparisons against primesieve 12.18 and primal 0.3.3, including
  raw samples, source snapshots and executable hashes.
- Make CI summaries use the same 3% comparison band and add summary regression
  tests.

At 100 billion on the measured EPYC 9V74 VM, counting improves 21.6% over the
previous Rust executable. Rust takes 10.754 seconds against primesieve's
10.695 seconds: 0.54% slower, within the comparison band. On the measured M2 Pro,
Rust takes 8.992 seconds against primesieve's 9.730 seconds. All implementations
return 4,118,054,813 primes. These measurements describe the recorded machines;
they are not a performance guarantee for other hardware.

The full [benchmark report](benchmarks/X86_COUNT_100B.md) documents the remaining
losses and regressions. The original reports and hashes are unchanged. Version
0.1.1 keeps the measured Rust source unchanged; package metadata and lockfiles
advance to 0.1.1. To rerun the archived auditor's exact working-tree hash check,
use benchmark commit `4b25c0b`.
