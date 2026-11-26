"""Regression tests for summary classification, using synthetic test timings."""

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

import ci_summary


class SummaryTests(unittest.TestCase):
    def test_threshold_boundaries_and_missing_comparisons(self):
        cases = [
            (96.9, 100, "✅"),
            (97, 100, "≈"),
            (99, 100, "≈"),
            (100, 100, "≈"),
            (103, 100, "≈"),
            (103.1, 100, "❌"),
            (None, 100, "—"),
            (100, None, "—"),
        ]
        for ours, cpp, expected in cases:
            with self.subTest(ours=ours, cpp=cpp):
                self.assertEqual(ci_summary.outcome(ours, cpp), expected)
                self.assertTrue(ci_summary.verdict(ours, cpp).startswith(expected))

    def test_total_matches_badges_across_all_report_types(self):
        # These values test the renderer, not implementation performance.
        def results(ours, **comparators):
            return {
                name: {"median_ms": value}
                for name, value in {"ours": ours, **comparators}.items()
            }

        count = {
            "limit_inclusive": 100,
            "results": results(99, primesieve_cpp=100),
        }
        ranges = {
            "cases": [
                {
                    "start": 100,
                    "width": 100,
                    "mode": "count",
                    "results": results(96, cpp_default=200, cpp_1024k=100),
                },
                {
                    "start": 100,
                    "width": 100,
                    "mode": "generate",
                    "results": results(97, cpp_default=100),
                },
            ]
        }
        batch = {
            "cases": [
                {
                    "name": name,
                    "query_count": 2,
                    "results": {
                        "ours_auto": {"median_ms": ours},
                        "primesieve": {"median_ms": 200},
                        "primesieve_stream": {"median_ms": 100},
                    },
                }
                for name, ours in [("win", 96.9), ("loss", 110)]
            ]
        }
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            for name, report in [
                ("count-100.json", count),
                ("ranges.json", ranges),
                ("batch.json", batch),
            ]:
                (folder / name).write_text(json.dumps(report))
            summary = subprocess.check_output(
                [
                    sys.executable,
                    str(Path(ci_summary.__file__)),
                    str(folder / "count-*.json"),
                    str(folder / "ranges.json"),
                    str(folder / "batch.json"),
                    "Synthetic classification fixture",
                ],
                text=True,
            )

        self.assertIn("wins against primesieve in 2/5 cases", summary)
        rows = [line for line in summary.splitlines() if line.startswith("|")]
        self.assertEqual(sum("✅" in row for row in rows), 2)
        self.assertEqual(sum("≈" in row for row in rows), 2)
        self.assertEqual(sum("❌" in row for row in rows), 1)


if __name__ == "__main__":
    unittest.main()
