## Linux X64 · 75dc448

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 6.181 ms | 5.721 ms | 9.825 ms | ❌ 1.08× |
| 1,000,000,000 | 68.170 ms | 67.405 ms | 129.785 ms | ≈ 1.01× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.142 ms | 0.126 ms | 0.588 ms | ❌ 1.13× |
| 1,000,000,000 | 1,000,000 | generate | 0.200 ms | 0.166 ms | 1.003 ms | ❌ 1.20× |
| 1,000,000,000 | 100,000,000 | count | 10.253 ms | 8.108 ms | 59.970 ms | ❌ 1.26× |
| 1,000,000,000 | 100,000,000 | generate | 30.270 ms | 24.807 ms | 114.999 ms | ❌ 1.22× |
| 1,000,000,000,000 | 1,000,000 | count | 1.063 ms | 0.922 ms | 2.217 ms | ❌ 1.15× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.117 ms | 1.012 ms | 2.544 ms | ❌ 1.10× |
| 1,000,000,000,000 | 100,000,000 | count | 23.847 ms | 12.922 ms | 140.145 ms | ❌ 1.85× |
| 1,000,000,000,000 | 100,000,000 | generate | 38.721 ms | 26.149 ms | 198.250 ms | ❌ 1.48× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 0.853 ms | 1.018 ms | 1.297 ms | ✅ 0.84× |
| clustered_1e8 | 8,192 | 0.528 ms | 0.757 ms | 4.918 ms | ✅ 0.70× |
| scattered_1e12 | 2,048 | 5.402 ms | 2.595 ms | 3.382 ms | ❌ 2.08× |
| clustered_1e12 | 8,192 | 2.020 ms | 1.672 ms | 11.149 ms | ❌ 1.21× |
| scattered_1e14 | 2,048 | 4.053 ms | 9.872 ms | 6.855 ms | ✅ 0.41× |
| clustered_1e14 | 8,192 | 6.431 ms | 5.564 ms | 20.491 ms | ❌ 1.16× |
| packed_1e14 | 8,192 | 1.621 ms | 2.529 ms | 1.740 ms | ✅ 0.64× |
| mixed_1e12 | 14,336 | 6.801 ms | 5.312 ms | 14.568 ms | ❌ 1.28× |
| spaced_1e14 | 1,024 | 2.078 ms | 28.270 ms | 3.135 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.115 ms | 1.941 s | 3.655 ms | ✅ <0.01× |

