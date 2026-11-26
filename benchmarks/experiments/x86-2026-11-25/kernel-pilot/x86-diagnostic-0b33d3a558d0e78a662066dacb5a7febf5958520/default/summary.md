## x86 default

**primeroute wins against primesieve in 5/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 19.277 ms | 6.968 ms | 11.546 ms | ❌ 2.77× |
| 1,000,000,000 | 206.268 ms | 84.764 ms | 161.280 ms | ❌ 2.43× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.278 ms | 0.148 ms | 1.020 ms | ❌ 1.88× |
| 1,000,000,000 | 1,000,000 | generate | 0.548 ms | 0.202 ms | 1.723 ms | ❌ 2.71× |
| 1,000,000,000 | 100,000,000 | count | 22.465 ms | 9.829 ms | 99.623 ms | ❌ 2.29× |
| 1,000,000,000 | 100,000,000 | generate | 65.861 ms | 30.164 ms | 183.679 ms | ❌ 2.18× |
| 1,000,000,000,000 | 1,000,000 | count | 1.810 ms | 1.236 ms | 3.181 ms | ❌ 1.46× |
| 1,000,000,000,000 | 1,000,000 | generate | 2.054 ms | 1.315 ms | 3.728 ms | ❌ 1.56× |
| 1,000,000,000,000 | 100,000,000 | count | 43.065 ms | 21.143 ms | 237.876 ms | ❌ 2.04× |
| 1,000,000,000,000 | 100,000,000 | generate | 80.887 ms | 36.668 ms | 303.942 ms | ❌ 2.21× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.774 ms | 1.450 ms | 2.203 ms | ❌ 1.22× |
| clustered_1e8 | 8,192 | 0.559 ms | 0.946 ms | 7.670 ms | ✅ 0.59× |
| scattered_1e12 | 2,048 | 5.877 ms | 3.472 ms | 4.964 ms | ❌ 1.69× |
| clustered_1e12 | 8,192 | 3.068 ms | 2.193 ms | 16.683 ms | ❌ 1.40× |
| scattered_1e14 | 2,048 | 5.824 ms | 12.690 ms | 8.772 ms | ✅ 0.46× |
| clustered_1e14 | 8,192 | 11.313 ms | 7.844 ms | 28.156 ms | ❌ 1.44× |
| packed_1e14 | 8,192 | 1.743 ms | 4.001 ms | 2.400 ms | ✅ 0.44× |
| mixed_1e12 | 14,336 | 8.307 ms | 7.022 ms | 20.159 ms | ❌ 1.18× |
| spaced_1e14 | 1,024 | 2.938 ms | 40.369 ms | 4.390 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.969 ms | 3.114 s | 4.311 ms | ✅ <0.01× |

