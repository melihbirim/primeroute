#!/usr/bin/env bash
# CI-sized benchmark run against primesieve and primal. One thread everywhere.
# Usage: benchmarks/ci.sh PRIMESIEVE_DIR OUT_DIR [TITLE]
# PRIMESIEVE_DIR must contain include/ and a CMake build/ with
# libprimesieve.a and the primesieve CLI.
set -euo pipefail
PS=$(cd "$1" && pwd)
mkdir -p "$2"
OUT=$(cd "$2" && pwd)
TITLE=${3:-Benchmarks}
cd "$(dirname "$0")/.."

cargo build --release --locked
cargo build --release --locked --example range_bench
cargo build --release --locked --manifest-path benchmarks/primal/Cargo.toml
mkdir -p target/release
"${CXX:-clang++}" -O3 -std=c++14 -I"$PS/include" benchmarks/range_bench.cpp \
  "$PS/build/libprimesieve.a" -pthread -o target/release/range-bench-cpp

for limit in 100000000 1000000000; do
  python3 benchmarks/compare.py --primesieve "$PS/build/primesieve" --limit "$limit" \
    --warmups 1 --runs 5 --output "$OUT/count-$limit.json"
done
python3 benchmarks/ranges.py --cpp target/release/range-bench-cpp \
  --primesieve "$PS/build/primesieve" --primal benchmarks/primal/target/release/range_bench \
  --runs 5 --warmups 1 --starts 1000000000 1000000000000 --widths 1000000 100000000 \
  --output "$OUT/ranges.json"
python3 benchmarks/batch_next.py --primesieve-dir "$PS" --runs 3 --warmups 1 \
  --seed 12648431 --output "$OUT/batch.json"
python3 benchmarks/ci_summary.py "$OUT/count-*.json" "$OUT/ranges.json" "$OUT/batch.json" "$TITLE" \
  > "$OUT/summary.md"
cat "$OUT/summary.md"
