## Linux X64 · 1387cc0

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 7.896 ms | 6.932 ms | 9.987 ms | ❌ 1.14× |
| 1,000,000,000 | 87.237 ms | 83.069 ms | 131.922 ms | ❌ 1.05× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.181 ms | 0.173 ms | 0.756 ms | ❌ 1.05× |
| 1,000,000,000 | 1,000,000 | generate | 0.257 ms | 0.231 ms | 1.181 ms | ❌ 1.12× |
| 1,000,000,000 | 100,000,000 | count | 11.150 ms | 8.497 ms | 74.495 ms | ❌ 1.31× |
| 1,000,000,000 | 100,000,000 | generate | 32.628 ms | 26.812 ms | 127.986 ms | ❌ 1.22× |
| 1,000,000,000,000 | 1,000,000 | count | 1.159 ms | 1.105 ms | 2.466 ms | ❌ 1.05× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.240 ms | 1.164 ms | 2.824 ms | ❌ 1.07× |
| 1,000,000,000,000 | 100,000,000 | count | 28.121 ms | 15.265 ms | 180.356 ms | ❌ 1.84× |
| 1,000,000,000,000 | 100,000,000 | generate | 44.111 ms | 29.101 ms | 222.599 ms | ❌ 1.52× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.034 ms | 1.360 ms | 1.439 ms | ✅ 0.76× |
| clustered_1e8 | 8,192 | 0.536 ms | 0.818 ms | 5.042 ms | ✅ 0.66× |
| scattered_1e12 | 2,048 | 5.655 ms | 3.040 ms | 3.808 ms | ❌ 1.86× |
| clustered_1e12 | 8,192 | 2.289 ms | 1.915 ms | 12.980 ms | ❌ 1.20× |
| scattered_1e14 | 2,048 | 4.596 ms | 11.224 ms | 6.984 ms | ✅ 0.41× |
| clustered_1e14 | 8,192 | 6.846 ms | 6.306 ms | 22.503 ms | ❌ 1.09× |
| packed_1e14 | 8,192 | 1.440 ms | 2.989 ms | 1.994 ms | ✅ 0.48× |
| mixed_1e12 | 14,336 | 7.504 ms | 5.848 ms | 15.532 ms | ❌ 1.28× |
| spaced_1e14 | 1,024 | 2.288 ms | 34.081 ms | 3.453 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.328 ms | 2.228 s | 3.483 ms | ✅ <0.01× |

