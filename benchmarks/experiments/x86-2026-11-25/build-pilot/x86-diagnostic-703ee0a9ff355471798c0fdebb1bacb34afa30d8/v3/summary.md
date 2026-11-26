## x86 v3

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 10.260 ms | 6.920 ms | 9.972 ms | ❌ 1.48× |
| 1,000,000,000 | 101.919 ms | 82.891 ms | 131.722 ms | ❌ 1.23× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.215 ms | 0.178 ms | 0.605 ms | ❌ 1.21× |
| 1,000,000,000 | 1,000,000 | generate | 0.446 ms | 0.229 ms | 1.255 ms | ❌ 1.95× |
| 1,000,000,000 | 100,000,000 | count | 11.860 ms | 8.478 ms | 59.051 ms | ❌ 1.40× |
| 1,000,000,000 | 100,000,000 | generate | 49.657 ms | 27.417 ms | 135.076 ms | ❌ 1.81× |
| 1,000,000,000,000 | 1,000,000 | count | 1.488 ms | 1.112 ms | 2.318 ms | ❌ 1.34× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.719 ms | 1.162 ms | 2.939 ms | ❌ 1.48× |
| 1,000,000,000,000 | 100,000,000 | count | 29.616 ms | 15.205 ms | 163.916 ms | ❌ 1.95× |
| 1,000,000,000,000 | 100,000,000 | generate | 61.056 ms | 29.535 ms | 232.199 ms | ❌ 2.07× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.113 ms | 1.371 ms | 1.465 ms | ✅ 0.81× |
| clustered_1e8 | 8,192 | 0.546 ms | 0.820 ms | 5.112 ms | ✅ 0.67× |
| scattered_1e12 | 2,048 | 4.698 ms | 3.052 ms | 3.819 ms | ❌ 1.54× |
| clustered_1e12 | 8,192 | 2.676 ms | 1.903 ms | 12.952 ms | ❌ 1.41× |
| scattered_1e14 | 2,048 | 4.711 ms | 11.282 ms | 6.987 ms | ✅ 0.42× |
| clustered_1e14 | 8,192 | 8.984 ms | 6.293 ms | 22.518 ms | ❌ 1.43× |
| packed_1e14 | 8,192 | 1.483 ms | 2.975 ms | 1.973 ms | ✅ 0.50× |
| mixed_1e12 | 14,336 | 6.747 ms | 5.835 ms | 15.564 ms | ❌ 1.16× |
| spaced_1e14 | 1,024 | 2.343 ms | 34.234 ms | 3.449 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.371 ms | 2.229 s | 3.494 ms | ✅ <0.01× |

