## macOS ARM64 · 4daa474

**primeroute wins against primesieve in 13/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 14.438 ms | 13.214 ms | 24.129 ms | ❌ 1.09× |
| 1,000,000,000 | 103.767 ms | 101.859 ms | 229.382 ms | ≈ 1.02× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.081 ms | 0.126 ms | 0.975 ms | ✅ 0.64× |
| 1,000,000,000 | 1,000,000 | generate | 0.316 ms | 0.161 ms | 1.124 ms | ❌ 1.97× |
| 1,000,000,000 | 100,000,000 | count | 9.123 ms | 10.015 ms | 98.652 ms | ✅ 0.91× |
| 1,000,000,000 | 100,000,000 | generate | 18.614 ms | 15.122 ms | 138.812 ms | ❌ 1.23× |
| 1,000,000,000,000 | 1,000,000 | count | 0.835 ms | 1.117 ms | 2.361 ms | ✅ 0.75× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.433 ms | 1.256 ms | 3.071 ms | ❌ 1.14× |
| 1,000,000,000,000 | 100,000,000 | count | 30.601 ms | 21.019 ms | 210.554 ms | ❌ 1.46× |
| 1,000,000,000,000 | 100,000,000 | generate | 32.819 ms | 23.211 ms | 246.443 ms | ❌ 1.41× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.170 ms | 1.295 ms | 1.702 ms | ✅ 0.90× |
| clustered_1e8 | 8,192 | 0.377 ms | 0.661 ms | 10.675 ms | ✅ 0.57× |
| scattered_1e12 | 2,048 | 3.638 ms | 4.004 ms | 12.600 ms | ✅ 0.91× |
| clustered_1e12 | 8,192 | 1.729 ms | 2.090 ms | 45.008 ms | ✅ 0.83× |
| scattered_1e14 | 2,048 | 7.662 ms | 15.232 ms | 22.648 ms | ✅ 0.50× |
| clustered_1e14 | 8,192 | 6.278 ms | 6.974 ms | 61.700 ms | ✅ 0.90× |
| packed_1e14 | 8,192 | 2.330 ms | 3.822 ms | 5.285 ms | ✅ 0.61× |
| mixed_1e12 | 14,336 | 6.627 ms | 9.852 ms | 58.351 ms | ✅ 0.67× |
| spaced_1e14 | 1,024 | 5.403 ms | 46.290 ms | 17.777 ms | ✅ 0.12× |
| isolated_1e14 | 1,024 | 3.440 ms | 2.637 s | 10.645 ms | ✅ <0.01× |

