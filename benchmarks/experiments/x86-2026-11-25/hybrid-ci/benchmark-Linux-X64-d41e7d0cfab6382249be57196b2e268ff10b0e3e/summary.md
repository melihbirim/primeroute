## Linux X64 · d41e7d0

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 5.633 ms | 4.740 ms | 8.097 ms | ❌ 1.19× |
| 1,000,000,000 | 57.832 ms | 55.567 ms | 110.286 ms | ❌ 1.04× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.131 ms | 0.109 ms | 0.500 ms | ❌ 1.20× |
| 1,000,000,000 | 1,000,000 | generate | 0.175 ms | 0.156 ms | 0.875 ms | ❌ 1.12× |
| 1,000,000,000 | 100,000,000 | count | 7.610 ms | 6.490 ms | 48.823 ms | ❌ 1.17× |
| 1,000,000,000 | 100,000,000 | generate | 24.654 ms | 21.006 ms | 94.003 ms | ❌ 1.17× |
| 1,000,000,000,000 | 1,000,000 | count | 0.894 ms | 0.866 ms | 1.906 ms | ❌ 1.03× |
| 1,000,000,000,000 | 1,000,000 | generate | 0.977 ms | 0.936 ms | 2.229 ms | ❌ 1.04× |
| 1,000,000,000,000 | 100,000,000 | count | 16.206 ms | 11.415 ms | 132.973 ms | ❌ 1.42× |
| 1,000,000,000,000 | 100,000,000 | generate | 28.799 ms | 22.413 ms | 167.873 ms | ❌ 1.28× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.791 ms | 0.943 ms | 1.229 ms | ✅ 0.84× |
| clustered_1e8 | 8,192 | 0.433 ms | 0.696 ms | 4.365 ms | ✅ 0.62× |
| scattered_1e12 | 2,048 | 4.261 ms | 2.344 ms | 3.141 ms | ❌ 1.82× |
| clustered_1e12 | 8,192 | 1.748 ms | 1.534 ms | 10.295 ms | ❌ 1.14× |
| scattered_1e14 | 2,048 | 3.666 ms | 8.759 ms | 5.656 ms | ✅ 0.42× |
| clustered_1e14 | 8,192 | 5.286 ms | 5.247 ms | 18.325 ms | ≈ 1.01× |
| packed_1e14 | 8,192 | 1.154 ms | 2.443 ms | 1.577 ms | ✅ 0.47× |
| mixed_1e12 | 14,336 | 5.670 ms | 4.724 ms | 12.536 ms | ❌ 1.20× |
| spaced_1e14 | 1,024 | 1.866 ms | 25.030 ms | 2.893 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 1.867 ms | 1.767 s | 2.845 ms | ✅ <0.01× |

