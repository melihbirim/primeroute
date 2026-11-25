# Counting-kernel optimization experiments

Exploratory runs used to select the final implementation. They are not the final
performance comparison. All timed programs compute the full sieve count; known
counts are used only by the drivers for validation. Each screening report records
all samples and candidate source/binary hashes. Sources and the original baseline
executable are retained in the corresponding directories.

Adopted: expanded masks for long requests, an outlined dense loop with ordered
tail marking, packed rotating medium-prime steps, and a small-root odd sieve.
Dense state stays 48 bytes on 64-bit targets by storing its bounded prime in u32.

Discarded: broad u32 arithmetic, branchless identity stores, cache-blocked marking,
extra pre-sieve primes, smaller segments, hybrid tails and the later adaptive request/initialization experiments. Their reports remain
for audit. The C++ cache sweep selected settings for fresh final runs.

Use ../../README.md and ../../results-100b.json for the final comparison.

The first complete production comparison is the final selected version. Later
adaptive experiments were not adopted because they did not consistently improve
short-range performance. The 100-million case remains effectively tied; no
universal performance-win claim is made.
