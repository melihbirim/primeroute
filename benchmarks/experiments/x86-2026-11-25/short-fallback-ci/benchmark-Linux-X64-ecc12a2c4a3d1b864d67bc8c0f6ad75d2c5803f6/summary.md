## Linux X64 · ecc12a2

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 7.982 ms | 7.299 ms | 10.197 ms | ❌ 1.09× |
| 1,000,000,000 | 82.507 ms | 83.048 ms | 131.793 ms | ≈ 0.99× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.178 ms | 0.173 ms | 0.774 ms | ≈ 1.03× |
| 1,000,000,000 | 1,000,000 | generate | 0.253 ms | 0.233 ms | 1.209 ms | ❌ 1.09× |
| 1,000,000,000 | 100,000,000 | count | 10.710 ms | 8.515 ms | 74.356 ms | ❌ 1.26× |
| 1,000,000,000 | 100,000,000 | generate | 33.336 ms | 27.800 ms | 128.351 ms | ❌ 1.20× |
| 1,000,000,000,000 | 1,000,000 | count | 1.164 ms | 1.108 ms | 2.499 ms | ❌ 1.05× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.242 ms | 1.164 ms | 2.840 ms | ❌ 1.07× |
| 1,000,000,000,000 | 100,000,000 | count | 21.651 ms | 15.240 ms | 183.835 ms | ❌ 1.42× |
| 1,000,000,000,000 | 100,000,000 | generate | 38.117 ms | 29.752 ms | 222.089 ms | ❌ 1.28× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.188 ms | 1.357 ms | 1.441 ms | ✅ 0.88× |
| clustered_1e8 | 8,192 | 0.553 ms | 0.819 ms | 5.037 ms | ✅ 0.68× |
| scattered_1e12 | 2,048 | 5.823 ms | 3.074 ms | 3.816 ms | ❌ 1.89× |
| clustered_1e12 | 8,192 | 2.358 ms | 1.895 ms | 12.939 ms | ❌ 1.24× |
| scattered_1e14 | 2,048 | 4.588 ms | 11.383 ms | 6.978 ms | ✅ 0.40× |
| clustered_1e14 | 8,192 | 7.274 ms | 6.392 ms | 22.576 ms | ❌ 1.14× |
| packed_1e14 | 8,192 | 1.445 ms | 2.999 ms | 2.000 ms | ✅ 0.48× |
| mixed_1e12 | 14,336 | 7.813 ms | 5.848 ms | 15.593 ms | ❌ 1.34× |
| spaced_1e14 | 1,024 | 2.296 ms | 34.513 ms | 3.457 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.345 ms | 2.232 s | 3.493 ms | ✅ <0.01× |

