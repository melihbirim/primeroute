"""Compare fresh inclusive range queries through the Rust and C++ library APIs."""

import argparse
import hashlib
import json
import math
import platform
import statistics
import subprocess
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--cpp", required=True, help="Path to compiled range_bench.cpp")
parser.add_argument("--primesieve", required=True, help="Path to C++ primesieve CLI for CPU metadata")
parser.add_argument("--baseline", help="Optional preserved Rust range helper")
parser.add_argument("--primal", help="Optional primal range helper (direct tests and independent segmented adapter)")
parser.add_argument("--primal-direct", action="store_true", help="Also benchmark primal::is_prime scanning (can be slow)")
parser.add_argument("--cpp-sizes", type=int, nargs="+", default=[1024])
parser.add_argument("--runs", type=int, default=15)
parser.add_argument("--warmups", type=int, default=3)
parser.add_argument("--repeats", type=int, default=3)
parser.add_argument("--starts", type=int, nargs="+", default=[1_000_000_000, 100_000_000_000, 1_000_000_000_000])
parser.add_argument("--widths", type=int, nargs="+", default=[1_000_000, 100_000_000])
parser.add_argument("--output", type=Path, default=ROOT / "benchmarks/range-results.json")
args = parser.parse_args()
if args.runs < 1 or args.warmups < 0 or args.repeats < 1:
    parser.error("runs and repeats must be positive; warmups must be nonnegative")
if any(start < 0 for start in args.starts) or any(width < 1 for width in args.widths):
    parser.error("starts must be nonnegative and widths must be positive")
if any(not 1 <= size <= 8192 for size in args.cpp_sizes):
    parser.error("cpp-sizes must be between 1 and 8192 KiB")

rust = str(ROOT / "target/release/examples/range_bench")
sources = sorted((ROOT / "src").glob("*.rs")) + [
    ROOT / "Cargo.toml", ROOT / "Cargo.lock", ROOT / "examples/range_bench.rs",
    ROOT / "benchmarks/range_bench.cpp", ROOT / "benchmarks/ranges.py",
]
if args.primal:
    sources += [ROOT / p for p in ["benchmarks/primal/Cargo.toml", "benchmarks/primal/Cargo.lock",
                                  "benchmarks/primal/src/bin/range_bench.rs"]]
source_hashes = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
binary_paths = {"ours": rust, "cpp": args.cpp}
if args.baseline:
    binary_paths["ours_baseline"] = args.baseline
if args.primal:
    binary_paths["primal"] = args.primal
binary_hashes = {name: hashlib.sha256(Path(p).read_bytes()).hexdigest() for name, p in binary_paths.items()}
report = {
    "status": "running",
    "expected_cases": len(args.starts) * len(args.widths) * 2,
    "timestamp_utc": datetime.now(timezone.utc).isoformat(),
    "platform": platform.platform(),
    "threads": 1,
    "runs": args.runs,
    "warmup_batches": args.warmups,
    "fresh_queries_per_sample": args.repeats,
    "metric": "In-process milliseconds per fresh query; setup, allocation and result checksum included",
    "cpu_info": subprocess.check_output([args.primesieve, "--cpu-info"], text=True).strip(),
    "primesieve_version": subprocess.check_output([args.primesieve, "--version"], text=True).splitlines()[0],
    "rustc_version": subprocess.check_output(["rustc", "--version"], text=True).strip(),
    "binary_sha256": binary_hashes,
    "source_sha256": source_hashes,
    "primal_comparison": {
        "segmented_adapter": "Independent odd-only range adapter using primal::Sieve::new(sqrt(end)); all setup included; sqrt(end) <= 2^24",
        "direct": "Primal public is_prime test over candidates coprime to 2, 3 and 5; no cached results",
        "native_range_api": False,
    } if args.primal else None,
    "cases": [],
}
def atomic_write(destination, value):
    temporary = destination.with_name(destination.name + ".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n")
    temporary.replace(destination)

progress = args.output.with_name(args.output.stem + ".progress.json")
for start in args.starts:
    for width in args.widths:
        end = start + width - 1
        for mode in ["count", "generate"]:
            common = [str(start), str(end), mode, str(args.repeats)]
            commands = {
                "ours": [rust, *common],
                "cpp_default": [args.cpp, *common, "0"],
            }
            for size in args.cpp_sizes:
                commands[f"cpp_{size}k"] = [args.cpp, *common, str(size)]
            if args.baseline:
                commands["ours_baseline"] = [args.baseline, *common]
            if args.primal:
                if math.isqrt(end) <= 1 << 24:
                    commands["primal_segmented_adapter"] = [args.primal, *common, "segmented"]
                if args.primal_direct:
                    commands["primal_direct"] = [args.primal, *common, "direct"]
            samples = {name: [] for name in commands}
            expected = None
            for round_index in range(args.warmups + args.runs):
                names = list(commands)
                shift = round_index % len(names)
                names = names[shift:] + names[:shift]
                if round_index % 2:
                    names.reverse()
                for name in names:
                    data = json.loads(subprocess.check_output(commands[name], text=True))
                    result = (data["count"], data["checksum"])
                    if expected is None:
                        expected = result
                    if result != expected or data["threads"] != 1:
                        raise RuntimeError(f"{name}: result/thread mismatch for {start}, {end}, {mode}")
                    if round_index >= args.warmups:
                        samples[name].append(data["elapsed_ns"] / data["repeats"] / 1_000_000)
            summaries = {name: {
                "median_ms": statistics.median(times), "mean_ms": statistics.mean(times),
                "min_ms": min(times), "max_ms": max(times), "samples_ms": times,
            } for name, times in samples.items()}
            report["cases"].append({
                "start": start, "end": end, "width": width, "mode": mode,
                "count": expected[0], "checksum": expected[1],
                "commands": commands, "results": summaries,
            })
            print(f"{mode:8s} start={start} width={width}: " + ", ".join(
                f"{name}={summary['median_ms']:.3f} ms" for name, summary in summaries.items()
            ), flush=True)
            atomic_write(progress, report)
for p in sources:
    if hashlib.sha256(p.read_bytes()).hexdigest() != source_hashes[str(p.relative_to(ROOT))]:
        raise RuntimeError(f"Source changed during measurement: {p}; report not published")
for name, p in binary_paths.items():
    if hashlib.sha256(Path(p).read_bytes()).hexdigest() != binary_hashes[name]:
        raise RuntimeError(f"Binary changed during measurement: {name}; report not published")
report["status"] = "complete"
report["completed_at_utc"] = datetime.now(timezone.utc).isoformat()
atomic_write(args.output, report)
progress.unlink(missing_ok=True)
print(f"All counts and generation checksums agree. Saved {args.output}")
