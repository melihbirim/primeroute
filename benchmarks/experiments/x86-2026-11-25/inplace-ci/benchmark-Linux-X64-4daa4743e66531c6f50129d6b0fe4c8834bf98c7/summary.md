## Linux X64 · 4daa474

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 5.625 ms | 4.803 ms | 8.058 ms | ❌ 1.17× |
| 1,000,000,000 | 58.537 ms | 56.555 ms | 110.255 ms | ❌ 1.04× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.128 ms | 0.111 ms | 0.511 ms | ❌ 1.15× |
| 1,000,000,000 | 1,000,000 | generate | 0.180 ms | 0.159 ms | 0.889 ms | ❌ 1.13× |
| 1,000,000,000 | 100,000,000 | count | 7.847 ms | 6.586 ms | 49.325 ms | ❌ 1.19× |
| 1,000,000,000 | 100,000,000 | generate | 25.294 ms | 21.569 ms | 95.815 ms | ❌ 1.17× |
| 1,000,000,000,000 | 1,000,000 | count | 0.914 ms | 0.877 ms | 1.946 ms | ❌ 1.04× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.012 ms | 0.941 ms | 2.315 ms | ❌ 1.08× |
| 1,000,000,000,000 | 100,000,000 | count | 16.319 ms | 11.760 ms | 137.324 ms | ❌ 1.39× |
| 1,000,000,000,000 | 100,000,000 | generate | 29.642 ms | 22.867 ms | 170.201 ms | ❌ 1.30× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.817 ms | 0.952 ms | 1.293 ms | ✅ 0.86× |
| clustered_1e8 | 8,192 | 0.438 ms | 0.690 ms | 4.275 ms | ✅ 0.63× |
| scattered_1e12 | 2,048 | 3.507 ms | 2.301 ms | 3.087 ms | ❌ 1.52× |
| clustered_1e12 | 8,192 | 1.750 ms | 1.593 ms | 10.194 ms | ❌ 1.10× |
| scattered_1e14 | 2,048 | 3.644 ms | 8.985 ms | 5.642 ms | ✅ 0.41× |
| clustered_1e14 | 8,192 | 5.479 ms | 5.287 ms | 18.204 ms | ❌ 1.04× |
| packed_1e14 | 8,192 | 1.160 ms | 2.460 ms | 1.614 ms | ✅ 0.47× |
| mixed_1e12 | 14,336 | 4.975 ms | 4.754 ms | 12.459 ms | ❌ 1.05× |
| spaced_1e14 | 1,024 | 1.808 ms | 24.926 ms | 2.740 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 1.842 ms | 1.818 s | 2.836 ms | ✅ <0.01× |

