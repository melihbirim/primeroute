## macOS ARM64 · ecc12a2

**primeroute wins against primesieve in 12/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 15.924 ms | 13.011 ms | 19.368 ms | ❌ 1.22× |
| 1,000,000,000 | 91.395 ms | 114.411 ms | 210.785 ms | ✅ 0.80× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.091 ms | 0.132 ms | 0.737 ms | ✅ 0.69× |
| 1,000,000,000 | 1,000,000 | generate | 0.120 ms | 0.145 ms | 0.983 ms | ✅ 0.82× |
| 1,000,000,000 | 100,000,000 | count | 12.149 ms | 9.699 ms | 81.614 ms | ❌ 1.25× |
| 1,000,000,000 | 100,000,000 | generate | 18.449 ms | 14.786 ms | 132.467 ms | ❌ 1.25× |
| 1,000,000,000,000 | 1,000,000 | count | 0.788 ms | 1.055 ms | 2.230 ms | ✅ 0.75× |
| 1,000,000,000,000 | 1,000,000 | generate | 0.826 ms | 1.094 ms | 2.906 ms | ✅ 0.75× |
| 1,000,000,000,000 | 100,000,000 | count | 24.687 ms | 18.166 ms | 186.898 ms | ❌ 1.36× |
| 1,000,000,000,000 | 100,000,000 | generate | 32.605 ms | 21.888 ms | 232.195 ms | ❌ 1.49× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.848 ms | 1.150 ms | 1.447 ms | ✅ 0.74× |
| clustered_1e8 | 8,192 | 0.377 ms | 0.628 ms | 4.960 ms | ✅ 0.60× |
| scattered_1e12 | 2,048 | 3.787 ms | 2.623 ms | 10.270 ms | ❌ 1.44× |
| clustered_1e12 | 8,192 | 1.743 ms | 1.698 ms | 33.519 ms | ≈ 1.03× |
| scattered_1e14 | 2,048 | 7.382 ms | 12.334 ms | 17.796 ms | ✅ 0.60× |
| clustered_1e14 | 8,192 | 7.819 ms | 7.131 ms | 62.740 ms | ❌ 1.10× |
| packed_1e14 | 8,192 | 2.275 ms | 2.962 ms | 5.101 ms | ✅ 0.77× |
| mixed_1e12 | 14,336 | 5.649 ms | 7.816 ms | 54.994 ms | ✅ 0.72× |
| spaced_1e14 | 1,024 | 3.777 ms | 31.847 ms | 9.217 ms | ✅ 0.12× |
| isolated_1e14 | 1,024 | 10.313 ms | 3.024 s | 8.716 ms | ✅ <0.01× |

