## x86 v3

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 8.117 ms | 5.699 ms | 8.770 ms | ❌ 1.42× |
| 1,000,000,000 | 74.420 ms | 59.759 ms | 117.694 ms | ❌ 1.25× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.176 ms | 0.122 ms | 0.464 ms | ❌ 1.44× |
| 1,000,000,000 | 1,000,000 | generate | 0.409 ms | 0.174 ms | 0.987 ms | ❌ 2.35× |
| 1,000,000,000 | 100,000,000 | count | 8.872 ms | 6.965 ms | 42.182 ms | ❌ 1.27× |
| 1,000,000,000 | 100,000,000 | generate | 44.516 ms | 23.717 ms | 103.217 ms | ❌ 1.88× |
| 1,000,000,000,000 | 1,000,000 | count | 1.320 ms | 0.949 ms | 1.948 ms | ❌ 1.39× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.498 ms | 0.971 ms | 2.358 ms | ❌ 1.54× |
| 1,000,000,000,000 | 100,000,000 | count | 22.975 ms | 12.477 ms | 131.958 ms | ❌ 1.84× |
| 1,000,000,000,000 | 100,000,000 | generate | 53.893 ms | 25.291 ms | 180.145 ms | ❌ 2.13× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.845 ms | 1.004 ms | 1.315 ms | ✅ 0.84× |
| clustered_1e8 | 8,192 | 0.462 ms | 0.736 ms | 4.562 ms | ✅ 0.63× |
| scattered_1e12 | 2,048 | 3.966 ms | 2.471 ms | 3.303 ms | ❌ 1.60× |
| clustered_1e12 | 8,192 | 2.442 ms | 1.647 ms | 10.977 ms | ❌ 1.48× |
| scattered_1e14 | 2,048 | 3.799 ms | 9.666 ms | 5.945 ms | ✅ 0.39× |
| clustered_1e14 | 8,192 | 8.235 ms | 5.670 ms | 19.284 ms | ❌ 1.45× |
| packed_1e14 | 8,192 | 1.265 ms | 2.656 ms | 1.700 ms | ✅ 0.48× |
| mixed_1e12 | 14,336 | 5.945 ms | 5.358 ms | 13.950 ms | ❌ 1.11× |
| spaced_1e14 | 1,024 | 2.024 ms | 28.178 ms | 3.166 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 1.922 ms | 1.873 s | 2.931 ms | ✅ <0.01× |

