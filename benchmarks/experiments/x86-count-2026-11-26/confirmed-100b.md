# Confirmed 100-billion counts

All rows use one thread. Each row is a separate same-VM comparison; absolute times must not be compared across rows. C++ uses its fastest measured configuration median. Every invocation returns 4,118,054,813.

| Candidate | CPU | Previous Rust (s) | Rust (s) | C++ (s) | Primal (s) | Change vs Rust | Rust / C++ |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| canonical16_32 | INTEL(R) XEON(R) PLATINUM 8573C | 17.860 | 19.309 | 15.188 | 32.420 | +8.1% | 1.271× |
| wide8_outer512 | Intel(R) Xeon(R) 6973P-C | 13.833 | 12.012 | 11.232 | 23.160 | -13.2% | 1.069× |
| compact16 | AMD EPYC 7763 64-Core Processor | 20.045 | 20.476 | 15.470 | 30.906 | +2.2% | 1.324× |
| fill17 | AMD EPYC 9V74 80-Core Processor | 14.509 | 14.822 | 11.285 | 26.142 | +2.2% | 1.313× |
| hybrid32_outer512 | AMD EPYC 7763 64-Core Processor | 18.949 | 17.824 | 15.196 | 30.690 | -5.9% | 1.173× |
| queue8 | Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz | 16.558 | 15.205 | 13.517 | 31.416 | -8.2% | 1.125× |
| queue8_outer512 | AMD EPYC 9V74 80-Core Processor | 13.715 | 11.019 | 10.725 | 24.839 | -19.7% | 1.027× |
| wide8_oldpasses_outer512 | AMD EPYC 9V74 80-Core Processor | 13.718 | 10.754 | 10.695 | 24.959 | -21.6% | 1.005× |
| compact8_outer512 | INTEL(R) XEON(R) PLATINUM 8573C | 18.020 | 17.121 | 15.276 | 32.745 | -5.0% | 1.121× |
| vector48 | AMD EPYC 9V74 80-Core Processor | 13.703 | 13.559 | 10.738 | 24.783 | -1.0% | 1.263× |
| hybrid32_dense256_outer512 | Intel(R) Xeon(R) 6975P-C | 14.384 | 15.332 | 11.638 | 24.655 | +6.6% | 1.317× |
| tail48 | AMD EPYC 9V45 96-Core Processor | 9.734 | 11.590 | 7.632 | 20.890 | +19.1% | 1.519× |
| hybrid32_dense256_outer1024 | AMD EPYC 9V74 80-Core Processor | 13.703 | 13.463 | 10.743 | 24.733 | -1.7% | 1.253× |
