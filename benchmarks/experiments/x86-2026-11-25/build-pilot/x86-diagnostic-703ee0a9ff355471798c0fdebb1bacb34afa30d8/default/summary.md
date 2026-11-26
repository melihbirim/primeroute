## x86 default

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 11.735 ms | 6.810 ms | 9.985 ms | ❌ 1.72× |
| 1,000,000,000 | 116.971 ms | 82.935 ms | 131.965 ms | ❌ 1.41× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.279 ms | 0.178 ms | 0.751 ms | ❌ 1.56× |
| 1,000,000,000 | 1,000,000 | generate | 0.477 ms | 0.230 ms | 1.186 ms | ❌ 2.07× |
| 1,000,000,000 | 100,000,000 | count | 13.324 ms | 8.481 ms | 73.463 ms | ❌ 1.57× |
| 1,000,000,000 | 100,000,000 | generate | 48.988 ms | 27.272 ms | 129.435 ms | ❌ 1.80× |
| 1,000,000,000,000 | 1,000,000 | count | 1.635 ms | 1.115 ms | 2.501 ms | ❌ 1.47× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.833 ms | 1.158 ms | 2.827 ms | ❌ 1.58× |
| 1,000,000,000,000 | 100,000,000 | count | 31.103 ms | 15.192 ms | 180.271 ms | ❌ 2.05× |
| 1,000,000,000,000 | 100,000,000 | generate | 60.259 ms | 29.145 ms | 221.540 ms | ❌ 2.07× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.107 ms | 1.374 ms | 1.472 ms | ✅ 0.81× |
| clustered_1e8 | 8,192 | 0.538 ms | 0.815 ms | 5.125 ms | ✅ 0.66× |
| scattered_1e12 | 2,048 | 4.697 ms | 3.048 ms | 3.832 ms | ❌ 1.54× |
| clustered_1e12 | 8,192 | 2.825 ms | 1.920 ms | 13.000 ms | ❌ 1.47× |
| scattered_1e14 | 2,048 | 4.579 ms | 11.248 ms | 7.006 ms | ✅ 0.41× |
| clustered_1e14 | 8,192 | 9.203 ms | 6.301 ms | 22.639 ms | ❌ 1.46× |
| packed_1e14 | 8,192 | 1.450 ms | 2.985 ms | 1.998 ms | ✅ 0.49× |
| mixed_1e12 | 14,336 | 6.713 ms | 5.827 ms | 15.595 ms | ❌ 1.15× |
| spaced_1e14 | 1,024 | 2.282 ms | 33.963 ms | 3.456 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.312 ms | 2.228 s | 3.499 ms | ✅ <0.01× |

