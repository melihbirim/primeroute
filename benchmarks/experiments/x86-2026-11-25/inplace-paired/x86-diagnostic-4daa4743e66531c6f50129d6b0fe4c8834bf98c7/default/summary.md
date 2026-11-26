## x86 default

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 15.124 ms | 8.906 ms | 12.937 ms | ❌ 1.70× |
| 1,000,000,000 | 150.342 ms | 107.281 ms | 169.706 ms | ❌ 1.40× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.369 ms | 0.228 ms | 0.994 ms | ❌ 1.62× |
| 1,000,000,000 | 1,000,000 | generate | 0.634 ms | 0.318 ms | 1.545 ms | ❌ 2.00× |
| 1,000,000,000 | 100,000,000 | count | 17.176 ms | 10.969 ms | 95.203 ms | ❌ 1.57× |
| 1,000,000,000 | 100,000,000 | generate | 62.155 ms | 35.833 ms | 165.099 ms | ❌ 1.73× |
| 1,000,000,000,000 | 1,000,000 | count | 2.166 ms | 1.424 ms | 3.204 ms | ❌ 1.52× |
| 1,000,000,000,000 | 1,000,000 | generate | 2.317 ms | 1.506 ms | 3.643 ms | ❌ 1.54× |
| 1,000,000,000,000 | 100,000,000 | count | 40.110 ms | 19.512 ms | 232.870 ms | ❌ 2.06× |
| 1,000,000,000,000 | 100,000,000 | generate | 77.830 ms | 39.156 ms | 285.339 ms | ❌ 1.99× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.459 ms | 1.896 ms | 1.911 ms | ✅ 0.77× |
| clustered_1e8 | 8,192 | 0.711 ms | 1.065 ms | 6.617 ms | ✅ 0.67× |
| scattered_1e12 | 2,048 | 6.046 ms | 4.048 ms | 4.979 ms | ❌ 1.49× |
| clustered_1e12 | 8,192 | 3.688 ms | 2.465 ms | 16.791 ms | ❌ 1.50× |
| scattered_1e14 | 2,048 | 5.921 ms | 14.778 ms | 9.078 ms | ✅ 0.40× |
| clustered_1e14 | 8,192 | 11.873 ms | 8.162 ms | 29.182 ms | ❌ 1.45× |
| packed_1e14 | 8,192 | 1.854 ms | 3.853 ms | 2.574 ms | ✅ 0.48× |
| mixed_1e12 | 14,336 | 8.705 ms | 7.649 ms | 20.066 ms | ❌ 1.14× |
| spaced_1e14 | 1,024 | 2.947 ms | 45.641 ms | 4.496 ms | ✅ 0.06× |
| isolated_1e14 | 1,024 | 2.983 ms | 2.880 s | 4.541 ms | ✅ <0.01× |

