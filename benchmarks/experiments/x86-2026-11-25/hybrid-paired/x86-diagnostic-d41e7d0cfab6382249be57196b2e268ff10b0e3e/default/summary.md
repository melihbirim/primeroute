## x86 default

**primeroute wins against primesieve in 6/20 cases.** Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. All comparators use one thread. Shared CI runners are noisy; treat these as indicative, not as published results.

### Count primes up to N (process time)

| N | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: |
| 100,000,000 | 12.341 ms | 7.508 ms | 10.360 ms | ❌ 1.64× |
| 1,000,000,000 | 118.312 ms | 83.591 ms | 132.367 ms | ❌ 1.42× |

### Ranges (per fresh query)

| Start | Width | Mode | primeroute | primesieve | primal adapter | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1,000,000,000 | 1,000,000 | count | 0.276 ms | 0.180 ms | 0.750 ms | ❌ 1.53× |
| 1,000,000,000 | 1,000,000 | generate | 0.474 ms | 0.238 ms | 1.191 ms | ❌ 1.99× |
| 1,000,000,000 | 100,000,000 | count | 13.547 ms | 8.529 ms | 74.217 ms | ❌ 1.59× |
| 1,000,000,000 | 100,000,000 | generate | 51.479 ms | 29.238 ms | 129.853 ms | ❌ 1.76× |
| 1,000,000,000,000 | 1,000,000 | count | 1.622 ms | 1.117 ms | 2.490 ms | ❌ 1.45× |
| 1,000,000,000,000 | 1,000,000 | generate | 1.838 ms | 1.159 ms | 2.818 ms | ❌ 1.59× |
| 1,000,000,000,000 | 100,000,000 | count | 31.693 ms | 15.419 ms | 181.386 ms | ❌ 2.06× |
| 1,000,000,000,000 | 100,000,000 | generate | 62.498 ms | 30.927 ms | 223.361 ms | ❌ 2.02× |

### Batch next-prime (whole batch)

| Workload | Queries | primeroute | primesieve | primal | vs primesieve |
| --- | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 2,048 | 1.128 ms | 1.372 ms | 1.474 ms | ✅ 0.82× |
| clustered_1e8 | 8,192 | 0.570 ms | 0.845 ms | 5.150 ms | ✅ 0.67× |
| scattered_1e12 | 2,048 | 4.766 ms | 3.084 ms | 3.836 ms | ❌ 1.55× |
| clustered_1e12 | 8,192 | 2.847 ms | 1.946 ms | 13.062 ms | ❌ 1.46× |
| scattered_1e14 | 2,048 | 4.583 ms | 11.476 ms | 7.016 ms | ✅ 0.40× |
| clustered_1e14 | 8,192 | 9.399 ms | 6.422 ms | 22.674 ms | ❌ 1.46× |
| packed_1e14 | 8,192 | 1.456 ms | 3.026 ms | 2.013 ms | ✅ 0.48× |
| mixed_1e12 | 14,336 | 6.956 ms | 5.962 ms | 15.711 ms | ❌ 1.17× |
| spaced_1e14 | 1,024 | 2.286 ms | 34.843 ms | 3.483 ms | ✅ 0.07× |
| isolated_1e14 | 1,024 | 2.314 ms | 2.238 s | 3.515 ms | ✅ <0.01× |

