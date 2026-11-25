# Short-range optimization experiments

The baseline directory preserves the previous 100-billion-winning source
(`09abee545c4ac726fcc68c98cb12c0fe3514c7558b760f0bd4500f71db9f906f`),
range/counter executables, reports and documentation. Both executables were
remeasured in the final interleaved runs.

All screening reports are exploratory, not the final performance comparison.
They retain all samples and hashes; saved candidate sources, profile crates and
logs show rejected variants. Some early intermediate variants are represented by
their hashed reports and later saved sources rather than complete build trees.

Selected: bounded compiler-generated seed metadata, exact integer reciprocal
alignment, fused cyclic masks, NEON rotation during dense initialization, a wider
single-window dense crossover, read-only sorted tail marking, and bounded sparse
hit tails. The existing NEON word decoder and long counting loops remain.

Rejected: floating quotient estimates, simple noninterleaved streams, enlarged
210/2310 wheels, larger/fused dense variants that slowed cases, byte decoders,
and scalar word-decoder replacements. Profile timings are selection evidence,
not final benchmark claims.

The fixed extra metadata is 2,088,300 bytes before linker alignment. No query
counts or output lists are precomputed. All timed requests build and mark their
own interval. This is a disclosed binary-size/preprocessing tradeoff.

`final-run.py` reproduces the exact sequential suite commands with local tool
paths adjusted as needed. `memory.py` records separate native peak-RSS points.
Completed canonical reports and source identity are in
[the manifest](../../measurement-2026-11-25.json),
[the range report](../../RANGES.md), and
[the counting report](../../README.md). Previous reports retain their original
identities; no pilot sample was promoted into a final timing result.
