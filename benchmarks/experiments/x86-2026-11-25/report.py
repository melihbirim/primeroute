"""Audit completed paired reports and render their full, unselected results."""
import hashlib
import json
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(ROOT / 'benchmarks'))
from ci_summary import best, fmt, outcome, table, verdict

sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
paths = {
    'linux': next((HERE / 'inplace-paired').glob('*/paired-results.json')),
    'apple': HERE / 'apple-inplace-paired-results.json',
}
reports = {name: json.loads(path.read_text()) for name, path in paths.items()}
assert [c['label'] for c in reports['linux']['cases']] == [c['label'] for c in reports['apple']['cases']]
assessment = {'comparison_band': {'win_below': 0.97, 'loss_above': 1.03}, 'platforms': {}}
sections = ['# Full paired results', '', 'Every case is included. Win/tie/loss uses full-precision medians and the same 3% band as CI. C++ and primal columns use the faster measured configuration median; individual samples are never selected.', '']
for name, report in reports.items():
    assert report['status'] == 'complete' and report['threads'] == 1 and len(report['cases']) == 26
    for rel, digest in report['source_sha256']['ours'].items():
        assert sha(ROOT / rel) == digest, (name, rel)
    assert report['source_sha256']['baseline'] == reports['linux']['source_sha256']['baseline']
    assert all(sha(HERE / 'baseline' / rel) == digest for rel, digest in report['source_sha256']['baseline'].items())
    scores = {key: {'wins': 0, 'ties': 0, 'losses': 0} for key in ['cpp', 'primal', 'baseline']}
    rows = []
    lost_wins = []
    for case in report['cases']:
        results = case['results']
        for result in results.values():
            assert len(result['samples_ms']) == case['runs']
            assert statistics.median(result['samples_ms']) == result['median_ms']
            assert min(result['samples_ms']) == result['min_ms']
            assert max(result['samples_ms']) == result['max_ms']
        own, base = results['ours']['median_ms'], results['baseline']['median_ms']
        cpp, primal = best(results, 'cpp_'), best(results, 'primal')
        for key, ref in [('cpp', cpp), ('primal', primal), ('baseline', base)]:
            scores[key][{'✅': 'wins', '≈': 'ties', '❌': 'losses'}[outcome(own, ref)]] += 1
        if outcome(base, cpp) == '✅' and outcome(own, cpp) != '✅':
            lost_wins.append(case['label'])
        if case['kind'] == 'prefix':
            assert case['prime_count'] == {10**8: 5_761_455, 10**9: 50_847_534, 10**10: 455_052_511, 10**11: 4_118_054_813}[case['limit']]
        rows.append({'label': case['label'], 'kind': case['kind'], 'baseline_ms': base, 'ours_ms': own,
                     'cpp_ms': cpp, 'primal_ms': primal, 'change_percent': 100 * (own / base - 1),
                     'vs_cpp': outcome(own, cpp), 'vs_primal': outcome(own, primal), 'vs_baseline': outcome(own, base)})
    assessment['platforms'][name] = {'raw_report': str(paths[name].relative_to(HERE)), 'raw_sha256': sha(paths[name]),
        'cpu_info': report['cpu_info'], 'affinity': report['affinity'], 'scores': scores,
        'baseline_cpp_wins_lost': lost_wins, 'source_sha256': report['source_sha256'], 'cases': rows}
    labels = {'cpp': 'primesieve', 'primal': 'primal', 'baseline': 'Previous Rust'}
    sections += ['## ' + {'linux': 'Linux x86', 'apple': 'Apple Silicon'}[name], '', report['cpu_info'].splitlines()[0], '',
                 table(['Comparator', 'Wins', 'Ties', 'Losses'],
                       [[labels[key], str(v['wins']), str(v['ties']), str(v['losses'])] for key, v in scores.items()]), '']
    for kind in ['prefix', 'range', 'batch']:
        entries = []
        for row, case in zip(rows, report['cases']):
            if row['kind'] != kind:
                continue
            v3 = case['results'].get('baseline_v3', {}).get('median_ms')
            entries.append([row['label'], fmt(row['baseline_ms']), fmt(v3), fmt(row['ours_ms']),
                            fmt(row['cpp_ms']), fmt(row['primal_ms']), f"{row['change_percent']:+.1f}%",
                            verdict(row['ours_ms'], row['cpp_ms'])])
        sections += ['### ' + kind, '', table(['Case', 'Baseline', 'Baseline v3', 'Candidate', 'C++', 'Primal', 'Change vs baseline', 'vs C++'], entries), '']

assessment['supplemental_apple'] = {}
for name in ['apple-inplace-short-results.json', 'apple-inplace-holdout-results.json']:
    path = HERE / name
    report = json.loads(path.read_text())
    assert report['status'] == 'complete' and len(report['cases']) == 24
    assert report['source_sha256']['src/lib.rs'] == reports['apple']['source_sha256']['ours']['src/lib.rs']
    assert report['binary_sha256']['ours'] == reports['apple']['binary_sha256']['ours_range']
    scores = {}
    for variant in ['ours', 'ours_baseline']:
        marks = [outcome(c['results'][variant]['median_ms'], best(c['results'], 'cpp_')) for c in report['cases']]
        scores[variant] = {'wins': marks.count('✅'), 'ties': marks.count('≈'), 'losses': marks.count('❌')}
    assessment['supplemental_apple'][name] = {'raw_sha256': sha(path), 'scores': scores}
assessment['generator_sha256'] = sha(Path(__file__))
(HERE / 'assessment.json').write_text(json.dumps(assessment, indent=2) + '\n')
(HERE / 'paired-summary.md').write_text('\n'.join(sections))
print(json.dumps({name: data['scores'] for name, data in assessment['platforms'].items()}, indent=2))
print('Raw medians, sample counts, known counts, baseline hashes and selected source hashes verified.')
