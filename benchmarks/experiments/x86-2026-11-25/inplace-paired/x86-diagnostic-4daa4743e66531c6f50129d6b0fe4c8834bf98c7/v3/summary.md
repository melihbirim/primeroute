## x86 v3

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 13.069 ms | 8.862 ms | 12.774 ms | ❌ 1.47× |
| 1,000,000,000 | 131.403 ms | 107.006 ms | 168.982 ms | ❌ 1.23× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.286 ms | 0.229 ms | 0.776 ms | ❌ 1.25× |
| 1,000,000,000 | 1,000,000 | generate | 0.579 ms | 0.316 ms | 1.671 ms | ❌ 1.83× |
| 1,000,000,000 | 100,000,000 | count | 15.271 ms | 10.994 ms | 75.884 ms | ❌ 1.39× |
| 1,000,000,000 | 100,000,000 | generate | 63.374 ms | 36.065 ms | 172.469 ms | ❌ 1.76× |
| 1,000,000,000,000 | 1,000,000 | count | 1.902 ms | 1.427 ms | 2.963 ms | ❌ 1.33× |
| 1,000,000,000,000 | 1,000,000 | generate | 2.199 ms | 1.521 ms | 3.756 ms | ❌ 1.45× |
| 1,000,000,000,000 | 100,000,000 | count | 38.193 ms | 19.560 ms | 210.770 ms | ❌ 1.95× |
| 1,000,000,000,000 | 100,000,000 | generate | 78.132 ms | 39.186 ms | 298.307 ms | ❌ 1.99× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.441 ms | 1.897 ms | 1.881 ms | ✅ 0.76× |
| clustered_1e8 | 8,192 | 0.676 ms | 1.071 ms | 6.562 ms | ✅ 0.63× |
| scattered_1e12 | 2,048 | 6.002 ms | 4.037 ms | 4.929 ms | ❌ 1.49× |
| clustered_1e12 | 8,192 | 3.423 ms | 2.466 ms | 16.656 ms | ❌ 1.39× |
| scattered_1e14 | 2,048 | 6.075 ms | 14.658 ms | 9.000 ms | ✅ 0.41× |
| clustered_1e14 | 8,192 | 11.479 ms | 8.127 ms | 29.014 ms | ❌ 1.41× |
| packed_1e14 | 8,192 | 1.891 ms | 3.848 ms | 2.531 ms | ✅ 0.49× |
| mixed_1e12 | 14,336 | 8.605 ms | 7.606 ms | 19.968 ms | ❌ 1.13× |
| spaced_1e14 | 1,024 | 3.024 ms | 45.223 ms | 4.441 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 3.218 ms | 2.889 s | 4.515 ms | ✅ <0.01× |

