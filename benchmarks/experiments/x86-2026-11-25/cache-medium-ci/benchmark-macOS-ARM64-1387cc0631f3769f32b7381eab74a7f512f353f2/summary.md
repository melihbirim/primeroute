## macOS ARM64 · 1387cc0

**primeroute wins against primesieve in 14/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 11.858 ms | 10.651 ms | 14.410 ms | ❌ 1.11× |
| 1,000,000,000 | 72.565 ms | 81.766 ms | 200.798 ms | ✅ 0.89× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.080 ms | 0.135 ms | 0.598 ms | ✅ 0.59× |
| 1,000,000,000 | 1,000,000 | generate | 0.128 ms | 0.146 ms | 1.118 ms | ✅ 0.88× |
| 1,000,000,000 | 100,000,000 | count | 9.207 ms | 8.113 ms | 64.686 ms | ❌ 1.13× |
| 1,000,000,000 | 100,000,000 | generate | 17.210 ms | 13.244 ms | 126.222 ms | ❌ 1.30× |
| 1,000,000,000,000 | 1,000,000 | count | 0.811 ms | 1.086 ms | 2.633 ms | ✅ 0.75× |
| 1,000,000,000,000 | 1,000,000 | generate | 0.917 ms | 1.075 ms | 3.037 ms | ✅ 0.85× |
| 1,000,000,000,000 | 100,000,000 | count | 23.837 ms | 15.884 ms | 187.841 ms | ❌ 1.50× |
| 1,000,000,000,000 | 100,000,000 | generate | 36.975 ms | 31.278 ms | 252.720 ms | ❌ 1.18× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.825 ms | 1.188 ms | 1.499 ms | ✅ 0.69× |
| clustered_1e8 | 8,192 | 0.373 ms | 0.629 ms | 6.407 ms | ✅ 0.59× |
| scattered_1e12 | 2,048 | 3.561 ms | 2.601 ms | 12.667 ms | ❌ 1.37× |
| clustered_1e12 | 8,192 | 1.756 ms | 2.864 ms | 45.294 ms | ✅ 0.61× |
| scattered_1e14 | 2,048 | 9.093 ms | 14.841 ms | 23.014 ms | ✅ 0.61× |
| clustered_1e14 | 8,192 | 5.200 ms | 6.053 ms | 56.809 ms | ✅ 0.86× |
| packed_1e14 | 8,192 | 1.819 ms | 2.783 ms | 4.272 ms | ✅ 0.65× |
| mixed_1e12 | 14,336 | 4.902 ms | 5.497 ms | 53.616 ms | ✅ 0.89× |
| spaced_1e14 | 1,024 | 3.415 ms | 37.898 ms | 8.284 ms | ✅ 0.09× |
| isolated_1e14 | 1,024 | 3.152 ms | 2.077 s | 8.466 ms | ✅ <0.01× |

