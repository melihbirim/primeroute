# All independently confirmed experiments

Times are milliseconds. Each table includes every measured case. C++ uses the fastest configuration median measured for that case; individual samples are never selected. Screening pilots are excluded.

## canonical-confirmation/paired-results.json

INTEL(R) XEON(R) PLATINUM 8573C

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 5 | 0 | 21 |
| primal | 25 | 0 | 1 |
| baseline | 0 | 20 | 6 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 8.658 ms | 8.678 ms | 7.034 ms | 11.821 ms | ❌ |
| count through 1000000000 | 102.274 ms | 108.862 ms | 87.192 ms | 165.204 ms | ❌ |
| count through 10000000000 | 1.375 s | 1.400 s | 1.147 s | 2.261 s | ❌ |
| count through 100000000000 | 17.860 s | 19.309 s | 15.188 s | 32.420 s | ❌ |
| count [1000000000, 1000999999] | 0.185 ms | 0.185 ms | 0.143 ms | 1.031 ms | ❌ |
| generate [1000000000, 1000999999] | 0.269 ms | 0.275 ms | 0.201 ms | 1.768 ms | ❌ |
| count [1000000000, 1099999999] | 12.933 ms | 13.203 ms | 10.052 ms | 102.329 ms | ❌ |
| generate [1000000000, 1099999999] | 39.342 ms | 39.467 ms | 32.312 ms | 188.755 ms | ❌ |
| count [100000000000, 100000999999] | 0.776 ms | 0.779 ms | 0.512 ms | 1.914 ms | ❌ |
| generate [100000000000, 100000999999] | 0.860 ms | 0.871 ms | 0.551 ms | 2.625 ms | ❌ |
| count [100000000000, 100099999999] | 21.648 ms | 23.296 ms | 17.672 ms | 168.713 ms | ❌ |
| generate [100000000000, 100099999999] | 42.856 ms | 44.361 ms | 36.305 ms | 241.004 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.355 ms | 1.370 ms | 1.268 ms | 3.220 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.437 ms | 1.448 ms | 1.328 ms | 3.827 ms | ❌ |
| count [1000000000000, 1000099999999] | 27.495 ms | 34.268 ms | 21.763 ms | 242.586 ms | ❌ |
| generate [1000000000000, 1000099999999] | 46.797 ms | 54.175 ms | 38.949 ms | 309.971 ms | ❌ |
| scattered_1e8 | 1.824 ms | 1.848 ms | 1.464 ms | 2.214 ms | ❌ |
| clustered_1e8 | 0.561 ms | 0.560 ms | 0.946 ms | 7.828 ms | ✅ |
| scattered_1e12 | 5.708 ms | 5.811 ms | 3.541 ms | 5.059 ms | ❌ |
| clustered_1e12 | 2.394 ms | 2.426 ms | 2.218 ms | 16.993 ms | ❌ |
| scattered_1e14 | 5.950 ms | 5.955 ms | 13.061 ms | 8.930 ms | ✅ |
| clustered_1e14 | 8.382 ms | 8.401 ms | 7.969 ms | 28.657 ms | ❌ |
| packed_1e14 | 1.820 ms | 1.799 ms | 4.141 ms | 2.496 ms | ✅ |
| mixed_1e12 | 7.945 ms | 7.971 ms | 7.140 ms | 20.510 ms | ❌ |
| spaced_1e14 | 2.999 ms | 2.955 ms | 41.550 ms | 4.476 ms | ✅ |
| isolated_1e14 | 3.012 ms | 3.027 ms | 3.165 s | 4.464 ms | ✅ |

## compact-confirmation/paired-results.json

Intel(R) Xeon(R) 6973P-C

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 5 | 0 | 21 |
| primal | 25 | 0 | 1 |
| baseline | 11 | 14 | 1 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 6.867 ms | 5.411 ms | 5.204 ms | 8.691 ms | ❌ |
| count through 1000000000 | 77.635 ms | 70.596 ms | 62.871 ms | 119.713 ms | ❌ |
| count through 10000000000 | 1.024 s | 946.681 ms | 841.723 ms | 1.612 s | ❌ |
| count through 100000000000 | 13.833 s | 12.012 s | 11.232 s | 23.160 s | ❌ |
| count [1000000000, 1000999999] | 0.135 ms | 0.135 ms | 0.105 ms | 0.698 ms | ❌ |
| generate [1000000000, 1000999999] | 0.216 ms | 0.211 ms | 0.160 ms | 1.280 ms | ❌ |
| count [1000000000, 1099999999] | 9.729 ms | 8.188 ms | 7.224 ms | 68.204 ms | ❌ |
| generate [1000000000, 1099999999] | 30.211 ms | 26.496 ms | 23.885 ms | 129.590 ms | ❌ |
| count [100000000000, 100000999999] | 0.570 ms | 0.568 ms | 0.365 ms | 1.302 ms | ❌ |
| generate [100000000000, 100000999999] | 0.633 ms | 0.637 ms | 0.400 ms | 1.814 ms | ❌ |
| count [100000000000, 100099999999] | 16.555 ms | 15.392 ms | 13.501 ms | 114.091 ms | ❌ |
| generate [100000000000, 100099999999] | 32.758 ms | 28.983 ms | 26.575 ms | 165.420 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.011 ms | 1.018 ms | 0.898 ms | 2.245 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.077 ms | 1.100 ms | 0.931 ms | 2.640 ms | ❌ |
| count [1000000000000, 1000099999999] | 20.641 ms | 17.258 ms | 15.886 ms | 166.497 ms | ❌ |
| generate [1000000000000, 1000099999999] | 38.590 ms | 31.873 ms | 28.679 ms | 224.404 ms | ❌ |
| scattered_1e8 | 1.321 ms | 1.569 ms | 0.966 ms | 1.623 ms | ❌ |
| clustered_1e8 | 0.405 ms | 0.408 ms | 0.659 ms | 5.656 ms | ✅ |
| scattered_1e12 | 4.120 ms | 4.085 ms | 2.500 ms | 3.636 ms | ❌ |
| clustered_1e12 | 1.828 ms | 1.796 ms | 1.511 ms | 12.235 ms | ❌ |
| scattered_1e14 | 4.261 ms | 4.242 ms | 9.389 ms | 6.419 ms | ✅ |
| clustered_1e14 | 6.238 ms | 6.334 ms | 5.781 ms | 20.629 ms | ❌ |
| packed_1e14 | 1.282 ms | 1.287 ms | 2.919 ms | 1.774 ms | ✅ |
| mixed_1e12 | 6.720 ms | 6.502 ms | 5.951 ms | 17.179 ms | ❌ |
| spaced_1e14 | 2.129 ms | 2.120 ms | 29.622 ms | 3.204 ms | ✅ |
| isolated_1e14 | 2.167 ms | 2.157 ms | 2.294 s | 3.185 ms | ✅ |

## cycle-confirmation/paired-results.json

AMD EPYC 7763 64-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 6 | 0 | 20 |
| primal | 24 | 0 | 2 |
| baseline | 2 | 19 | 5 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 12.258 ms | 12.671 ms | 10.465 ms | 14.123 ms | ❌ |
| count through 1000000000 | 121.035 ms | 129.780 ms | 108.677 ms | 170.217 ms | ❌ |
| count through 10000000000 | 1.538 s | 1.556 s | 1.228 s | 2.237 s | ❌ |
| count through 100000000000 | 20.045 s | 20.476 s | 15.470 s | 30.906 s | ❌ |
| count [1000000000, 1000999999] | 0.280 ms | 0.281 ms | 0.238 ms | 1.023 ms | ❌ |
| generate [1000000000, 1000999999] | 0.460 ms | 0.456 ms | 0.333 ms | 1.434 ms | ❌ |
| count [1000000000, 1099999999] | 15.522 ms | 14.688 ms | 11.221 ms | 96.168 ms | ❌ |
| generate [1000000000, 1099999999] | 58.941 ms | 59.265 ms | 41.240 ms | 154.699 ms | ❌ |
| count [100000000000, 100000999999] | 1.012 ms | 0.983 ms | 0.613 ms | 1.811 ms | ❌ |
| generate [100000000000, 100000999999] | 1.205 ms | 1.193 ms | 0.680 ms | 2.430 ms | ❌ |
| count [100000000000, 100099999999] | 26.331 ms | 24.551 ms | 17.397 ms | 153.741 ms | ❌ |
| generate [100000000000, 100099999999] | 61.301 ms | 59.883 ms | 42.854 ms | 201.240 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.535 ms | 1.545 ms | 1.384 ms | 2.960 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.740 ms | 1.760 ms | 1.412 ms | 3.264 ms | ❌ |
| count [1000000000000, 1000099999999] | 32.882 ms | 34.364 ms | 23.514 ms | 209.424 ms | ❌ |
| generate [1000000000000, 1000099999999] | 65.166 ms | 68.370 ms | 47.766 ms | 251.146 ms | ❌ |
| scattered_1e8 | 1.842 ms | 1.879 ms | 2.030 ms | 1.693 ms | ✅ |
| clustered_1e8 | 0.836 ms | 0.897 ms | 1.169 ms | 6.074 ms | ✅ |
| scattered_1e12 | 6.567 ms | 6.614 ms | 4.925 ms | 4.502 ms | ❌ |
| clustered_1e12 | 3.151 ms | 3.128 ms | 2.674 ms | 15.362 ms | ❌ |
| scattered_1e14 | 5.294 ms | 5.269 ms | 15.481 ms | 8.099 ms | ✅ |
| clustered_1e14 | 10.395 ms | 10.224 ms | 7.795 ms | 26.418 ms | ❌ |
| packed_1e14 | 1.834 ms | 1.805 ms | 3.911 ms | 2.427 ms | ✅ |
| mixed_1e12 | 9.219 ms | 9.091 ms | 8.371 ms | 18.307 ms | ❌ |
| spaced_1e14 | 2.697 ms | 2.679 ms | 53.733 ms | 4.112 ms | ✅ |
| isolated_1e14 | 2.719 ms | 2.684 ms | 2.608 s | 4.094 ms | ✅ |

## first-confirmation/paired-results.json

AMD EPYC 9V74 80-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 6 | 2 | 18 |
| primal | 25 | 0 | 1 |
| baseline | 4 | 18 | 4 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 7.839 ms | 7.672 ms | 6.878 ms | 10.089 ms | ❌ |
| count through 1000000000 | 82.288 ms | 81.128 ms | 83.066 ms | 132.081 ms | ≈ |
| count through 10000000000 | 1.068 s | 1.055 s | 886.536 ms | 1.754 s | ❌ |
| count through 100000000000 | 14.509 s | 14.822 s | 11.285 s | 26.142 s | ❌ |
| count [1000000000, 1000999999] | 0.199 ms | 0.194 ms | 0.185 ms | 0.796 ms | ❌ |
| generate [1000000000, 1000999999] | 0.263 ms | 0.269 ms | 0.241 ms | 1.225 ms | ❌ |
| count [1000000000, 1099999999] | 10.801 ms | 10.674 ms | 8.587 ms | 77.825 ms | ❌ |
| generate [1000000000, 1099999999] | 36.744 ms | 35.477 ms | 29.710 ms | 135.035 ms | ❌ |
| count [100000000000, 100000999999] | 0.705 ms | 0.706 ms | 0.479 ms | 1.486 ms | ❌ |
| generate [100000000000, 100000999999] | 0.867 ms | 0.850 ms | 0.547 ms | 2.178 ms | ❌ |
| count [100000000000, 100099999999] | 17.498 ms | 18.559 ms | 12.428 ms | 134.727 ms | ❌ |
| generate [100000000000, 100099999999] | 38.708 ms | 41.536 ms | 32.469 ms | 181.641 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.260 ms | 1.287 ms | 1.121 ms | 2.545 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.251 ms | 1.259 ms | 1.166 ms | 2.826 ms | ❌ |
| count [1000000000000, 1000099999999] | 22.912 ms | 27.885 ms | 15.911 ms | 190.012 ms | ❌ |
| generate [1000000000000, 1000099999999] | 41.251 ms | 47.604 ms | 32.742 ms | 234.120 ms | ❌ |
| scattered_1e8 | 1.191 ms | 1.152 ms | 1.398 ms | 1.545 ms | ✅ |
| clustered_1e8 | 0.546 ms | 0.541 ms | 0.839 ms | 5.037 ms | ✅ |
| scattered_1e12 | 4.753 ms | 4.688 ms | 3.126 ms | 3.924 ms | ❌ |
| clustered_1e12 | 2.282 ms | 2.238 ms | 2.154 ms | 13.543 ms | ❌ |
| scattered_1e14 | 4.608 ms | 4.613 ms | 11.645 ms | 6.989 ms | ✅ |
| clustered_1e14 | 7.908 ms | 7.807 ms | 6.578 ms | 23.556 ms | ❌ |
| packed_1e14 | 1.470 ms | 1.459 ms | 3.021 ms | 1.996 ms | ✅ |
| mixed_1e12 | 6.869 ms | 6.588 ms | 6.559 ms | 17.336 ms | ≈ |
| spaced_1e14 | 2.331 ms | 2.371 ms | 36.665 ms | 3.506 ms | ✅ |
| isolated_1e14 | 2.531 ms | 2.420 ms | 2.380 s | 3.776 ms | ✅ |

## hybrid-confirmation/paired-results.json

AMD EPYC 7763 64-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 7 | 0 | 19 |
| primal | 23 | 1 | 2 |
| baseline | 4 | 17 | 5 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 10.696 ms | 13.467 ms | 9.198 ms | 13.155 ms | ❌ |
| count through 1000000000 | 115.300 ms | 118.748 ms | 107.137 ms | 168.999 ms | ❌ |
| count through 10000000000 | 1.502 s | 1.497 s | 1.214 s | 2.224 s | ❌ |
| count through 100000000000 | 18.949 s | 17.824 s | 15.196 s | 30.690 s | ❌ |
| count [1000000000, 1000999999] | 0.266 ms | 0.266 ms | 0.222 ms | 1.018 ms | ❌ |
| generate [1000000000, 1000999999] | 0.357 ms | 0.358 ms | 0.307 ms | 1.432 ms | ❌ |
| count [1000000000, 1099999999] | 14.537 ms | 18.133 ms | 11.062 ms | 95.112 ms | ❌ |
| generate [1000000000, 1099999999] | 41.825 ms | 45.626 ms | 34.813 ms | 148.982 ms | ❌ |
| count [100000000000, 100000999999] | 0.968 ms | 0.945 ms | 0.582 ms | 1.771 ms | ❌ |
| generate [100000000000, 100000999999] | 1.077 ms | 1.085 ms | 0.645 ms | 2.335 ms | ❌ |
| count [100000000000, 100099999999] | 23.318 ms | 21.680 ms | 16.968 ms | 152.872 ms | ❌ |
| generate [100000000000, 100099999999] | 45.896 ms | 44.547 ms | 37.106 ms | 195.377 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.506 ms | 1.472 ms | 1.323 ms | 2.900 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.601 ms | 1.575 ms | 1.367 ms | 3.190 ms | ❌ |
| count [1000000000000, 1000099999999] | 28.281 ms | 30.921 ms | 22.483 ms | 206.684 ms | ❌ |
| generate [1000000000000, 1000099999999] | 48.983 ms | 52.462 ms | 41.226 ms | 246.854 ms | ❌ |
| scattered_1e8 | 1.680 ms | 1.705 ms | 1.840 ms | 1.645 ms | ✅ |
| clustered_1e8 | 0.681 ms | 0.670 ms | 0.984 ms | 5.809 ms | ✅ |
| scattered_1e12 | 5.988 ms | 4.968 ms | 4.414 ms | 4.451 ms | ❌ |
| clustered_1e12 | 2.708 ms | 2.686 ms | 2.352 ms | 15.036 ms | ❌ |
| scattered_1e14 | 5.303 ms | 5.278 ms | 14.632 ms | 8.102 ms | ✅ |
| clustered_1e14 | 8.812 ms | 8.729 ms | 7.241 ms | 26.037 ms | ❌ |
| packed_1e14 | 1.701 ms | 1.685 ms | 3.589 ms | 2.318 ms | ✅ |
| mixed_1e12 | 8.364 ms | 7.168 ms | 7.704 ms | 18.070 ms | ✅ |
| spaced_1e14 | 2.649 ms | 2.644 ms | 49.854 ms | 4.037 ms | ✅ |
| isolated_1e14 | 2.683 ms | 2.680 ms | 2.580 s | 4.076 ms | ✅ |

## medium-confirmation/paired-results.json

Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 5 | 0 | 21 |
| primal | 25 | 0 | 1 |
| baseline | 7 | 19 | 0 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 9.097 ms | 8.941 ms | 7.435 ms | 13.424 ms | ❌ |
| count through 1000000000 | 98.483 ms | 90.631 ms | 85.114 ms | 176.232 ms | ❌ |
| count through 10000000000 | 1.284 s | 1.141 s | 1.045 s | 2.295 s | ❌ |
| count through 100000000000 | 16.558 s | 15.205 s | 13.517 s | 31.416 s | ❌ |
| count [1000000000, 1000999999] | 0.193 ms | 0.196 ms | 0.151 ms | 0.969 ms | ❌ |
| generate [1000000000, 1000999999] | 0.291 ms | 0.285 ms | 0.210 ms | 1.596 ms | ❌ |
| count [1000000000, 1099999999] | 12.408 ms | 11.010 ms | 9.807 ms | 94.464 ms | ❌ |
| generate [1000000000, 1099999999] | 37.983 ms | 36.514 ms | 31.084 ms | 171.948 ms | ❌ |
| count [100000000000, 100000999999] | 0.753 ms | 0.757 ms | 0.537 ms | 1.787 ms | ❌ |
| generate [100000000000, 100000999999] | 0.869 ms | 0.865 ms | 0.585 ms | 2.460 ms | ❌ |
| count [100000000000, 100099999999] | 20.405 ms | 18.713 ms | 15.313 ms | 154.958 ms | ❌ |
| generate [100000000000, 100099999999] | 41.840 ms | 40.048 ms | 32.951 ms | 218.845 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.414 ms | 1.406 ms | 1.325 ms | 3.061 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.509 ms | 1.514 ms | 1.399 ms | 3.585 ms | ❌ |
| count [1000000000000, 1000099999999] | 25.522 ms | 25.716 ms | 19.117 ms | 223.703 ms | ❌ |
| generate [1000000000000, 1000099999999] | 45.346 ms | 45.630 ms | 35.516 ms | 284.395 ms | ❌ |
| scattered_1e8 | 1.555 ms | 1.550 ms | 1.409 ms | 1.993 ms | ❌ |
| clustered_1e8 | 0.569 ms | 0.557 ms | 0.913 ms | 6.934 ms | ✅ |
| scattered_1e12 | 5.444 ms | 5.428 ms | 3.450 ms | 4.473 ms | ❌ |
| clustered_1e12 | 2.338 ms | 2.322 ms | 2.240 ms | 15.008 ms | ❌ |
| scattered_1e14 | 5.481 ms | 5.478 ms | 13.374 ms | 7.834 ms | ✅ |
| clustered_1e14 | 8.490 ms | 8.460 ms | 7.775 ms | 25.178 ms | ❌ |
| packed_1e14 | 1.678 ms | 1.678 ms | 4.070 ms | 2.185 ms | ✅ |
| mixed_1e12 | 7.800 ms | 7.754 ms | 7.032 ms | 18.120 ms | ❌ |
| spaced_1e14 | 2.761 ms | 2.759 ms | 40.180 ms | 3.914 ms | ✅ |
| isolated_1e14 | 2.808 ms | 2.802 ms | 3.079 s | 3.904 ms | ✅ |

## medium-window-confirmation/paired-results.json

AMD EPYC 9V74 80-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 8 | 1 | 17 |
| primal | 25 | 0 | 1 |
| baseline | 5 | 19 | 2 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 7.795 ms | 11.072 ms | 6.899 ms | 10.099 ms | ❌ |
| count through 1000000000 | 82.037 ms | 79.934 ms | 83.012 ms | 132.137 ms | ✅ |
| count through 10000000000 | 1.072 s | 916.478 ms | 887.448 ms | 1.764 s | ❌ |
| count through 100000000000 | 13.715 s | 11.019 s | 10.725 s | 24.839 s | ≈ |
| count [1000000000, 1000999999] | 0.186 ms | 0.187 ms | 0.177 ms | 0.769 ms | ❌ |
| generate [1000000000, 1000999999] | 0.258 ms | 0.256 ms | 0.234 ms | 1.186 ms | ❌ |
| count [1000000000, 1099999999] | 10.611 ms | 9.606 ms | 8.478 ms | 74.091 ms | ❌ |
| generate [1000000000, 1099999999] | 33.012 ms | 32.585 ms | 27.500 ms | 128.664 ms | ❌ |
| count [100000000000, 100000999999] | 0.692 ms | 0.693 ms | 0.483 ms | 1.475 ms | ❌ |
| generate [100000000000, 100000999999] | 0.780 ms | 0.778 ms | 0.528 ms | 2.017 ms | ❌ |
| count [100000000000, 100099999999] | 17.387 ms | 16.881 ms | 12.310 ms | 127.328 ms | ❌ |
| generate [100000000000, 100099999999] | 35.616 ms | 35.748 ms | 27.731 ms | 172.371 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.160 ms | 1.159 ms | 1.107 ms | 2.484 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.243 ms | 1.229 ms | 1.163 ms | 2.826 ms | ❌ |
| count [1000000000000, 1000099999999] | 21.552 ms | 21.606 ms | 15.234 ms | 180.544 ms | ❌ |
| generate [1000000000000, 1000099999999] | 38.414 ms | 39.031 ms | 29.619 ms | 222.314 ms | ❌ |
| scattered_1e8 | 1.132 ms | 1.194 ms | 1.367 ms | 1.440 ms | ✅ |
| clustered_1e8 | 0.536 ms | 0.524 ms | 0.821 ms | 5.049 ms | ✅ |
| scattered_1e12 | 4.507 ms | 3.671 ms | 3.045 ms | 3.817 ms | ❌ |
| clustered_1e12 | 2.150 ms | 2.099 ms | 1.906 ms | 12.977 ms | ❌ |
| scattered_1e14 | 4.598 ms | 4.601 ms | 11.346 ms | 6.995 ms | ✅ |
| clustered_1e14 | 7.043 ms | 7.118 ms | 6.332 ms | 22.569 ms | ❌ |
| packed_1e14 | 1.465 ms | 1.452 ms | 2.984 ms | 1.987 ms | ✅ |
| mixed_1e12 | 6.496 ms | 5.552 ms | 5.828 ms | 15.525 ms | ✅ |
| spaced_1e14 | 2.307 ms | 2.296 ms | 34.582 ms | 3.446 ms | ✅ |
| isolated_1e14 | 2.348 ms | 2.329 ms | 2.227 s | 3.510 ms | ✅ |

## mixed-fill-confirmation/paired-results.json

AMD EPYC 9V74 80-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 10 | 2 | 14 |
| primal | 26 | 0 | 0 |
| baseline | 14 | 11 | 1 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 7.893 ms | 6.719 ms | 6.931 ms | 10.155 ms | ✅ |
| count through 1000000000 | 82.178 ms | 73.152 ms | 82.999 ms | 132.128 ms | ✅ |
| count through 10000000000 | 1.067 s | 888.749 ms | 884.665 ms | 1.753 s | ≈ |
| count through 100000000000 | 13.718 s | 10.754 s | 10.695 s | 24.959 s | ≈ |
| count [1000000000, 1000999999] | 0.191 ms | 0.183 ms | 0.177 ms | 0.773 ms | ❌ |
| generate [1000000000, 1000999999] | 0.263 ms | 0.262 ms | 0.237 ms | 1.217 ms | ❌ |
| count [1000000000, 1099999999] | 10.574 ms | 8.150 ms | 8.488 ms | 74.355 ms | ✅ |
| generate [1000000000, 1099999999] | 32.981 ms | 29.896 ms | 27.845 ms | 128.943 ms | ❌ |
| count [100000000000, 100000999999] | 0.693 ms | 0.693 ms | 0.483 ms | 1.484 ms | ❌ |
| generate [100000000000, 100000999999] | 0.780 ms | 0.779 ms | 0.528 ms | 2.008 ms | ❌ |
| count [100000000000, 100099999999] | 17.296 ms | 12.919 ms | 12.273 ms | 127.605 ms | ❌ |
| generate [100000000000, 100099999999] | 35.642 ms | 31.015 ms | 28.404 ms | 173.526 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.166 ms | 1.155 ms | 1.108 ms | 2.490 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.286 ms | 1.285 ms | 1.209 ms | 2.950 ms | ❌ |
| count [1000000000000, 1000099999999] | 21.680 ms | 17.731 ms | 15.267 ms | 180.107 ms | ❌ |
| generate [1000000000000, 1000099999999] | 38.610 ms | 34.020 ms | 29.673 ms | 223.183 ms | ❌ |
| scattered_1e8 | 1.116 ms | 1.200 ms | 1.353 ms | 1.441 ms | ✅ |
| clustered_1e8 | 0.547 ms | 0.530 ms | 0.825 ms | 5.037 ms | ✅ |
| scattered_1e12 | 4.559 ms | 3.683 ms | 3.069 ms | 3.825 ms | ❌ |
| clustered_1e12 | 2.169 ms | 2.132 ms | 1.919 ms | 12.991 ms | ❌ |
| scattered_1e14 | 4.607 ms | 4.594 ms | 11.380 ms | 6.988 ms | ✅ |
| clustered_1e14 | 7.413 ms | 7.277 ms | 6.371 ms | 22.525 ms | ❌ |
| packed_1e14 | 1.484 ms | 1.474 ms | 3.015 ms | 1.995 ms | ✅ |
| mixed_1e12 | 6.545 ms | 5.533 ms | 5.877 ms | 15.617 ms | ✅ |
| spaced_1e14 | 2.314 ms | 2.303 ms | 34.633 ms | 3.462 ms | ✅ |
| isolated_1e14 | 2.346 ms | 2.339 ms | 2.246 s | 3.497 ms | ✅ |

## overlap-confirmation/paired-results.json

INTEL(R) XEON(R) PLATINUM 8573C

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 5 | 0 | 21 |
| primal | 25 | 0 | 1 |
| baseline | 10 | 15 | 1 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 9.179 ms | 7.749 ms | 7.280 ms | 11.997 ms | ❌ |
| count through 1000000000 | 103.563 ms | 102.545 ms | 88.108 ms | 166.751 ms | ❌ |
| count through 10000000000 | 1.388 s | 1.379 s | 1.158 s | 2.286 s | ❌ |
| count through 100000000000 | 18.020 s | 17.121 s | 15.276 s | 32.745 s | ❌ |
| count [1000000000, 1000999999] | 0.193 ms | 0.192 ms | 0.147 ms | 1.059 ms | ❌ |
| generate [1000000000, 1000999999] | 0.278 ms | 0.283 ms | 0.209 ms | 1.789 ms | ❌ |
| count [1000000000, 1099999999] | 13.256 ms | 12.024 ms | 10.209 ms | 103.429 ms | ❌ |
| generate [1000000000, 1099999999] | 42.390 ms | 40.215 ms | 33.973 ms | 191.015 ms | ❌ |
| count [100000000000, 100000999999] | 0.791 ms | 0.791 ms | 0.515 ms | 1.924 ms | ❌ |
| generate [100000000000, 100000999999] | 0.904 ms | 0.903 ms | 0.563 ms | 2.697 ms | ❌ |
| count [100000000000, 100099999999] | 22.345 ms | 19.942 ms | 18.091 ms | 170.177 ms | ❌ |
| generate [100000000000, 100099999999] | 44.327 ms | 41.860 ms | 37.131 ms | 244.602 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.375 ms | 1.381 ms | 1.277 ms | 3.287 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.481 ms | 1.487 ms | 1.338 ms | 3.860 ms | ❌ |
| count [1000000000000, 1000099999999] | 28.032 ms | 25.371 ms | 22.279 ms | 245.541 ms | ❌ |
| generate [1000000000000, 1000099999999] | 48.891 ms | 46.125 ms | 40.996 ms | 315.421 ms | ❌ |
| scattered_1e8 | 1.899 ms | 2.240 ms | 1.502 ms | 2.318 ms | ❌ |
| clustered_1e8 | 0.613 ms | 0.599 ms | 0.973 ms | 7.961 ms | ✅ |
| scattered_1e12 | 6.183 ms | 5.489 ms | 3.712 ms | 5.120 ms | ❌ |
| clustered_1e12 | 2.590 ms | 2.534 ms | 2.322 ms | 17.275 ms | ❌ |
| scattered_1e14 | 6.042 ms | 6.015 ms | 13.673 ms | 8.999 ms | ✅ |
| clustered_1e14 | 8.728 ms | 8.734 ms | 8.179 ms | 29.000 ms | ❌ |
| packed_1e14 | 1.834 ms | 1.843 ms | 4.141 ms | 2.498 ms | ✅ |
| mixed_1e12 | 8.318 ms | 7.646 ms | 7.341 ms | 20.715 ms | ❌ |
| spaced_1e14 | 3.020 ms | 3.015 ms | 42.787 ms | 4.497 ms | ✅ |
| isolated_1e14 | 3.072 ms | 3.069 ms | 3.206 s | 4.446 ms | ✅ |

## statevec-confirmation/paired-results.json

AMD EPYC 9V74 80-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 6 | 1 | 19 |
| primal | 25 | 0 | 1 |
| baseline | 1 | 24 | 1 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 7.917 ms | 8.274 ms | 7.027 ms | 10.170 ms | ❌ |
| count through 1000000000 | 82.221 ms | 82.634 ms | 83.066 ms | 132.109 ms | ≈ |
| count through 10000000000 | 1.070 s | 1.051 s | 885.506 ms | 1.759 s | ❌ |
| count through 100000000000 | 13.703 s | 13.559 s | 10.738 s | 24.783 s | ❌ |
| count [1000000000, 1000999999] | 0.182 ms | 0.182 ms | 0.176 ms | 0.759 ms | ❌ |
| generate [1000000000, 1000999999] | 0.254 ms | 0.259 ms | 0.230 ms | 1.190 ms | ❌ |
| count [1000000000, 1099999999] | 10.635 ms | 10.410 ms | 8.497 ms | 73.448 ms | ❌ |
| generate [1000000000, 1099999999] | 34.707 ms | 34.400 ms | 29.349 ms | 129.894 ms | ❌ |
| count [100000000000, 100000999999] | 0.697 ms | 0.691 ms | 0.479 ms | 1.470 ms | ❌ |
| generate [100000000000, 100000999999] | 0.787 ms | 0.779 ms | 0.528 ms | 2.012 ms | ❌ |
| count [100000000000, 100099999999] | 17.308 ms | 17.184 ms | 12.306 ms | 127.509 ms | ❌ |
| generate [100000000000, 100099999999] | 36.060 ms | 35.998 ms | 28.787 ms | 173.482 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.160 ms | 1.167 ms | 1.110 ms | 2.485 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.247 ms | 1.255 ms | 1.168 ms | 2.850 ms | ❌ |
| count [1000000000000, 1000099999999] | 21.619 ms | 21.549 ms | 15.322 ms | 180.353 ms | ❌ |
| generate [1000000000000, 1000099999999] | 39.264 ms | 38.887 ms | 30.907 ms | 223.400 ms | ❌ |
| scattered_1e8 | 1.125 ms | 1.116 ms | 1.356 ms | 1.440 ms | ✅ |
| clustered_1e8 | 0.552 ms | 0.547 ms | 0.835 ms | 5.049 ms | ✅ |
| scattered_1e12 | 4.536 ms | 4.520 ms | 3.063 ms | 3.831 ms | ❌ |
| clustered_1e12 | 2.182 ms | 2.092 ms | 1.893 ms | 12.997 ms | ❌ |
| scattered_1e14 | 4.605 ms | 4.587 ms | 11.378 ms | 7.001 ms | ✅ |
| clustered_1e14 | 7.073 ms | 6.952 ms | 6.287 ms | 22.615 ms | ❌ |
| packed_1e14 | 1.463 ms | 1.442 ms | 2.965 ms | 1.991 ms | ✅ |
| mixed_1e12 | 6.461 ms | 6.384 ms | 5.813 ms | 15.543 ms | ❌ |
| spaced_1e14 | 2.300 ms | 2.279 ms | 34.230 ms | 3.455 ms | ✅ |
| isolated_1e14 | 2.332 ms | 2.321 ms | 2.229 s | 3.501 ms | ✅ |

## streams-confirmation/paired-results.json

Intel(R) Xeon(R) 6975P-C

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 5 | 1 | 20 |
| primal | 25 | 0 | 1 |
| baseline | 3 | 18 | 5 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 7.429 ms | 7.334 ms | 5.647 ms | 9.354 ms | ❌ |
| count through 1000000000 | 82.420 ms | 93.234 ms | 67.188 ms | 128.415 ms | ❌ |
| count through 10000000000 | 1.108 s | 1.259 s | 885.449 ms | 1.739 s | ❌ |
| count through 100000000000 | 14.384 s | 15.332 s | 11.638 s | 24.655 s | ❌ |
| count [1000000000, 1000999999] | 0.149 ms | 0.143 ms | 0.113 ms | 0.750 ms | ❌ |
| generate [1000000000, 1000999999] | 0.213 ms | 0.209 ms | 0.157 ms | 1.292 ms | ❌ |
| count [1000000000, 1099999999] | 10.339 ms | 11.196 ms | 7.746 ms | 73.222 ms | ❌ |
| generate [1000000000, 1099999999] | 26.274 ms | 26.352 ms | 18.721 ms | 134.570 ms | ❌ |
| count [100000000000, 100000999999] | 0.613 ms | 0.612 ms | 0.390 ms | 1.393 ms | ❌ |
| generate [100000000000, 100000999999] | 0.684 ms | 0.685 ms | 0.424 ms | 1.948 ms | ❌ |
| count [100000000000, 100099999999] | 17.463 ms | 17.306 ms | 13.570 ms | 122.354 ms | ❌ |
| generate [100000000000, 100099999999] | 29.894 ms | 29.343 ms | 22.613 ms | 172.863 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.091 ms | 1.098 ms | 0.972 ms | 2.425 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.149 ms | 1.151 ms | 1.001 ms | 2.843 ms | ❌ |
| count [1000000000000, 1000099999999] | 21.991 ms | 21.623 ms | 16.771 ms | 178.987 ms | ❌ |
| generate [1000000000000, 1000099999999] | 33.365 ms | 32.605 ms | 25.035 ms | 225.880 ms | ❌ |
| scattered_1e8 | 1.398 ms | 1.685 ms | 1.018 ms | 1.743 ms | ❌ |
| clustered_1e8 | 0.423 ms | 0.425 ms | 0.697 ms | 6.031 ms | ✅ |
| scattered_1e12 | 4.338 ms | 4.143 ms | 2.670 ms | 3.932 ms | ❌ |
| clustered_1e12 | 1.753 ms | 1.757 ms | 1.621 ms | 13.252 ms | ❌ |
| scattered_1e14 | 4.573 ms | 4.574 ms | 9.992 ms | 6.916 ms | ✅ |
| clustered_1e14 | 6.128 ms | 6.210 ms | 6.143 ms | 22.266 ms | ≈ |
| packed_1e14 | 1.365 ms | 1.362 ms | 3.120 ms | 1.918 ms | ✅ |
| mixed_1e12 | 5.948 ms | 5.720 ms | 5.380 ms | 15.838 ms | ❌ |
| spaced_1e14 | 2.308 ms | 2.294 ms | 31.229 ms | 3.461 ms | ✅ |
| isolated_1e14 | 2.322 ms | 2.331 ms | 2.460 s | 3.455 ms | ✅ |

## tail-confirmation/paired-results.json

AMD EPYC 9V45 96-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 6 | 2 | 18 |
| primal | 25 | 0 | 1 |
| baseline | 3 | 13 | 10 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 5.895 ms | 6.130 ms | 4.886 ms | 8.429 ms | ❌ |
| count through 1000000000 | 60.212 ms | 65.765 ms | 57.434 ms | 114.122 ms | ❌ |
| count through 10000000000 | 755.201 ms | 882.435 ms | 640.101 ms | 1.461 s | ❌ |
| count through 100000000000 | 9.734 s | 11.590 s | 7.632 s | 20.890 s | ❌ |
| count [1000000000, 1000999999] | 0.126 ms | 0.129 ms | 0.111 ms | 0.510 ms | ❌ |
| generate [1000000000, 1000999999] | 0.178 ms | 0.179 ms | 0.158 ms | 0.904 ms | ❌ |
| count [1000000000, 1099999999] | 7.663 ms | 8.620 ms | 6.502 ms | 48.943 ms | ❌ |
| generate [1000000000, 1099999999] | 25.023 ms | 25.965 ms | 21.041 ms | 94.186 ms | ❌ |
| count [100000000000, 100000999999] | 0.522 ms | 0.514 ms | 0.370 ms | 1.114 ms | ❌ |
| generate [100000000000, 100000999999] | 0.590 ms | 0.583 ms | 0.402 ms | 1.634 ms | ❌ |
| count [100000000000, 100099999999] | 12.663 ms | 14.802 ms | 8.860 ms | 92.570 ms | ❌ |
| generate [100000000000, 100099999999] | 26.565 ms | 28.700 ms | 20.751 ms | 130.018 ms | ❌ |
| count [1000000000000, 1000000999999] | 0.900 ms | 0.893 ms | 0.870 ms | 1.905 ms | ≈ |
| generate [1000000000000, 1000000999999] | 1.039 ms | 0.986 ms | 0.988 ms | 2.368 ms | ≈ |
| count [1000000000000, 1000099999999] | 15.870 ms | 18.065 ms | 11.455 ms | 132.229 ms | ❌ |
| generate [1000000000000, 1000099999999] | 28.669 ms | 30.947 ms | 22.304 ms | 166.894 ms | ❌ |
| scattered_1e8 | 0.808 ms | 0.805 ms | 0.944 ms | 1.233 ms | ✅ |
| clustered_1e8 | 0.445 ms | 0.414 ms | 0.704 ms | 4.377 ms | ✅ |
| scattered_1e12 | 3.501 ms | 3.465 ms | 2.332 ms | 3.161 ms | ❌ |
| clustered_1e12 | 1.873 ms | 1.767 ms | 1.556 ms | 10.588 ms | ❌ |
| scattered_1e14 | 3.653 ms | 3.626 ms | 8.946 ms | 5.564 ms | ✅ |
| clustered_1e14 | 5.705 ms | 5.636 ms | 5.331 ms | 18.355 ms | ❌ |
| packed_1e14 | 1.174 ms | 1.179 ms | 2.451 ms | 1.611 ms | ✅ |
| mixed_1e12 | 5.070 ms | 5.088 ms | 4.841 ms | 13.473 ms | ❌ |
| spaced_1e14 | 1.821 ms | 1.836 ms | 25.440 ms | 2.921 ms | ✅ |
| isolated_1e14 | 1.847 ms | 1.858 ms | 1.756 s | 2.775 ms | ✅ |

## window-confirmation/paired-results.json

AMD EPYC 9V74 80-Core Processor

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 7 | 1 | 18 |
| primal | 26 | 0 | 0 |
| baseline | 8 | 15 | 3 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 8.004 ms | 7.091 ms | 7.056 ms | 10.172 ms | ≈ |
| count through 1000000000 | 82.129 ms | 88.099 ms | 83.086 ms | 132.383 ms | ❌ |
| count through 10000000000 | 1.066 s | 1.155 s | 885.685 ms | 1.755 s | ❌ |
| count through 100000000000 | 13.703 s | 13.463 s | 10.743 s | 24.733 s | ❌ |
| count [1000000000, 1000999999] | 0.187 ms | 0.182 ms | 0.173 ms | 0.755 ms | ❌ |
| generate [1000000000, 1000999999] | 0.258 ms | 0.251 ms | 0.233 ms | 1.193 ms | ❌ |
| count [1000000000, 1099999999] | 10.635 ms | 10.847 ms | 8.489 ms | 74.134 ms | ❌ |
| generate [1000000000, 1099999999] | 33.124 ms | 32.818 ms | 27.576 ms | 128.490 ms | ❌ |
| count [100000000000, 100000999999] | 0.697 ms | 0.693 ms | 0.476 ms | 1.470 ms | ❌ |
| generate [100000000000, 100000999999] | 0.781 ms | 0.779 ms | 0.525 ms | 2.019 ms | ❌ |
| count [100000000000, 100099999999] | 17.309 ms | 15.961 ms | 12.304 ms | 127.323 ms | ❌ |
| generate [100000000000, 100099999999] | 35.283 ms | 33.964 ms | 27.852 ms | 171.576 ms | ❌ |
| count [1000000000000, 1000000999999] | 1.167 ms | 1.168 ms | 1.113 ms | 2.484 ms | ❌ |
| generate [1000000000000, 1000000999999] | 1.246 ms | 1.246 ms | 1.161 ms | 2.820 ms | ❌ |
| count [1000000000000, 1000099999999] | 21.563 ms | 19.752 ms | 15.249 ms | 180.542 ms | ❌ |
| generate [1000000000000, 1000099999999] | 38.202 ms | 36.253 ms | 29.642 ms | 222.534 ms | ❌ |
| scattered_1e8 | 1.127 ms | 1.206 ms | 1.352 ms | 1.458 ms | ✅ |
| clustered_1e8 | 0.553 ms | 0.534 ms | 0.832 ms | 5.141 ms | ✅ |
| scattered_1e12 | 4.506 ms | 3.682 ms | 3.049 ms | 3.805 ms | ❌ |
| clustered_1e12 | 2.151 ms | 2.087 ms | 1.903 ms | 12.970 ms | ❌ |
| scattered_1e14 | 4.589 ms | 4.588 ms | 11.281 ms | 7.002 ms | ✅ |
| clustered_1e14 | 6.986 ms | 6.963 ms | 6.274 ms | 22.571 ms | ❌ |
| packed_1e14 | 1.459 ms | 1.456 ms | 2.964 ms | 1.997 ms | ✅ |
| mixed_1e12 | 6.467 ms | 5.577 ms | 5.809 ms | 15.522 ms | ✅ |
| spaced_1e14 | 2.316 ms | 2.291 ms | 34.134 ms | 3.460 ms | ✅ |
| isolated_1e14 | 2.333 ms | 2.314 ms | 2.228 s | 3.498 ms | ✅ |

## apple-final-paired-results.json

Apple M2 Pro

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 19 | 3 | 4 |
| primal | 26 | 0 | 0 |
| baseline | 2 | 23 | 1 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 7.369 ms | 7.259 ms | 7.095 ms | 12.138 ms | ≈ |
| count through 1000000000 | 53.391 ms | 51.993 ms | 64.767 ms | 132.103 ms | ✅ |
| count through 10000000000 | 634.251 ms | 637.287 ms | 759.503 ms | 1.754 s | ✅ |
| count through 100000000000 | 8.982 s | 8.992 s | 9.730 s | 23.924 s | ✅ |
| count [1000000000, 1000999999] | 0.071 ms | 0.072 ms | 0.097 ms | 0.551 ms | ✅ |
| generate [1000000000, 1000999999] | 0.103 ms | 0.102 ms | 0.114 ms | 0.881 ms | ✅ |
| count [1000000000, 1099999999] | 5.308 ms | 5.296 ms | 6.829 ms | 50.335 ms | ✅ |
| generate [1000000000, 1099999999] | 9.507 ms | 9.530 ms | 10.205 ms | 87.930 ms | ✅ |
| count [100000000000, 100000999999] | 0.239 ms | 0.240 ms | 0.299 ms | 1.024 ms | ✅ |
| generate [100000000000, 100000999999] | 0.269 ms | 0.275 ms | 0.326 ms | 1.347 ms | ✅ |
| count [100000000000, 100099999999] | 10.861 ms | 10.865 ms | 10.653 ms | 92.612 ms | ≈ |
| generate [100000000000, 100099999999] | 14.139 ms | 14.307 ms | 13.545 ms | 125.336 ms | ❌ |
| count [1000000000000, 1000000999999] | 0.642 ms | 0.644 ms | 0.847 ms | 1.841 ms | ✅ |
| generate [1000000000000, 1000000999999] | 0.686 ms | 0.706 ms | 0.870 ms | 2.214 ms | ✅ |
| count [1000000000000, 1000099999999] | 18.273 ms | 18.201 ms | 13.210 ms | 139.726 ms | ❌ |
| generate [1000000000000, 1000099999999] | 21.180 ms | 21.292 ms | 16.008 ms | 171.480 ms | ❌ |
| scattered_1e8 | 0.625 ms | 0.688 ms | 1.040 ms | 1.256 ms | ✅ |
| clustered_1e8 | 0.254 ms | 0.255 ms | 0.519 ms | 4.219 ms | ✅ |
| scattered_1e12 | 2.533 ms | 2.503 ms | 2.129 ms | 7.526 ms | ❌ |
| clustered_1e12 | 1.769 ms | 1.205 ms | 1.345 ms | 27.030 ms | ✅ |
| scattered_1e14 | 9.264 ms | 9.293 ms | 9.553 ms | 14.216 ms | ≈ |
| clustered_1e14 | 3.960 ms | 3.943 ms | 4.804 ms | 42.934 ms | ✅ |
| packed_1e14 | 1.528 ms | 1.481 ms | 2.016 ms | 3.445 ms | ✅ |
| mixed_1e12 | 3.812 ms | 3.807 ms | 4.347 ms | 30.105 ms | ✅ |
| spaced_1e14 | 2.724 ms | 2.722 ms | 26.879 ms | 6.606 ms | ✅ |
| isolated_1e14 | 2.749 ms | 2.755 ms | 1.492 s | 6.672 ms | ✅ |

## apple-paired-results.json

Apple M2 Pro

| Comparator | Wins | Ties | Losses |
| --- | ---: | ---: | ---: |
| cpp | 19 | 1 | 6 |
| primal | 26 | 0 | 0 |
| baseline | 3 | 19 | 4 |

| Case | Previous Rust | Candidate | C++ | Primal | vs C++ |
| --- | ---: | ---: | ---: | ---: | ---: |
| count through 100000000 | 6.597 ms | 6.599 ms | 6.623 ms | 11.076 ms | ≈ |
| count through 1000000000 | 50.397 ms | 50.040 ms | 62.278 ms | 127.721 ms | ✅ |
| count through 10000000000 | 599.232 ms | 603.172 ms | 726.425 ms | 1.672 s | ✅ |
| count through 100000000000 | 8.871 s | 8.781 s | 9.589 s | 23.456 s | ✅ |
| count [1000000000, 1000999999] | 0.072 ms | 0.071 ms | 0.095 ms | 0.555 ms | ✅ |
| generate [1000000000, 1000999999] | 0.103 ms | 0.104 ms | 0.116 ms | 0.885 ms | ✅ |
| count [1000000000, 1099999999] | 5.279 ms | 5.299 ms | 6.671 ms | 50.335 ms | ✅ |
| generate [1000000000, 1099999999] | 9.464 ms | 9.457 ms | 10.044 ms | 87.698 ms | ✅ |
| count [100000000000, 100000999999] | 0.239 ms | 0.248 ms | 0.299 ms | 1.028 ms | ✅ |
| generate [100000000000, 100000999999] | 0.269 ms | 0.281 ms | 0.323 ms | 1.340 ms | ✅ |
| count [100000000000, 100099999999] | 10.850 ms | 10.825 ms | 10.412 ms | 92.569 ms | ❌ |
| generate [100000000000, 100099999999] | 14.127 ms | 14.210 ms | 13.286 ms | 125.222 ms | ❌ |
| count [1000000000000, 1000000999999] | 0.650 ms | 0.653 ms | 0.848 ms | 1.841 ms | ✅ |
| generate [1000000000000, 1000000999999] | 0.671 ms | 0.676 ms | 0.855 ms | 2.138 ms | ✅ |
| count [1000000000000, 1000099999999] | 18.213 ms | 18.171 ms | 12.876 ms | 140.594 ms | ❌ |
| generate [1000000000000, 1000099999999] | 21.266 ms | 21.302 ms | 15.954 ms | 172.862 ms | ❌ |
| scattered_1e8 | 0.630 ms | 0.633 ms | 1.079 ms | 1.274 ms | ✅ |
| clustered_1e8 | 0.266 ms | 0.282 ms | 0.552 ms | 4.402 ms | ✅ |
| scattered_1e12 | 2.702 ms | 2.679 ms | 2.164 ms | 8.005 ms | ❌ |
| clustered_1e12 | 6.270 ms | 5.576 ms | 1.824 ms | 44.821 ms | ❌ |
| scattered_1e14 | 6.656 ms | 6.740 ms | 9.205 ms | 15.594 ms | ✅ |
| clustered_1e14 | 4.621 ms | 5.811 ms | 6.664 ms | 63.919 ms | ✅ |
| packed_1e14 | 2.505 ms | 1.832 ms | 2.260 ms | 4.139 ms | ✅ |
| mixed_1e12 | 4.056 ms | 3.893 ms | 4.522 ms | 31.493 ms | ✅ |
| spaced_1e14 | 2.893 ms | 2.919 ms | 30.313 ms | 7.091 ms | ✅ |
| isolated_1e14 | 2.837 ms | 2.874 ms | 1.535 s | 7.043 ms | ✅ |
