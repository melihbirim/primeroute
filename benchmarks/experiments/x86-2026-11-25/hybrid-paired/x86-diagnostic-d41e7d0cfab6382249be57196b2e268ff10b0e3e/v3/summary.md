## x86 v3

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 10.888 ms | 7.525 ms | 10.325 ms | ❌ 1.45× |
| 1,000,000,000 | 102.546 ms | 83.178 ms | 131.662 ms | ❌ 1.23× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.211 ms | 0.176 ms | 0.612 ms | ❌ 1.20× |
| 1,000,000,000 | 1,000,000 | generate | 0.450 ms | 0.230 ms | 1.269 ms | ❌ 1.96× |
| 1,000,000,000 | 100,000,000 | count | 11.984 ms | 8.514 ms | 58.099 ms | ❌ 1.41× |
| 1,000,000,000 | 100,000,000 | generate | 51.684 ms | 28.789 ms | 137.116 ms | ❌ 1.80× |
| 1,000,000,000,000 | 1,000,000 | count | 1.503 ms | 1.112 ms | 2.282 ms | ❌ 1.35× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.737 ms | 1.162 ms | 2.931 ms | ❌ 1.49× |
| 1,000,000,000,000 | 100,000,000 | count | 30.023 ms | 15.424 ms | 163.622 ms | ❌ 1.95× |
| 1,000,000,000,000 | 100,000,000 | generate | 63.030 ms | 31.010 ms | 235.995 ms | ❌ 2.03× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.146 ms | 1.407 ms | 1.486 ms | ✅ 0.81× |
| clustered_1e8 | 8,192 | 0.558 ms | 0.873 ms | 5.145 ms | ✅ 0.64× |
| scattered_1e12 | 2,048 | 4.905 ms | 3.151 ms | 3.827 ms | ❌ 1.56× |
| clustered_1e12 | 8,192 | 2.755 ms | 1.980 ms | 12.965 ms | ❌ 1.39× |
| scattered_1e14 | 2,048 | 4.729 ms | 12.081 ms | 6.977 ms | ✅ 0.39× |
| clustered_1e14 | 8,192 | 9.252 ms | 6.779 ms | 22.608 ms | ❌ 1.36× |
| packed_1e14 | 8,192 | 1.504 ms | 3.094 ms | 1.983 ms | ✅ 0.49× |
| mixed_1e12 | 14,336 | 6.979 ms | 6.024 ms | 15.620 ms | ❌ 1.16× |
| spaced_1e14 | 1,024 | 2.366 ms | 35.632 ms | 3.468 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.381 ms | 2.244 s | 3.508 ms | ✅ <0.01× |

