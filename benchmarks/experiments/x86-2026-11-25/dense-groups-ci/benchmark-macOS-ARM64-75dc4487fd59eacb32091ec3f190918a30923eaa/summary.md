## macOS ARM64 · 75dc448

**primeroute wins against primesieve in 13/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 18.861 ms | 14.539 ms | 21.734 ms | ❌ 1.30× |
| 1,000,000,000 | 100.735 ms | 85.508 ms | 197.898 ms | ❌ 1.18× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.085 ms | 0.127 ms | 0.621 ms | ✅ 0.67× |
| 1,000,000,000 | 1,000,000 | generate | 0.169 ms | 0.150 ms | 1.375 ms | ❌ 1.13× |
| 1,000,000,000 | 100,000,000 | count | 9.590 ms | 10.067 ms | 76.524 ms | ✅ 0.95× |
| 1,000,000,000 | 100,000,000 | generate | 15.297 ms | 14.378 ms | 124.959 ms | ❌ 1.06× |
| 1,000,000,000,000 | 1,000,000 | count | 1.117 ms | 1.260 ms | 2.950 ms | ✅ 0.89× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.312 ms | 1.433 ms | 3.361 ms | ✅ 0.92× |
| 1,000,000,000,000 | 100,000,000 | count | 26.165 ms | 17.062 ms | 218.209 ms | ❌ 1.53× |
| 1,000,000,000,000 | 100,000,000 | generate | 42.006 ms | 26.836 ms | 259.069 ms | ❌ 1.57× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.876 ms | 1.255 ms | 1.609 ms | ✅ 0.70× |
| clustered_1e8 | 8,192 | 0.404 ms | 0.697 ms | 5.629 ms | ✅ 0.58× |
| scattered_1e12 | 2,048 | 3.409 ms | 2.841 ms | 10.400 ms | ❌ 1.20× |
| clustered_1e12 | 8,192 | 1.622 ms | 1.791 ms | 34.607 ms | ✅ 0.91× |
| scattered_1e14 | 2,048 | 6.982 ms | 10.944 ms | 19.291 ms | ✅ 0.64× |
| clustered_1e14 | 8,192 | 5.220 ms | 6.942 ms | 60.648 ms | ✅ 0.75× |
| packed_1e14 | 8,192 | 2.066 ms | 3.214 ms | 4.413 ms | ✅ 0.64× |
| mixed_1e12 | 14,336 | 5.559 ms | 5.983 ms | 43.776 ms | ✅ 0.93× |
| spaced_1e14 | 1,024 | 3.671 ms | 38.085 ms | 9.268 ms | ✅ 0.10× |
| isolated_1e14 | 1,024 | 3.544 ms | 2.715 s | 9.311 ms | ✅ <0.01× |

