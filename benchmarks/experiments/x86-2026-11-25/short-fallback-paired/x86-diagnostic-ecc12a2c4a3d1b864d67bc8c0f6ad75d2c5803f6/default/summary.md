## x86 default

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 9.732 ms | 5.548 ms | 8.711 ms | ❌ 1.75× |
| 1,000,000,000 | 92.194 ms | 59.400 ms | 116.159 ms | ❌ 1.55× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.189 ms | 0.117 ms | 0.544 ms | ❌ 1.62× |
| 1,000,000,000 | 1,000,000 | generate | 0.410 ms | 0.164 ms | 0.922 ms | ❌ 2.49× |
| 1,000,000,000 | 100,000,000 | count | 10.585 ms | 6.899 ms | 51.624 ms | ❌ 1.53× |
| 1,000,000,000 | 100,000,000 | generate | 45.336 ms | 23.847 ms | 101.782 ms | ❌ 1.90× |
| 1,000,000,000,000 | 1,000,000 | count | 1.355 ms | 0.918 ms | 2.037 ms | ❌ 1.48× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.554 ms | 0.958 ms | 2.331 ms | ❌ 1.62× |
| 1,000,000,000,000 | 100,000,000 | count | 24.881 ms | 12.428 ms | 142.141 ms | ❌ 2.00× |
| 1,000,000,000,000 | 100,000,000 | generate | 54.123 ms | 24.660 ms | 179.655 ms | ❌ 2.19× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.916 ms | 1.022 ms | 1.350 ms | ✅ 0.90× |
| clustered_1e8 | 8,192 | 0.474 ms | 0.760 ms | 4.688 ms | ✅ 0.62× |
| scattered_1e12 | 2,048 | 4.099 ms | 2.500 ms | 3.257 ms | ❌ 1.64× |
| clustered_1e12 | 8,192 | 2.592 ms | 1.679 ms | 10.964 ms | ❌ 1.54× |
| scattered_1e14 | 2,048 | 3.843 ms | 9.725 ms | 6.007 ms | ✅ 0.40× |
| clustered_1e14 | 8,192 | 8.240 ms | 5.774 ms | 19.861 ms | ❌ 1.43× |
| packed_1e14 | 8,192 | 1.235 ms | 2.665 ms | 1.722 ms | ✅ 0.46× |
| mixed_1e12 | 14,336 | 5.891 ms | 5.124 ms | 13.064 ms | ❌ 1.15× |
| spaced_1e14 | 1,024 | 1.917 ms | 27.356 ms | 2.966 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 1.954 ms | 1.883 s | 2.952 ms | ✅ <0.01× |

