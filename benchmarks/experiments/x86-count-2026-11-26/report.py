"""Audit this experiment's reports without changing historical measurements."""
import hashlib
import json
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(ROOT / 'benchmarks'))
from ci_summary import best, fmt, outcome, table

sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
reports = sorted(HERE.glob('*/paired-results.json'))
reports += sorted(HERE.glob('apple*paired-results.json'))
assessment = {'comparison_band': {'win_below': 0.97, 'loss_above': 1.03}, 'reports': {}}
sections = ['# All independently confirmed experiments', '',
            'Times are milliseconds. Each table includes every measured case. '
            'C++ uses the fastest configuration median measured for that case; '
            'individual samples are never selected. Screening pilots are excluded.', '']
for path in reports:
    report = json.loads(path.read_text())
    assert report['status'] == 'complete' and report['threads'] == 1
    assert len(report['cases']) == 26
    baseline = report['source_sha256']['baseline']
    assert all(sha(HERE / 'baseline' / rel) == digest for rel, digest in baseline.items())
    assert report['input_sha256'] == sha(ROOT / 'benchmarks/batch-next-results.json')
    assert report['runner_sha256'] == sha(ROOT / 'benchmarks/platform_compare.py')
    assert all(sha(ROOT / 'benchmarks' / rel) == digest
               for rel, digest in report['cpp_sources_sha256'].items())
    pilot_path = path.parent / 'pilot.json'
    if pilot_path.exists():
        pilot = json.loads(pilot_path.read_text())
        assert pilot['status'] == 'complete'
        selected = pilot['selected_variant']
        for variant, sources in [('baseline', baseline), ('ours', report['source_sha256']['ours'])]:
            source_root = path.parent / 'sources' / ('baseline' if variant == 'baseline' else selected)
            assert all(sha(source_root / rel) == digest for rel, digest in sources.items())
        assert all(sha(path.parent / 'binaries' / name) == digest
                   for name, digest in pilot['retained_counter_binaries_sha256'].items())
    elif path.name.startswith('apple'):
        snapshot = HERE / ('apple-final-source' if path.name == 'apple-final-paired-results.json'
                           else 'apple-shared-source')
        assert all(sha(snapshot / rel) == digest
                   for rel, digest in report['source_sha256']['ours'].items())
    scores = {key: {'wins': 0, 'ties': 0, 'losses': 0} for key in ['cpp', 'primal', 'baseline']}
    rows, lost_wins = [], []
    for case in report['cases']:
        results = case['results']
        for result in results.values():
            samples = result['samples_ms']
            assert len(samples) == case['runs']
            assert (statistics.median(samples), min(samples), max(samples)) == (
                result['median_ms'], result['min_ms'], result['max_ms'])
        own, base = results['ours']['median_ms'], results['baseline']['median_ms']
        cpp, primal = best(results, 'cpp_'), best(results, 'primal')
        for key, ref in [('cpp', cpp), ('primal', primal), ('baseline', base)]:
            scores[key][{'✅': 'wins', '≈': 'ties', '❌': 'losses'}[outcome(own, ref)]] += 1
        if outcome(base, cpp) == '✅' and outcome(own, cpp) != '✅':
            lost_wins.append(case['label'])
        if case['kind'] == 'prefix':
            assert case['prime_count'] == {10**8: 5_761_455, 10**9: 50_847_534,
                                         10**10: 455_052_511, 10**11: 4_118_054_813}[case['limit']]
        rows.append([case['label'], fmt(base), fmt(own), fmt(cpp), fmt(primal), outcome(own, cpp)])
    name = str(path.relative_to(HERE))
    assessment['reports'][name] = {'sha256': sha(path), 'scores': scores,
        'baseline_cpp_wins_lost': lost_wins, 'source_sha256': report['source_sha256']}
    sections += ['## ' + name, '', report['cpu_info'].splitlines()[0], '',
                 table(['Comparator', 'Wins', 'Ties', 'Losses'],
                       [[key, str(v['wins']), str(v['ties']), str(v['losses'])] for key, v in scores.items()]), '',
                 table(['Case', 'Previous Rust', 'Candidate', 'C++', 'Primal', 'vs C++'], rows), '']
assessment['generator_sha256'] = sha(Path(__file__))
selected_path = HERE / 'mixed-fill-confirmation/paired-results.json'
selected = json.loads(selected_path.read_text())
assert all(sha(ROOT / rel) == digest for rel, digest in selected['source_sha256']['ours'].items())
assessment['selected_source_matches_working_tree'] = True
assessment['selected_report'] = str(selected_path.relative_to(HERE))
equivalence = json.loads((HERE / 'arm-selected-equivalence.json').read_text())
assert equivalence['before_binaries'] == equivalence['after_binaries']
assert equivalence['all_arm_executables_byte_identical']
assessment['arm_selected_executables_unchanged'] = True
assessment['supplemental_apple'] = {}
for filename in ['apple-final-short-results.json', 'apple-final-holdout-results.json']:
    path = HERE / filename
    extra = json.loads(path.read_text())
    assert extra['status'] == 'complete' and len(extra['cases']) == 24 and extra['threads'] == 1
    scores, lost = {}, []
    for variant in ['ours', 'ours_baseline']:
        marks = []
        for case in extra['cases']:
            for result in case['results'].values():
                assert len(result['samples_ms']) == extra['runs']
                assert statistics.median(result['samples_ms']) == result['median_ms']
            marks.append(outcome(case['results'][variant]['median_ms'], best(case['results'], 'cpp_')))
        scores[variant] = {'wins': marks.count('✅'), 'ties': marks.count('≈'), 'losses': marks.count('❌')}
    for case in extra['cases']:
        cpp = best(case['results'], 'cpp_')
        if outcome(case['results']['ours_baseline']['median_ms'], cpp) == '✅' and outcome(case['results']['ours']['median_ms'], cpp) != '✅':
            lost.append({'start': case['start'], 'width': case['width'], 'mode': case['mode']})
    assert not lost, (filename, lost)
    assessment['supplemental_apple'][filename] = {'sha256': sha(path), 'scores': scores, 'baseline_wins_lost': lost}
stability = json.loads((HERE / 'apple-scattered-stability.json').read_text())
assert stability['reference_sha256'] == sha(HERE / 'apple-final-paired-results.json')
assert stability['driver_sha256'] == sha(HERE / 'repeat_scattered.py')
assert all(len(result['samples_ms']) == 31 and statistics.median(result['samples_ms']) == result['median_ms'] for result in stability['results'].values())
historical = json.loads((HERE / 'historical-results-sha256.json').read_text())
assert all(sha(ROOT / rel) == digest for rel, digest in historical.items())
assessment['historical_reports_and_generator_unchanged'] = len(historical)
(HERE / 'assessment.json').write_text(json.dumps(assessment, indent=2) + '\n')
(HERE / 'paired-summary.md').write_text('\n'.join(sections))
print(json.dumps({name: data['scores'] for name, data in assessment['reports'].items()}, indent=2))
print('Verified complete samples, medians, counts, baseline sources, retained sources and binaries.')
