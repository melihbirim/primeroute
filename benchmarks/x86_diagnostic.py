"""Native x86 build-feature pilot; not the final optimization comparison."""

import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
primesieve, output = map(Path, sys.argv[1:3])
assert platform.machine() in {"x86_64", "AMD64"}, "Native x86 executor required"
output.mkdir(parents=True, exist_ok=True)
baseline = ROOT / "benchmarks/experiments/x86-2026-11-25/baseline"
manifest = {
    "kind": "exploratory build-feature pilot, sequential suites on one x86 VM",
    "status": "running",
    "started_at_utc": datetime.now(timezone.utc).isoformat(),
    "platform": platform.platform(),
    "lscpu": subprocess.check_output(["lscpu", "--json"], text=True),
    "rustc": subprocess.check_output(["rustc", "-Vv"], text=True),
    "threads": 1,
    "variants": [],
}
for name, flags in [("default", ""), ("v3", "-C target-cpu=x86-64-v3")]:
    source = output.parent / ("x86-source-" + name)
    shutil.copytree(baseline, source, ignore=shutil.ignore_patterns("target", "primeroute", "range_bench"))
    env = os.environ.copy()
    env["RUSTFLAGS"] = flags
    destination = output / name
    command = ["bash", str(source / "benchmarks/ci.sh"), str(primesieve), str(destination), "x86 " + name]
    subprocess.run(command, env=env, check=True)
    manifest["variants"].append({
        "name": name,
        "rustflags": flags,
        "command": command,
        "source_sha256": {str(p.relative_to(source)): hashlib.sha256(p.read_bytes()).hexdigest()
                          for p in sorted((source / "src").glob("*.rs"))},
    })
    (output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
manifest["status"] = "complete"
manifest["completed_at_utc"] = datetime.now(timezone.utc).isoformat()
(output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
