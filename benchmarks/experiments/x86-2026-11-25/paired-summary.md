# Full paired results

Every case is included. Win/tie/loss uses full-precision medians and the same 3% band as CI. C++ and primal columns use the faster measured configuration median; individual samples are never selected.

## Linux x86

AMD EPYC 9V74 80-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| primesieve | 6 | 1 | 19 |
| primal | 25 | 0 | 1 |
| Previous Rust | 21 | 5 | 0 |

### prefix

| Case | Baseline | Baseline v3 | Candidate | C++ | Primal | Change vs baseline | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 15.273 ms | 13.250 ms | 10.030 ms | 8.940 ms | 12.970 ms | -34.3% | ❌ 1.12× |
| count through 1000000000 | 150.495 ms | 131.387 ms | 105.406 ms | 107.397 ms | 169.342 ms | -30.0% | ≈ 0.98× |
| count through 10000000000 | 1.766 s | 1.566 s | 1.375 s | 1.147 s | 2.256 s | -22.2% | ❌ 1.20× |
| count through 100000000000 | 22.883 s | — | 17.668 s | 13.887 s | 31.934 s | -22.8% | ❌ 1.27× |

### range

| Case | Baseline | Baseline v3 | Candidate | C++ | Primal | Change vs baseline | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| count [1000000000, 1000999999] | 0.359 ms | 0.270 ms | 0.237 ms | 0.229 ms | 0.789 ms | -34.0% | ❌ 1.04× |
| generate [1000000000, 1000999999] | 0.619 ms | 0.574 ms | 0.333 ms | 0.317 ms | 1.515 ms | -46.2% | ❌ 1.05× |
| count [1000000000, 1099999999] | 17.179 ms | 15.315 ms | 13.663 ms | 10.997 ms | 76.967 ms | -20.5% | ❌ 1.24× |
| generate [1000000000, 1099999999] | 62.467 ms | 63.564 ms | 41.734 ms | 36.380 ms | 164.586 ms | -33.2% | ❌ 1.15× |
| count [100000000000, 100000999999] | 1.566 ms | 1.277 ms | 0.896 ms | 0.617 ms | 1.674 ms | -42.8% | ❌ 1.45× |
| generate [100000000000, 100000999999] | 1.834 ms | 1.589 ms | 0.997 ms | 0.689 ms | 2.595 ms | -45.6% | ❌ 1.45× |
| count [100000000000, 100099999999] | 27.867 ms | 25.788 ms | 22.274 ms | 15.766 ms | 144.137 ms | -20.1% | ❌ 1.41× |
| generate [100000000000, 100099999999] | 67.782 ms | 68.654 ms | 45.227 ms | 37.184 ms | 220.835 ms | -33.3% | ❌ 1.22× |
| count [1000000000000, 1000000999999] | 2.107 ms | 1.899 ms | 1.493 ms | 1.432 ms | 2.944 ms | -29.1% | ❌ 1.04× |
| generate [1000000000000, 1000000999999] | 2.398 ms | 2.193 ms | 1.599 ms | 1.515 ms | 3.652 ms | -33.3% | ❌ 1.06× |
| count [1000000000000, 1000099999999] | 40.166 ms | 38.198 ms | 27.764 ms | 19.566 ms | 211.103 ms | -30.9% | ❌ 1.42× |
| generate [1000000000000, 1000099999999] | 78.093 ms | 78.644 ms | 49.000 ms | 39.635 ms | 285.057 ms | -37.3% | ❌ 1.24× |

### batch

| Case | Baseline | Baseline v3 | Candidate | C++ | Primal | Change vs baseline | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 1.449 ms | 1.428 ms | 1.443 ms | 1.887 ms | 1.867 ms | -0.5% | ✅ 0.76× |
| clustered_1e8 | 0.716 ms | 0.687 ms | 0.687 ms | 1.064 ms | 6.508 ms | -4.0% | ✅ 0.65× |
| scattered_1e12 | 6.042 ms | 5.990 ms | 5.780 ms | 4.028 ms | 4.900 ms | -4.3% | ❌ 1.43× |
| clustered_1e12 | 3.634 ms | 3.376 ms | 2.776 ms | 2.472 ms | 16.701 ms | -23.6% | ❌ 1.12× |
| scattered_1e14 | 5.898 ms | 6.062 ms | 5.930 ms | 14.697 ms | 8.996 ms | +0.5% | ✅ 0.40× |
| clustered_1e14 | 11.733 ms | 11.498 ms | 8.985 ms | 8.136 ms | 28.992 ms | -23.4% | ❌ 1.10× |
| packed_1e14 | 1.878 ms | 1.893 ms | 1.882 ms | 3.856 ms | 2.536 ms | +0.2% | ✅ 0.49× |
| mixed_1e12 | 8.654 ms | 8.607 ms | 8.138 ms | 7.598 ms | 19.915 ms | -6.0% | ❌ 1.07× |
| spaced_1e14 | 2.962 ms | 3.030 ms | 2.991 ms | 45.766 ms | 4.473 ms | +1.0% | ✅ 0.07× |
| isolated_1e14 | 2.973 ms | 3.050 ms | 3.003 ms | 2.866 s | 4.494 ms | +1.0% | ✅ <0.01× |

## Apple Silicon

Apple M2 Pro

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| primesieve | 20 | 2 | 4 |
| primal | 26 | 0 | 0 |
| Previous Rust | 4 | 19 | 3 |

### prefix

| Case | Baseline | Baseline v3 | Candidate | C++ | Primal | Change vs baseline | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 7.844 ms | — | 7.669 ms | 7.318 ms | 12.410 ms | -2.2% | ❌ 1.05× |
| count through 1000000000 | 53.318 ms | — | 53.286 ms | 64.491 ms | 131.808 ms | -0.1% | ✅ 0.83× |
| count through 10000000000 | 643.808 ms | — | 653.555 ms | 782.260 ms | 1.799 s | +1.5% | ✅ 0.84× |
| count through 100000000000 | 9.385 s | — | 9.375 s | 10.369 s | 24.794 s | -0.1% | ✅ 0.90× |

### range

| Case | Baseline | Baseline v3 | Candidate | C++ | Primal | Change vs baseline | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| count [1000000000, 1000999999] | 0.070 ms | — | 0.073 ms | 0.097 ms | 0.554 ms | +5.3% | ✅ 0.76× |
| generate [1000000000, 1000999999] | 0.112 ms | — | 0.113 ms | 0.123 ms | 0.947 ms | +0.7% | ✅ 0.92× |
| count [1000000000, 1099999999] | 5.557 ms | — | 5.612 ms | 7.137 ms | 53.268 ms | +1.0% | ✅ 0.79× |
| generate [1000000000, 1099999999] | 10.854 ms | — | 11.071 ms | 11.823 ms | 95.291 ms | +2.0% | ✅ 0.94× |
| count [100000000000, 100000999999] | 0.252 ms | — | 0.262 ms | 0.324 ms | 1.087 ms | +3.7% | ✅ 0.81× |
| generate [100000000000, 100000999999] | 0.289 ms | — | 0.297 ms | 0.348 ms | 1.443 ms | +2.7% | ✅ 0.85× |
| count [100000000000, 100099999999] | 11.515 ms | — | 11.481 ms | 11.547 ms | 98.398 ms | -0.3% | ≈ 0.99× |
| generate [100000000000, 100099999999] | 15.493 ms | — | 15.668 ms | 15.577 ms | 133.370 ms | +1.1% | ≈ 1.01× |
| count [1000000000000, 1000000999999] | 0.694 ms | — | 0.683 ms | 0.915 ms | 1.966 ms | -1.5% | ✅ 0.75× |
| generate [1000000000000, 1000000999999] | 0.728 ms | — | 0.726 ms | 0.910 ms | 2.321 ms | -0.3% | ✅ 0.80× |
| count [1000000000000, 1000099999999] | 19.413 ms | — | 19.450 ms | 14.564 ms | 151.247 ms | +0.2% | ❌ 1.34× |
| generate [1000000000000, 1000099999999] | 23.066 ms | — | 23.086 ms | 18.056 ms | 181.420 ms | +0.1% | ❌ 1.28× |

### batch

| Case | Baseline | Baseline v3 | Candidate | C++ | Primal | Change vs baseline | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| scattered_1e8 | 0.691 ms | — | 0.654 ms | 1.040 ms | 1.239 ms | -5.4% | ✅ 0.63× |
| clustered_1e8 | 0.328 ms | — | 0.286 ms | 0.552 ms | 4.381 ms | -12.8% | ✅ 0.52× |
| scattered_1e12 | 2.849 ms | — | 2.706 ms | 2.301 ms | 7.977 ms | -5.0% | ❌ 1.18× |
| clustered_1e12 | 1.412 ms | — | 1.291 ms | 1.420 ms | 27.061 ms | -8.6% | ✅ 0.91× |
| scattered_1e14 | 5.859 ms | — | 5.838 ms | 8.879 ms | 14.191 ms | -0.4% | ✅ 0.66× |
| clustered_1e14 | 4.411 ms | — | 4.507 ms | 5.050 ms | 47.844 ms | +2.2% | ✅ 0.89× |
| packed_1e14 | 1.546 ms | — | 1.596 ms | 2.077 ms | 3.704 ms | +3.3% | ✅ 0.77× |
| mixed_1e12 | 4.204 ms | — | 4.194 ms | 4.606 ms | 32.100 ms | -0.2% | ✅ 0.91× |
| spaced_1e14 | 2.909 ms | — | 2.845 ms | 28.886 ms | 6.935 ms | -2.2% | ✅ 0.10× |
| isolated_1e14 | 2.951 ms | — | 2.945 ms | 1.547 s | 7.066 ms | -0.2% | ✅ <0.01× |
