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
env = os.environ.copy()
env["RUSTFLAGS"] = ""
subprocess.run(["cargo", "fmt", "--check"], cwd=ROOT, env=env, check=True)
subprocess.run(["cargo", "clippy", "--all-targets", "--locked", "--", "-D", "warnings"], cwd=ROOT, env=env, check=True)
subprocess.run(["cargo", "test", "--release", "--locked", "--", "--test-threads=1"], cwd=ROOT, env=env, check=True)
with (output / "kernel-profiles.log").open("w") as log:
    subprocess.run(["cargo", "test", "--release", "--locked", "x86::tests::profile_", "--", "--ignored", "--nocapture", "--test-threads=1"], cwd=ROOT, env=env, stdout=log, check=True)
subprocess.run(["cargo", "build", "--release", "--locked", "--bin", "primeroute", "--example", "range_bench"],cwd=ROOT,env=env,check=True)
subprocess.run(["cargo", "build", "--release", "--locked", "--manifest-path", str(ROOT / "benchmarks/primal/Cargo.toml"), "--bins"],cwd=ROOT,env=env,check=True)
for name in ["range_bench", "batch_next"]:
    subprocess.run(["clang++", "-O3", "-std=c++14", "-I"+str(primesieve/"include"), str(ROOT/"benchmarks"/(name+".cpp")),str(primesieve/"build/libprimesieve.a"),"-pthread","-o",str(ROOT/"target/release"/(name+"-cpp"))],check=True)
subprocess.run([sys.executable,str(ROOT/"benchmarks/platform_compare.py"),"--baseline",str(output.parent/"x86-source-default"),"--v3",str(output.parent/"x86-source-v3"),"--primesieve-dir",str(primesieve),"--cpp-range",str(ROOT/"target/release/range_bench-cpp"),"--cpp-batch",str(ROOT/"target/release/batch_next-cpp"),"--output",str(output/"paired-results.json"),"--runs","9","--warmups","2","--large"],env=env,cwd=ROOT,check=True)
manifest["candidate_source_sha256"] = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((ROOT / "src").glob("*.rs"))}
manifest["status"] = "complete"
manifest["completed_at_utc"] = datetime.now(timezone.utc).isoformat()
(output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
