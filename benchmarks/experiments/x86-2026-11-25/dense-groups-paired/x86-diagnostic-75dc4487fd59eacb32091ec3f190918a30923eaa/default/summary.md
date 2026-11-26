## x86 default

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 12.026 ms | 7.181 ms | 10.132 ms | ❌ 1.67× |
| 1,000,000,000 | 117.096 ms | 82.892 ms | 131.901 ms | ❌ 1.41× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.272 ms | 0.176 ms | 0.744 ms | ❌ 1.54× |
| 1,000,000,000 | 1,000,000 | generate | 0.478 ms | 0.231 ms | 1.183 ms | ❌ 2.06× |
| 1,000,000,000 | 100,000,000 | count | 13.312 ms | 8.488 ms | 73.406 ms | ❌ 1.57× |
| 1,000,000,000 | 100,000,000 | generate | 49.318 ms | 27.629 ms | 128.909 ms | ❌ 1.79× |
| 1,000,000,000,000 | 1,000,000 | count | 1.622 ms | 1.111 ms | 2.501 ms | ❌ 1.46× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.832 ms | 1.158 ms | 2.804 ms | ❌ 1.58× |
| 1,000,000,000,000 | 100,000,000 | count | 31.187 ms | 15.242 ms | 181.547 ms | ❌ 2.05× |
| 1,000,000,000,000 | 100,000,000 | generate | 60.733 ms | 29.572 ms | 222.454 ms | ❌ 2.05× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.128 ms | 1.345 ms | 1.468 ms | ✅ 0.84× |
| clustered_1e8 | 8,192 | 0.533 ms | 0.836 ms | 5.162 ms | ✅ 0.64× |
| scattered_1e12 | 2,048 | 4.708 ms | 3.052 ms | 3.833 ms | ❌ 1.54× |
| clustered_1e12 | 8,192 | 2.860 ms | 1.931 ms | 12.995 ms | ❌ 1.48× |
| scattered_1e14 | 2,048 | 4.590 ms | 11.402 ms | 7.032 ms | ✅ 0.40× |
| clustered_1e14 | 8,192 | 9.261 ms | 6.316 ms | 22.654 ms | ❌ 1.47× |
| packed_1e14 | 8,192 | 1.434 ms | 2.987 ms | 1.997 ms | ✅ 0.48× |
| mixed_1e12 | 14,336 | 6.787 ms | 5.841 ms | 15.588 ms | ❌ 1.16× |
| spaced_1e14 | 1,024 | 2.295 ms | 34.456 ms | 3.455 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.310 ms | 2.232 s | 3.510 ms | ✅ <0.01× |

