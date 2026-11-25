"""Benchmark prebuilt, single-threaded prime counters. No Python dependencies."""

import argparse
import hashlib
import json
import platform
import statistics
import subprocess
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--primesieve", required=True, help="Path to C++ primesieve")
parser.add_argument("--baseline", help="Optional path to the original Rust executable")
parser.add_argument("--cpp-size", type=int, help="Add a C++ comparator with this sieve size in KiB")
parser.add_argument("--skip-primal", action="store_true", help="Compare only against C++ primesieve")
parser.add_argument("--progress", action="store_true", help="Print benchmark progress")
parser.add_argument("--limit", type=int, default=100_000_000)
parser.add_argument("--warmups", type=int, default=5)
parser.add_argument("--runs", type=int, default=30)
parser.add_argument("--output", type=Path, default=ROOT / "benchmarks/results.json")
args = parser.parse_args()
if args.limit < 0 or args.warmups < 0 or args.runs < 1:
    parser.error("limit and warmups must be nonnegative; runs must be positive")
if args.cpp_size is not None and not 1 <= args.cpp_size <= 8192:
    parser.error("cpp-size must be between 1 and 8192 KiB")

commands = {
    "ours": [str(ROOT / "target/release/prime-numbers"), str(args.limit)],
    "primesieve_cpp": [
        args.primesieve, str(args.limit), "--threads=1", "--count=1",
        "--quiet", "--no-status",
    ],
    "primal_rust": [
        str(ROOT / "benchmarks/primal/target/release/primal-benchmark"),
        str(args.limit),
    ],
}
if args.skip_primal:
    commands.pop("primal_rust")
if args.baseline:
    commands["ours_baseline"] = [args.baseline, str(args.limit)]
if args.cpp_size is not None:
    commands["primesieve_cpp_tuned"] = commands["primesieve_cpp"] + [f"--size={args.cpp_size}"]
sources = sorted((ROOT / "src").glob("*.rs")) + [
    ROOT / "Cargo.toml", ROOT / "Cargo.lock", ROOT / "benchmarks/compare.py",
    ROOT / "benchmarks/primal/Cargo.toml", ROOT / "benchmarks/primal/Cargo.lock",
    ROOT / "benchmarks/primal/src/main.rs",
]
source_hashes = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
binary_hashes = {name: hashlib.sha256(Path(cmd[0]).read_bytes()).hexdigest()
                 for name, cmd in commands.items()}
started = datetime.now(timezone.utc).isoformat()
samples = {name: [] for name in commands}
# Known counts validate results; every timed executable still computes its answer.
expected = {
    100_000_000: 5_761_455,
    1_000_000_000: 50_847_534,
    10_000_000_000: 455_052_511,
    100_000_000_000: 4_118_054_813,
}.get(args.limit)
for round_index in range(args.warmups + args.runs):
    # Alternate order so one implementation isn't always measured first.
    names = list(commands)
    shift = round_index % len(names)
    names = names[shift:] + names[:shift]
    if round_index % 2:
        names.reverse()
    for name in names:
        start = time.perf_counter_ns()
        result = subprocess.run(commands[name], capture_output=True, text=True, check=True)
        elapsed_ms = (time.perf_counter_ns() - start) / 1_000_000
        count = int(result.stdout.strip())
        if expected is None:
            expected = count
        if count != expected:
            raise RuntimeError(f"{name}: expected {expected} primes, got {count}")
        if round_index >= args.warmups:
            samples[name].append(elapsed_ms)
    measured = round_index - args.warmups + 1
    if args.progress and round_index + 1 == args.warmups:
        print(f"{args.limit}: completed {args.warmups} warm-up rounds", flush=True)
    if args.progress and measured > 0 and (args.runs <= 10 or measured % 5 == 0 or measured == args.runs):
        print(f"{args.limit}: completed {measured}/{args.runs} measured rounds", flush=True)

report = {
    "status": "complete",
    "started_at_utc": started,
    "completed_at_utc": datetime.now(timezone.utc).isoformat(),
    "timestamp_utc": datetime.now(timezone.utc).isoformat(),
    "platform": platform.platform(),
    "limit_inclusive": args.limit,
    "prime_count": expected,
    "threads": 1,
    "warmups_per_command": args.warmups,
    "runs_per_command": args.runs,
    "cpp_extra_sieve_size_kib": args.cpp_size,
    "metric": "Process wall time in milliseconds, including startup and output capture",
    "commands": commands,
    "binary_sha256": binary_hashes,
    "source_sha256": source_hashes,
    "cpu_info": subprocess.check_output([args.primesieve, "--cpu-info"], text=True).strip(),
    "primesieve_version": subprocess.check_output(
        [args.primesieve, "--version"], text=True
    ).splitlines()[0],
    "rustc_version": subprocess.check_output(["rustc", "--version"], text=True).strip(),
    "results": {},
}
for name, times in samples.items():
    summary = {
        "median_ms": statistics.median(times),
        "mean_ms": statistics.mean(times),
        "min_ms": min(times),
        "max_ms": max(times),
        "stdev_ms": statistics.stdev(times) if len(times) > 1 else 0,
        "samples_ms": times,
    }
    report["results"][name] = summary
    print(f"{name:16s} median {summary['median_ms']:8.3f} ms "
          f"mean {summary['mean_ms']:8.3f} ms "
          f"range {summary['min_ms']:.3f}–{summary['max_ms']:.3f} ms")
for p in sources:
    if hashlib.sha256(p.read_bytes()).hexdigest() != source_hashes[str(p.relative_to(ROOT))]:
        raise RuntimeError(f"Source changed during measurement: {p}; report not published")
for name, cmd in commands.items():
    if hashlib.sha256(Path(cmd[0]).read_bytes()).hexdigest() != binary_hashes[name]:
        raise RuntimeError(f"Binary changed during measurement: {name}; report not published")
temporary = args.output.with_name(args.output.name + ".tmp")
temporary.write_text(json.dumps(report, indent=2) + "\n")
temporary.replace(args.output)
print(f"All counts agree: {expected}. Saved {args.output}")
