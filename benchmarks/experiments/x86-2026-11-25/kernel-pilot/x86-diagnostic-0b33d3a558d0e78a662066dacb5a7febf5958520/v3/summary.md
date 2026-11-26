## x86 v3

**primeroute wins against primesieve in 5/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 16.675 ms | 6.880 ms | 11.456 ms | ❌ 2.42× |
| 1,000,000,000 | 181.369 ms | 84.988 ms | 161.457 ms | ❌ 2.13× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.240 ms | 0.145 ms | 0.841 ms | ❌ 1.65× |
| 1,000,000,000 | 1,000,000 | generate | 0.525 ms | 0.202 ms | 1.643 ms | ❌ 2.60× |
| 1,000,000,000 | 100,000,000 | count | 20.049 ms | 9.829 ms | 82.143 ms | ❌ 2.04× |
| 1,000,000,000 | 100,000,000 | generate | 66.431 ms | 30.221 ms | 176.781 ms | ❌ 2.20× |
| 1,000,000,000,000 | 1,000,000 | count | 1.714 ms | 1.240 ms | 2.972 ms | ❌ 1.38× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.979 ms | 1.294 ms | 3.631 ms | ❌ 1.53× |
| 1,000,000,000,000 | 100,000,000 | count | 40.683 ms | 21.139 ms | 219.350 ms | ❌ 1.92× |
| 1,000,000,000,000 | 100,000,000 | generate | 79.854 ms | 36.686 ms | 297.871 ms | ❌ 2.18× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.736 ms | 1.448 ms | 2.194 ms | ❌ 1.20× |
| clustered_1e8 | 8,192 | 0.586 ms | 0.954 ms | 7.646 ms | ✅ 0.61× |
| scattered_1e12 | 2,048 | 5.754 ms | 3.453 ms | 4.934 ms | ❌ 1.67× |
| clustered_1e12 | 8,192 | 3.018 ms | 2.221 ms | 16.661 ms | ❌ 1.36× |
| scattered_1e14 | 2,048 | 5.868 ms | 12.739 ms | 8.695 ms | ✅ 0.46× |
| clustered_1e14 | 8,192 | 11.189 ms | 7.819 ms | 28.047 ms | ❌ 1.43× |
| packed_1e14 | 8,192 | 1.776 ms | 4.023 ms | 2.416 ms | ✅ 0.44× |
| mixed_1e12 | 14,336 | 8.205 ms | 7.066 ms | 20.050 ms | ❌ 1.16× |
| spaced_1e14 | 1,024 | 2.950 ms | 40.174 ms | 4.361 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 3.008 ms | 3.111 s | 4.344 ms | ✅ <0.01× |

