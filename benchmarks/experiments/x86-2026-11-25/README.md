# x86 experiments, 25 November 2026

The [platform report](../../X86.md) describes the selected implementation and
the paired Linux and M2 comparisons. All timings are native and single threaded.
No historical benchmark JSON is replaced by these experiments.

- `baseline/` preserves the source and helper snapshot from the start of this
  task. `baseline-sha256.json` identifies its files. Its two top-level executables
  are ARM artifacts; Linux builds this source afresh. Build directories are
  ignored by git.
- `build-pilot/`: default versus x86-64-v3 builds, commit `703ee0a`, AMD EPYC
  9V74. These sequential suites establish that build flags help, but are not the
  final paired comparison.
- `kernel-pilot/`: runtime vector kernels, commit `0b33d3a`, Intel Xeon Platinum
  8573C. Kernel profiles exclude full-query setup and are selection evidence.
- `cache-medium-paired/`: cache blocking plus grouped medium marking, commit
  `1387cc0`, AMD EPYC 7763. This pilot includes regressions: counting to 100
  billion rose from 27.597 s to 31.068 s on that VM.
- `dense-groups-paired/`: grouping dense primes by residue, commit `75dc448`,
  AMD EPYC 9V74. Counting to 100 billion still regressed, from 17.752 s to
  19.228 s; smaller prefixes and lists improved. The final adjustment visits
  larger dense-prime state once per window instead of once per small chunk.
- `cache-medium-ci/` and `dense-groups-ci/` preserve the corresponding reduced
  CI suites on Linux and macOS. Their VMs can differ from the paired experiments.
- `hybrid-paired/` and `hybrid-ci/` preserve `d41e7d0`: smaller-prime blocking
  with larger dense primes advanced once per window. Long counting improved,
  but short multi-window batches still paid excess setup costs.
- `short-fallback-paired/` and `short-fallback-ci/` preserve `ecc12a2`, AMD EPYC
  9V45 for the paired run. Flat marking with copied state still regressed the
  scattered and mixed batches. This candidate was superseded.
- `inplace-paired/` and `inplace-ci/` measure the selected `4daa474` source:
  runtime x86 kernels, grouped long marking, existing state arrays for short
  calls, and reuse of the embedded bounded seed prefix. The paired VM was an
  AMD EPYC 9V74 with AVX2 and masked AVX512 BW/VBMI2 features.
- `apple-paired-results.json` preserves the first complete M2 comparison of
  `75dc448`. `apple-hybrid-paired-results.json` records that hybrid source
  separately. The ARM executable hashes are identical between these builds.
- `apple-short-results.json` retains the first 15-sample short-range pass
  (22/24 candidate wins, 23/24 baseline wins), including its unstable timings.
  `apple-short-confirmation-results.json` repeats all 24 cases with 30 samples
  after five warm-ups, yielding 24/24 wins for both. The confirmation was a full
  repeat, not a selection of the fastest samples from either run.
- `apple-holdout-results.json` retains the unaligned 24-case check: 19/24 wins
  for both candidate and baseline.
- `apple-final-binary-equivalence.json` verifies that rebuilding the earlier
  fallback source (`ecc12a2`) on ARM produced the exact executables already
  measured at `d41e7d0`. Raw source hashes in the measured reports are unchanged.
- `apple-inplace-paired-results.json` measures the selected `4daa474` source
  directly, including all competitors and 100 billion. Its supplemental
  `apple-inplace-short-results.json` and `apple-inplace-holdout-results.json`
  were measured on 26 November and retain 24/24 and 19/24 wins respectively.
- `paired-summary.md` lists every final paired case. `assessment.json` records
  calculated outcomes and raw-report hashes. Run `report.py` to audit the data
  and regenerate both; it uses the same comparison thresholds as CI.

Hardware changes between jobs. Compare baseline and candidate within a paired
report, never absolute timings across different VM runs. Completed reports retain
every sample, command, compiler version and source/executable hash. Counts and
checksums are verified on every invocation; batch answer arrays agree in full.
