## x86 v3

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 10.172 ms | 6.838 ms | 9.906 ms | ❌ 1.49× |
| 1,000,000,000 | 101.996 ms | 82.983 ms | 131.301 ms | ❌ 1.23× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.213 ms | 0.176 ms | 0.608 ms | ❌ 1.21× |
| 1,000,000,000 | 1,000,000 | generate | 0.441 ms | 0.230 ms | 1.263 ms | ❌ 1.92× |
| 1,000,000,000 | 100,000,000 | count | 11.870 ms | 8.471 ms | 59.180 ms | ❌ 1.40× |
| 1,000,000,000 | 100,000,000 | generate | 49.871 ms | 27.298 ms | 136.580 ms | ❌ 1.83× |
| 1,000,000,000,000 | 1,000,000 | count | 1.497 ms | 1.109 ms | 2.324 ms | ❌ 1.35× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.718 ms | 1.155 ms | 2.937 ms | ❌ 1.49× |
| 1,000,000,000,000 | 100,000,000 | count | 29.667 ms | 15.235 ms | 163.748 ms | ❌ 1.95× |
| 1,000,000,000,000 | 100,000,000 | generate | 61.330 ms | 29.672 ms | 231.523 ms | ❌ 2.07× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.141 ms | 1.345 ms | 1.457 ms | ✅ 0.85× |
| clustered_1e8 | 8,192 | 0.548 ms | 0.822 ms | 5.123 ms | ✅ 0.67× |
| scattered_1e12 | 2,048 | 4.743 ms | 3.088 ms | 3.815 ms | ❌ 1.54× |
| clustered_1e12 | 8,192 | 2.676 ms | 1.904 ms | 12.925 ms | ❌ 1.41× |
| scattered_1e14 | 2,048 | 4.689 ms | 11.306 ms | 6.958 ms | ✅ 0.41× |
| clustered_1e14 | 8,192 | 9.010 ms | 6.323 ms | 22.503 ms | ❌ 1.42× |
| packed_1e14 | 8,192 | 1.464 ms | 2.984 ms | 1.975 ms | ✅ 0.49× |
| mixed_1e12 | 14,336 | 6.755 ms | 5.882 ms | 15.436 ms | ❌ 1.15× |
| spaced_1e14 | 1,024 | 2.357 ms | 34.198 ms | 3.454 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.368 ms | 2.232 s | 3.484 ms | ✅ <0.01× |

