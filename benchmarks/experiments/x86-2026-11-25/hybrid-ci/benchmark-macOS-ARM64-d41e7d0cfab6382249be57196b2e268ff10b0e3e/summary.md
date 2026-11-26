## macOS ARM64 · d41e7d0

**primeroute wins against primesieve in 15/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 8.013 ms | 7.811 ms | 13.718 ms | ≈ 1.03× |
| 1,000,000,000 | 58.242 ms | 63.970 ms | 147.729 ms | ✅ 0.91× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.077 ms | 0.119 ms | 0.560 ms | ✅ 0.65× |
| 1,000,000,000 | 1,000,000 | generate | 0.121 ms | 0.144 ms | 0.978 ms | ✅ 0.84× |
| 1,000,000,000 | 100,000,000 | count | 6.289 ms | 7.080 ms | 56.566 ms | ✅ 0.89× |
| 1,000,000,000 | 100,000,000 | generate | 13.399 ms | 12.015 ms | 116.186 ms | ❌ 1.12× |
| 1,000,000,000,000 | 1,000,000 | count | 0.845 ms | 1.075 ms | 2.120 ms | ✅ 0.79× |
| 1,000,000,000,000 | 1,000,000 | generate | 0.999 ms | 1.119 ms | 2.801 ms | ✅ 0.89× |
| 1,000,000,000,000 | 100,000,000 | count | 22.869 ms | 15.163 ms | 167.549 ms | ❌ 1.51× |
| 1,000,000,000,000 | 100,000,000 | generate | 27.934 ms | 19.239 ms | 205.732 ms | ❌ 1.45× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.719 ms | 1.051 ms | 1.350 ms | ✅ 0.68× |
| clustered_1e8 | 8,192 | 0.328 ms | 0.609 ms | 4.889 ms | ✅ 0.54× |
| scattered_1e12 | 2,048 | 3.244 ms | 2.475 ms | 9.322 ms | ❌ 1.31× |
| clustered_1e12 | 8,192 | 1.512 ms | 1.598 ms | 31.175 ms | ✅ 0.95× |
| scattered_1e14 | 2,048 | 6.666 ms | 10.078 ms | 16.491 ms | ✅ 0.66× |
| clustered_1e14 | 8,192 | 4.692 ms | 6.037 ms | 53.994 ms | ✅ 0.78× |
| packed_1e14 | 8,192 | 1.737 ms | 2.420 ms | 4.143 ms | ✅ 0.72× |
| mixed_1e12 | 14,336 | 5.036 ms | 5.222 ms | 37.808 ms | ✅ 0.96× |
| spaced_1e14 | 1,024 | 3.571 ms | 30.224 ms | 8.112 ms | ✅ 0.12× |
| isolated_1e14 | 1,024 | 3.360 ms | 1.908 s | 8.209 ms | ✅ <0.01× |

