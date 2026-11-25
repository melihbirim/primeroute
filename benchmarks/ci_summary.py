"""Render CI benchmark reports as a Markdown summary. No dependencies.

Usage: ci_summary.py COUNT_GLOB_PATH RANGES_JSON BATCH_JSON TITLE
"""

import json
import sys
from pathlib import Path


def best(results, prefix):
    times = [r["median_ms"] for name, r in results.items() if name.startswith(prefix)]
    return min(times) if times else None


def fmt(ms):
    if ms is None:
        return "—"
    return f"{ms / 1000:.3f} s" if ms >= 1000 else f"{ms:.3f} ms"


def verdict(ours, cpp):
    if ours is None or cpp is None:
        return "—"
    ratio = ours / cpp
    mark = "✅" if ratio < 0.97 else "❌" if ratio > 1.03 else "≈"
    return f"{mark} <0.01×" if ratio < 0.01 else f"{mark} {ratio:.2f}×"


def table(header, rows):
    align = ["---"] + ["---:"] * (len(header) - 1)
    lines = ["| " + " | ".join(header) + " |", "| " + " | ".join(align) + " |"]
    lines += ["| " + " | ".join(row) + " |" for row in rows]
    return "\n".join(lines)


def main():
    count, ranges, batch = (Path(p) for p in sys.argv[1:4])
    title = sys.argv[4]
    sections = []
    wins = total = 0

    rows = []
    for path in sorted(count.parent.glob(count.name)):
        report = json.loads(path.read_text())
        res = report["results"]
        ours = res["ours"]["median_ms"]
        cpp = best(res, "primesieve")
        primal = res.get("primal_rust", {}).get("median_ms")
        rows.append([f"{report['limit_inclusive']:,}", fmt(ours), fmt(cpp), fmt(primal), verdict(ours, cpp)])
        total += 1
        wins += ours < cpp
    if rows:
        sections += ["### Count primes up to N (process time)", "",
                     table(["N", "primeroute", "primesieve", "primal", "vs primesieve"], rows), ""]

    if ranges.exists():
        rows = []
        for case in json.loads(ranges.read_text())["cases"]:
            res = case["results"]
            ours = res["ours"]["median_ms"]
            cpp = best(res, "cpp")
            primal = res.get("primal_segmented_adapter", {}).get("median_ms")
            rows.append([f"{case['start']:,}", f"{case['width']:,}", case["mode"],
                         fmt(ours), fmt(cpp), fmt(primal), verdict(ours, cpp)])
            total += 1
            wins += ours < cpp
        sections += ["### Ranges (per fresh query)", "",
                     table(["Start", "Width", "Mode", "primeroute", "primesieve", "primal adapter", "vs primesieve"], rows), ""]

    if batch.exists():
        rows = []
        for case in json.loads(batch.read_text())["cases"]:
            res = case["results"]
            ours = res["ours_auto"]["median_ms"]
            cpp = best(res, "primesieve")
            primal = res.get("primal", {}).get("median_ms")
            rows.append([case["name"], f"{case['query_count']:,}", fmt(ours), fmt(cpp), fmt(primal), verdict(ours, cpp)])
            total += 1
            wins += cpp is not None and ours < cpp
        sections += ["### Batch next-prime (whole batch)", "",
                     table(["Workload", "Queries", "primeroute", "primesieve", "primal", "vs primesieve"], rows), ""]

    header = [f"## {title}", "",
              f"**primeroute faster than primesieve in {wins}/{total} cases.** "
              "Ratio = primeroute time / primesieve time: ✅ below 0.97, ❌ above 1.03, ≈ in between. "
              "All comparators use one thread. Shared CI runners are noisy; "
              "treat these as indicative, not as published results.", ""]
    print("\n".join(header + sections))


main()
