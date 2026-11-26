# 100-billion x86 counting experiments

The selected implementation is documented in the [final report](../../X86_COUNT_100B.md).
It improves counting by 21.6% on the final EPYC VM and remains 0.54% behind
primesieve. This is a tie under the 3% band, not a claimed win. The
[complete 100-billion table](confirmed-100b.md) includes every completed experiment.

The selected `wide8_oldpasses_outer512` source is in
`mixed-fill-confirmation/sources/wide8_oldpasses_outer512/`. Root sources match
that snapshot. Its independent [native run](https://github.com/melihbirim/primeroute/actions/runs/45848350089)
uses template commit `b1fee675bde1ded5968361b9ab9a52cf83c6e200`; selected constants
are published as [commit 8f1606f](https://github.com/melihbirim/primeroute/commit/8f1606f0ac34089425d6fad9325d1a7633a9a989).

Final M2 data are `apple-final-paired-results.json`, `apple-final-short-results.json`
and `apple-final-holdout-results.json`. The earlier `apple-paired-results.json`
measures a rejected shared-code prototype and is retained separately. The final
ARM executable equivalence and targeted batch repeat are recorded in separate JSON
files; neither rewrites the original suite.

The baseline is the previously measured `4daa474` implementation, preserved in
`baseline/` with its SHA-256 manifest. Historical reports elsewhere in the
repository remain unchanged. Experiments run on native GitHub Actions x86 VMs;
all timed implementations use one thread and inherit affinity to one logical CPU.

Every candidate still sieves the complete interval. The expected count at
100 billion is 4,118,054,813; it is used to check every invocation, never to
answer a timed request. No count lookup or persistent result cache is used.

## Completed rejected experiments

These independently confirmed candidates did **not** improve the baseline and
were not adopted. Times are seconds. Each row compares programs on the same VM;
absolute times must not be compared between rows.

| Experiment | CPU | Previous Rust | Candidate | primesieve | primal |
| --- | --- | ---: | ---: | ---: | ---: |
| AVX2 expanded fill, `fill17` | AMD EPYC 9V74 | 14.509 | 14.822 | 11.285 | 26.142 |
| Queued compact dense state, `compact16` | AMD EPYC 7763 | 20.045 | 20.476 | 15.470 | 30.906 |

At 100 billion each program has five measured samples after one warm-up. C++
primesieve 12.18 is measured with default, 256 KiB and 1024 KiB sieve settings;
the table uses the fastest configuration median. Primal 0.3.3 uses its public
`StreamingSieve::prime_pi` API with the default portable target. All prefix
measurements include process startup and output capture.

Both confirmations also include all twelve inclusive count/list ranges and ten
next-prime batches. Nothing is omitted from the [full paired tables](paired-summary.md).
The [audit](report.py) recomputes medians and scores, verifies counts and source
snapshots, and checks the retained native executable hashes. Its calculated
outputs are in [assessment.json](assessment.json).

## Runs and source identity

- [Expanded-fill run](https://github.com/melihbirim/primeroute/actions/runs/45822078254),
  commit `8cadf826f13d4bae63c3c656d6f4e904d0fc9cb6`:
  [raw confirmation](first-confirmation/paired-results.json) and
  [selection evidence](first-confirmation/pilot.json).
- [Queued-state run](https://github.com/melihbirim/primeroute/actions/runs/45824992418),
  commit `007333e150880336adabf17193438d8d25df40b0`:
  [raw confirmation](cycle-confirmation/paired-results.json) and
  [selection evidence](cycle-confirmation/pilot.json).
- Canonical in-place experiments `f87a7a7` and `2ce9c92` stopped at a test's
  route-budget assertion before timing. Prime answers matched. The 512 KiB
  segment moved the test endpoint above its old seed-root budget; the corrected
  test derives its budget from the actual maximum query plus padding. Rejected
  builds are validation evidence, not performance results.

Screening measurements select candidates. They are not the final benchmark.
The separate confirmation rotates and reverses execution order, records every
sample, and verifies source and executable hashes before and after timing.
Compilation is excluded. Range and batch measurements include library setup
inside the process. The 3% comparison band is the same as CI.
