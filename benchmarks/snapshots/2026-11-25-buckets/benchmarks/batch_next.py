"""Benchmark strict next-prime batches. Builds frozen inputs and checks every answer."""

import argparse
import hashlib
import json
import os
import statistics
import subprocess
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--primesieve-dir", type=Path, required=True)
parser.add_argument("--runs", type=int, default=5)
parser.add_argument("--warmups", type=int, default=1)
parser.add_argument("--seed", type=int, default=20261124)
parser.add_argument("--only", help="Comma-separated dataset names")
parser.add_argument("--skip-isolated", action="store_true")
parser.add_argument("--skip-cpp", action="store_true", help="For router calibration only")
parser.add_argument("--include-sieve", action="store_true", help="Include a forced-sieve control")
parser.add_argument("--output", type=Path, default=ROOT / "benchmarks/batch-next-results.json")
args = parser.parse_args()
if args.runs < 1 or args.warmups < 0:
    parser.error("runs must be positive and warmups nonnegative")

def hashes(paths):
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}

sources = sorted((ROOT / "src").glob("*.rs")) + [
    ROOT / "Cargo.toml", ROOT / "benchmarks/primal/Cargo.toml",
    ROOT / "benchmarks/primal/Cargo.lock", ROOT / "benchmarks/primal/src/bin/batch_next.rs",
    ROOT / "benchmarks/batch_next.cpp", ROOT / "benchmarks/batch_next.py",
]
source_hashes = hashes(sources)
rust = ROOT / "benchmarks/primal/target/release/batch_next"
cpp = ROOT / "target/release/batch-next-cpp"
library = args.primesieve_dir / "build/libprimesieve.a"
cli = args.primesieve_dir / "build/primesieve"
build_commands = [
    ["cargo", "build", "--release", "--locked", "--offline", "--manifest-path",
     str(ROOT / "benchmarks/primal/Cargo.toml"), "--bin", "batch_next"],
    ["clang++", "-O3", "-std=c++14", "-I" + str(args.primesieve_dir / "include"),
     str(ROOT / "benchmarks/batch_next.cpp"), str(library), "-pthread", "-o", str(cpp)],
]
for command in build_commands:
    subprocess.run(command, check=True, cwd=ROOT)
if hashes(sources) != source_hashes:
    raise RuntimeError("Sources changed during the build")

MASK = (1 << 64) - 1
class Random:
    def __init__(self, seed): self.state = seed & MASK
    def next(self):
        self.state = (self.state + 0x9e3779b97f4a7c15) & MASK
        z = self.state
        z = ((z ^ (z >> 30)) * 0xbf58476d1ce4e5b9) & MASK
        z = ((z ^ (z >> 27)) * 0x94d049bb133111eb) & MASK
        return z ^ (z >> 31)
    def shuffle(self, values):
        for i in range(len(values) - 1, 0, -1):
            j = self.next() % (i + 1)
            values[i], values[j] = values[j], values[i]

random = Random(args.seed)
def queries(base, width, count):
    values = set()
    while len(values) < count: values.add(base + random.next() % width)
    values = sorted(values)
    random.shuffle(values)
    return values

datasets = []
for exponent in [8, 12, 14]:
    base = 10 ** exponent + 5_000_000
    datasets.append((f"scattered_1e{exponent}", queries(base, 10_000_000, 2_048)))
    datasets.append((f"clustered_1e{exponent}", queries(base, 1_000_000, 8_192)))
datasets.append(("packed_1e14", queries(10**14 + 5_000_000, 16_384, 8_192)))
mixed = []
for i in range(3): mixed.extend(queries(10**12 + i * 10_000_000, 100_000, 4_096))
mixed.extend(queries(10**12 + 100_000_000, 10_000_000, 2_048))
random.shuffle(mixed)
datasets.append(("mixed_1e12", mixed))
moderate = [10**14 + 5_000_000 + i * 100_003 + random.next() % 1_024 for i in range(1_024)]
random.shuffle(moderate)
datasets.append(("spaced_1e14", moderate))
isolated = [10**14 + 5_000_000 + i * 1_000_000_003 + random.next() % 1_024 for i in range(1_024)]
random.shuffle(isolated)
datasets.append(("isolated_1e14", isolated))
if args.only:
    wanted = set(args.only.split(","))
    if wanted - {name for name, _ in datasets}: parser.error("Unknown dataset name")
    datasets = [(name, values) for name, values in datasets if name in wanted]
if args.skip_isolated: datasets = [(name, values) for name, values in datasets if name != "isolated_1e14"]

commands = {"ours_auto": [str(rust), "auto"], "ours_direct": [str(rust), "direct"],
            "primal": [str(rust), "primal"]}
if args.include_sieve: commands["ours_sieve"] = [str(rust), "sieve"]
if not args.skip_cpp: commands["primesieve"] = [str(cpp), "adaptive"]
report = {
    "status": "running", "started_at_utc": datetime.now(timezone.utc).isoformat(),
    "platform": __import__("platform").platform(), "seed": args.seed,
    "runs": args.runs, "warmups": args.warmups, "threads": 1,
    "expected_cases": len(datasets), "commands": commands, "build_commands": build_commands,
    "source_sha256": source_hashes,
    "binary_sha256": {name: hashlib.sha256(path.read_bytes()).hexdigest()
                      for name, path in {"rust": rust, "cpp": cpp, "libprimesieve": library}.items()},
    "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
    "clang": subprocess.check_output(["clang++", "--version"], text=True).splitlines()[0],
    "primesieve": subprocess.check_output([str(cli), "--version"], text=True).splitlines()[0],
    "cpu_info": subprocess.check_output([str(cli), "--cpu-info"], text=True).strip(),
    "metric": "Native elapsed ms for a complete batch: sorting, deduplication, routing, computation and answer-order restoration; input parsing and result serialization excluded",
    "cases": [],
}

def atomic_write(path, value):
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n")
    os.replace(temporary, path)

progress = args.output.with_name(args.output.stem + ".progress.json")
for name, values in datasets:
    print(f"Starting {name}: {len(values)} queries", flush=True)
    text = "\n".join(map(str, values)) + "\n"
    case_commands = dict(commands)
    if not args.skip_cpp and max(values) - min(values) <= 200_000_000:
        case_commands["primesieve_stream"] = [str(cpp), "stream"]
    times = {key: [] for key in case_commands}
    stats = {}
    expected = None
    for round_index in range(args.warmups + args.runs):
        names = list(case_commands)
        shift = round_index % len(names)
        names = names[shift:] + names[:shift]
        if round_index % 2: names.reverse()
        for key in names:
            data = json.loads(subprocess.check_output(case_commands[key], input=text, text=True))
            if expected is None: expected = data["answers"]
            if data["answers"] != expected or data["threads"] != 1 or len(expected) != len(values):
                raise RuntimeError(f"Answer/thread mismatch: {name}, {key}")
            if any(p is not None and p <= n for n, p in zip(values, data["answers"])):
                raise RuntimeError("Non-strict next-prime result")
            current_stats = {k: v for k, v in data.items() if k not in ["answers", "elapsed_ns"]}
            if key in stats and stats[key] != current_stats: raise RuntimeError("Unstable router decisions")
            stats[key] = current_stats
            if round_index >= args.warmups: times[key].append(data["elapsed_ns"] / 1e6)
        if name == "isolated_1e14":
            print(f"  isolated round {round_index + 1}/{args.warmups + args.runs} completed", flush=True)
    results = {key: {"median_ms": statistics.median(t), "mean_ms": statistics.mean(t),
                     "min_ms": min(t), "max_ms": max(t), "samples_ms": t, "stats": stats[key]}
               for key, t in times.items()}
    report["cases"].append({"name": name, "query_count": len(values), "min": min(values), "max": max(values),
                            "commands": case_commands,
                            "queries": values, "answers": expected, "results": results})
    print(name + ": " + ", ".join(f"{key}={r['median_ms']:.3f} ms" for key, r in results.items()), flush=True)
    atomic_write(progress, report)

if hashes(sources) != source_hashes: raise RuntimeError("Sources changed during measurements; report not published")
for name, path in {"rust": rust, "cpp": cpp, "libprimesieve": library}.items():
    if hashlib.sha256(path.read_bytes()).hexdigest() != report["binary_sha256"][name]:
        raise RuntimeError(f"{name} binary changed during measurements; report not published")
report["status"] = "complete"
report["completed_at_utc"] = datetime.now(timezone.utc).isoformat()
atomic_write(args.output, report)
progress.unlink(missing_ok=True)
print(f"All full answer vectors agree. Published {args.output}", flush=True)
