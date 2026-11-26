## Linux X64 · 007333e

**primeroute wins against primesieve in 5/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 11.265 ms | 7.324 ms | 11.915 ms | ❌ 1.54× |
| 1,000,000,000 | 142.413 ms | 88.070 ms | 166.836 ms | ❌ 1.62× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.193 ms | 0.148 ms | 1.062 ms | ❌ 1.31× |
| 1,000,000,000 | 1,000,000 | generate | 0.275 ms | 0.206 ms | 1.783 ms | ❌ 1.33× |
| 1,000,000,000 | 100,000,000 | count | 16.259 ms | 10.175 ms | 102.703 ms | ❌ 1.60× |
| 1,000,000,000 | 100,000,000 | generate | 43.127 ms | 33.007 ms | 190.621 ms | ❌ 1.31× |
| 1,000,000,000,000 | 1,000,000 | count | 1.389 ms | 1.289 ms | 3.276 ms | ❌ 1.08× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.489 ms | 1.354 ms | 3.875 ms | ❌ 1.10× |
| 1,000,000,000,000 | 100,000,000 | count | 33.351 ms | 22.262 ms | 245.443 ms | ❌ 1.50× |
| 1,000,000,000,000 | 100,000,000 | generate | 53.255 ms | 39.908 ms | 313.498 ms | ❌ 1.33× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.932 ms | 1.532 ms | 2.300 ms | ❌ 1.26× |
| clustered_1e8 | 8,192 | 0.600 ms | 0.989 ms | 7.881 ms | ✅ 0.61× |
| scattered_1e12 | 2,048 | 6.110 ms | 3.653 ms | 5.222 ms | ❌ 1.67× |
| clustered_1e12 | 8,192 | 2.501 ms | 2.295 ms | 17.410 ms | ❌ 1.09× |
| scattered_1e14 | 2,048 | 6.002 ms | 13.533 ms | 9.121 ms | ✅ 0.44× |
| clustered_1e14 | 8,192 | 8.701 ms | 8.167 ms | 29.302 ms | ❌ 1.07× |
| packed_1e14 | 8,192 | 1.820 ms | 4.181 ms | 2.521 ms | ✅ 0.44× |
| mixed_1e12 | 14,336 | 8.535 ms | 7.378 ms | 20.878 ms | ❌ 1.16× |
| spaced_1e14 | 1,024 | 3.017 ms | 42.094 ms | 4.566 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 3.069 ms | 3.215 s | 4.501 ms | ✅ <0.01× |

