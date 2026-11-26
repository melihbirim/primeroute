## macOS ARM64 · 007333e

**primeroute wins against primesieve in 11/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 13.875 ms | 11.739 ms | 20.915 ms | ❌ 1.18× |
| 1,000,000,000 | 110.597 ms | 117.670 ms | 232.565 ms | ✅ 0.94× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.090 ms | 0.148 ms | 0.677 ms | ✅ 0.60× |
| 1,000,000,000 | 1,000,000 | generate | 0.134 ms | 0.158 ms | 1.122 ms | ✅ 0.85× |
| 1,000,000,000 | 100,000,000 | count | 8.677 ms | 11.397 ms | 70.578 ms | ✅ 0.76× |
| 1,000,000,000 | 100,000,000 | generate | 21.840 ms | 13.408 ms | 127.565 ms | ❌ 1.63× |
| 1,000,000,000,000 | 1,000,000 | count | 0.821 ms | 1.092 ms | 2.335 ms | ✅ 0.75× |
| 1,000,000,000,000 | 1,000,000 | generate | 0.955 ms | 1.110 ms | 2.885 ms | ✅ 0.86× |
| 1,000,000,000,000 | 100,000,000 | count | 28.302 ms | 17.773 ms | 207.495 ms | ❌ 1.59× |
| 1,000,000,000,000 | 100,000,000 | generate | 33.514 ms | 24.371 ms | 284.422 ms | ❌ 1.38× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.229 ms | 1.120 ms | 2.741 ms | ❌ 1.10× |
| clustered_1e8 | 8,192 | 0.367 ms | 0.711 ms | 10.300 ms | ✅ 0.52× |
| scattered_1e12 | 2,048 | 3.242 ms | 2.919 ms | 11.732 ms | ❌ 1.11× |
| clustered_1e12 | 8,192 | 1.819 ms | 1.871 ms | 42.914 ms | ≈ 0.97× |
| scattered_1e14 | 2,048 | 8.602 ms | 14.310 ms | 23.783 ms | ✅ 0.60× |
| clustered_1e14 | 8,192 | 12.864 ms | 10.428 ms | 87.807 ms | ❌ 1.23× |
| packed_1e14 | 8,192 | 1.918 ms | 2.633 ms | 4.562 ms | ✅ 0.73× |
| mixed_1e12 | 14,336 | 6.077 ms | 6.211 ms | 46.355 ms | ≈ 0.98× |
| spaced_1e14 | 1,024 | 3.875 ms | 38.393 ms | 9.456 ms | ✅ 0.10× |
| isolated_1e14 | 1,024 | 4.901 ms | 2.903 s | 17.509 ms | ✅ <0.01× |

