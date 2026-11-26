## x86 runtime candidate

**primeroute wins against primesieve in 5/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 15.879 ms | 6.865 ms | 11.521 ms | ❌ 2.31× |
| 1,000,000,000 | 174.059 ms | 84.947 ms | 161.255 ms | ❌ 2.05× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.185 ms | 0.143 ms | 1.015 ms | ❌ 1.30× |
| 1,000,000,000 | 1,000,000 | generate | 0.262 ms | 0.202 ms | 1.718 ms | ❌ 1.30× |
| 1,000,000,000 | 100,000,000 | count | 19.382 ms | 9.805 ms | 99.028 ms | ❌ 1.98× |
| 1,000,000,000 | 100,000,000 | generate | 43.459 ms | 30.102 ms | 183.444 ms | ❌ 1.44× |
| 1,000,000,000,000 | 1,000,000 | count | 1.328 ms | 1.240 ms | 3.172 ms | ❌ 1.07× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.413 ms | 1.303 ms | 3.730 ms | ❌ 1.08× |
| 1,000,000,000,000 | 100,000,000 | count | 40.113 ms | 21.152 ms | 237.720 ms | ❌ 1.90× |
| 1,000,000,000,000 | 100,000,000 | generate | 58.228 ms | 36.601 ms | 304.259 ms | ❌ 1.59× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.793 ms | 1.476 ms | 2.228 ms | ❌ 1.22× |
| clustered_1e8 | 8,192 | 0.578 ms | 0.944 ms | 7.655 ms | ✅ 0.61× |
| scattered_1e12 | 2,048 | 5.764 ms | 3.495 ms | 5.017 ms | ❌ 1.65× |
| clustered_1e12 | 8,192 | 2.456 ms | 2.243 ms | 16.727 ms | ❌ 1.10× |
| scattered_1e14 | 2,048 | 5.840 ms | 12.795 ms | 8.799 ms | ✅ 0.46× |
| clustered_1e14 | 8,192 | 8.158 ms | 7.811 ms | 28.205 ms | ❌ 1.04× |
| packed_1e14 | 8,192 | 1.753 ms | 4.011 ms | 2.435 ms | ✅ 0.44× |
| mixed_1e12 | 14,336 | 8.051 ms | 7.022 ms | 20.180 ms | ❌ 1.15× |
| spaced_1e14 | 1,024 | 2.899 ms | 40.203 ms | 4.375 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.967 ms | 3.116 s | 4.330 ms | ✅ <0.01× |

