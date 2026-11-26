## x86 v3

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 13.031 ms | 9.538 ms | 13.196 ms | ❌ 1.37× |
| 1,000,000,000 | 133.250 ms | 107.495 ms | 167.831 ms | ❌ 1.24× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.302 ms | 0.229 ms | 0.854 ms | ❌ 1.32× |
| 1,000,000,000 | 1,000,000 | generate | 0.574 ms | 0.309 ms | 1.487 ms | ❌ 1.85× |
| 1,000,000,000 | 100,000,000 | count | 15.674 ms | 11.068 ms | 81.168 ms | ❌ 1.42× |
| 1,000,000,000 | 100,000,000 | generate | 62.571 ms | 36.177 ms | 159.854 ms | ❌ 1.73× |
| 1,000,000,000,000 | 1,000,000 | count | 1.756 ms | 1.329 ms | 2.710 ms | ❌ 1.32× |
| 1,000,000,000,000 | 1,000,000 | generate | 2.052 ms | 1.366 ms | 3.255 ms | ❌ 1.50× |
| 1,000,000,000,000 | 100,000,000 | count | 36.871 ms | 22.645 ms | 190.070 ms | ❌ 1.63× |
| 1,000,000,000,000 | 100,000,000 | generate | 75.677 ms | 42.329 ms | 256.172 ms | ❌ 1.79× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.414 ms | 1.873 ms | 1.686 ms | ✅ 0.75× |
| clustered_1e8 | 8,192 | 0.680 ms | 1.001 ms | 5.903 ms | ✅ 0.68× |
| scattered_1e12 | 2,048 | 5.895 ms | 4.408 ms | 4.424 ms | ❌ 1.34× |
| clustered_1e12 | 8,192 | 3.325 ms | 2.414 ms | 14.957 ms | ❌ 1.38× |
| scattered_1e14 | 2,048 | 5.296 ms | 14.762 ms | 8.039 ms | ✅ 0.36× |
| clustered_1e14 | 8,192 | 11.126 ms | 7.278 ms | 25.957 ms | ❌ 1.53× |
| packed_1e14 | 8,192 | 1.737 ms | 3.641 ms | 2.283 ms | ✅ 0.48× |
| mixed_1e12 | 14,336 | 8.373 ms | 7.932 ms | 17.959 ms | ❌ 1.06× |
| spaced_1e14 | 1,024 | 2.651 ms | 50.647 ms | 4.011 ms | ✅ 0.05× |
| isolated_1e14 | 1,024 | 2.711 ms | 2.597 s | 4.056 ms | ✅ <0.01× |

