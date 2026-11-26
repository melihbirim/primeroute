## x86 default

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 17.882 ms | 9.759 ms | 13.301 ms | ❌ 1.83× |
| 1,000,000,000 | 179.846 ms | 107.402 ms | 168.850 ms | ❌ 1.67× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.335 ms | 0.219 ms | 1.006 ms | ❌ 1.53× |
| 1,000,000,000 | 1,000,000 | generate | 0.585 ms | 0.309 ms | 1.438 ms | ❌ 1.89× |
| 1,000,000,000 | 100,000,000 | count | 20.291 ms | 11.055 ms | 95.190 ms | ❌ 1.84× |
| 1,000,000,000 | 100,000,000 | generate | 65.393 ms | 36.167 ms | 149.585 ms | ❌ 1.81× |
| 1,000,000,000,000 | 1,000,000 | count | 1.852 ms | 1.337 ms | 2.919 ms | ❌ 1.39× |
| 1,000,000,000,000 | 1,000,000 | generate | 2.088 ms | 1.385 ms | 3.211 ms | ❌ 1.51× |
| 1,000,000,000,000 | 100,000,000 | count | 43.757 ms | 22.741 ms | 208.654 ms | ❌ 1.92× |
| 1,000,000,000,000 | 100,000,000 | generate | 83.761 ms | 45.408 ms | 251.477 ms | ❌ 1.84× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.732 ms | 1.907 ms | 1.726 ms | ✅ 0.91× |
| clustered_1e8 | 8,192 | 0.711 ms | 1.024 ms | 5.935 ms | ✅ 0.69× |
| scattered_1e12 | 2,048 | 6.411 ms | 4.420 ms | 4.456 ms | ❌ 1.45× |
| clustered_1e12 | 8,192 | 3.406 ms | 2.430 ms | 15.044 ms | ❌ 1.40× |
| scattered_1e14 | 2,048 | 5.240 ms | 14.930 ms | 8.067 ms | ✅ 0.35× |
| clustered_1e14 | 8,192 | 11.669 ms | 7.466 ms | 26.121 ms | ❌ 1.56× |
| packed_1e14 | 8,192 | 1.708 ms | 3.683 ms | 2.340 ms | ✅ 0.46× |
| mixed_1e12 | 14,336 | 9.009 ms | 7.981 ms | 18.131 ms | ❌ 1.13× |
| spaced_1e14 | 1,024 | 2.610 ms | 51.353 ms | 4.061 ms | ✅ 0.05× |
| isolated_1e14 | 1,024 | 2.640 ms | 2.587 s | 4.088 ms | ✅ <0.01× |

