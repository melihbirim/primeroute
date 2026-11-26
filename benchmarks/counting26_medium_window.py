"""Native x86 kernel search, followed by an independent full paired run."""
import hashlib
import json
import os
import platform
import re
import shutil
import statistics
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CPP, OUT = map(Path, sys.argv[1:3])
assert platform.machine() in {'x86_64', 'AMD64'}
OUT.mkdir(parents=True, exist_ok=False)
BASE = ROOT / 'benchmarks/experiments/x86-count-2026-11-26/baseline'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
env = os.environ.copy()
env['RUSTFLAGS'] = ''
env.pop('CARGO_ENCODED_RUSTFLAGS', None)
env.pop('CARGO_TARGET_DIR', None)
report = {'kind': 'kernel selection pilot; final comparison is paired-results.json',
          'started_at_utc': datetime.now(timezone.utc).isoformat(), 'status': 'running', 'threads': 1,
          'lscpu': subprocess.check_output(['lscpu', '--json'], text=True),
          'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
          'driver_sha256': sha(Path(__file__)), 'variants': {}, 'screening': []}

def save():
    (OUT / 'pilot.json').write_text(json.dumps(report, indent=2) + '\n')

def run(argv, **kwargs):
    subprocess.run(list(map(str, argv)), check=True, **kwargs)

def project(source, destination):
    destination.mkdir(parents=True)
    for directory in ['src', 'examples', 'benchmarks/primal/src']:
        shutil.copytree(source / directory, destination / directory)
    for name in ['Cargo.toml', 'Cargo.lock', 'benchmarks/primal/Cargo.toml', 'benchmarks/primal/Cargo.lock']:
        shutil.copy2(source / name, destination / name)
    for license_name in ['LICENSE', 'THIRD_PARTY_NOTICES.md']:
        if (ROOT / license_name).exists(): shutil.copy2(ROOT / license_name, destination / license_name)
    for name in ['platform_compare.py', 'batch-next-results.json', 'range_bench.cpp', 'batch_next.cpp']:
        shutil.copy2(ROOT / 'benchmarks' / name, destination / 'benchmarks' / name)

def hashes(source):
    files = [*sorted((source / 'src').glob('*.rs')), *sorted((source / 'examples').glob('*.rs')),
             source / 'Cargo.toml', source / 'Cargo.lock', source / 'benchmarks/primal/Cargo.toml',
             source / 'benchmarks/primal/Cargo.lock', *sorted((source / 'benchmarks/primal/src').rglob('*.rs'))]
    return {str(p.relative_to(source)): sha(p) for p in files}

def profile_project(source):
    path = source / 'src/lib.rs'
    text = path.read_text()
    begin = text.index('fn sieve_segments_with_seeds(')
    end = text.index('\nfn finish_segment', begin)
    body = text[begin:end]
    replacements = [
        ('    if start > limit', '    let profile_setup = std::time::Instant::now();\n    if start > limit'),
        ('    while low < total_bytes {', '    profile_record(0, profile_setup);\n    while low < total_bytes {\n        let profile_tick = std::time::Instant::now();'),
        ('        // Activate each prime once,', '        profile_record(1, profile_tick);\n        let profile_tick = std::time::Instant::now();\n        // Activate each prime once,'),
        ('        let next_medium_active =', '        profile_record(2, profile_tick);\n        let profile_tick = std::time::Instant::now();\n        let next_medium_active ='),
        ('        // Move pending primes', '        profile_record(3, profile_tick);\n        let profile_tick = std::time::Instant::now();\n        // Move pending primes'),
        ('        finish_segment(start, limit, first_byte, low, sieve);', '        profile_record(4, profile_tick);\n        let profile_tick = std::time::Instant::now();\n        finish_segment(start, limit, first_byte, low, sieve);'),
        ('        low += len;', '        profile_record(5, profile_tick);\n        low += len;'),
    ]
    for old, new in replacements:
        assert body.count(old) == 1, old
        body = body.replace(old, new)
    header = '''
std::thread_local! { static PROFILE_TIMES: std::cell::Cell<[u64; 6]> = const { std::cell::Cell::new([0; 6]) }; }
fn profile_record(index: usize, start: std::time::Instant) {
    PROFILE_TIMES.with(|slot| { let mut values = slot.get(); values[index] += start.elapsed().as_nanos() as u64; slot.set(values); });
}
pub fn profile_times() -> [u64; 6] { PROFILE_TIMES.with(|slot| slot.get()) }
'''
    path.write_text(header + text[:begin] + body + text[end:])
    (source / 'examples/profile_count.rs').write_text(r'''
fn main() {
    let limit: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let start = std::time::Instant::now();
    let count = primeroute::count_primes(limit);
    println!("{{\"count\":{},\"total_ns\":{},\"stages_ns\":{:?},\"threads\":1}}", count, start.elapsed().as_nanos(), primeroute::profile_times());
}
''')

# Variants are compile-time algorithms, not query-dependent shortcuts. Every
# screened implementation still sieves and verifies the complete prime count.
settings = [
    ('baseline', None, None, None, ''),
    ('queue8_outer128', 8, 32, 17, ''),
    ('queue8_outer256', 8, 32, 17, ''),
    ('queue8_outer512', 8, 32, 17, ''),
    ('queue8_outer1024', 8, 32, 17, ''),
    ('queue16_outer256', 16, 32, 17, ''),
    ('queue16_outer512', 16, 32, 17, ''),
    ('queue16_outer1024', 16, 32, 17, ''),
]
sources = {}
programs = {}
for name, dense, block, fill, flags in settings:
    source = OUT / 'sources' / name
    project(BASE if dense is None else ROOT, source)
    if dense is not None:
        p = source / 'src/x86.rs'
        text = p.read_text()
        segment = int(re.search(r'outer(\d+)$', name)[1])
        cache_limit = 32
        edits = [(r'CACHE_BLOCK: usize = \d+ \* 1024', f'CACHE_BLOCK: usize = {block} * 1024'),
                 (r'SEGMENT_BYTES: usize = \d+ \* 1024', f'SEGMENT_BYTES: usize = {segment} * 1024'),
                 (r'VECTOR_STATE_ADVANCE: bool = (true|false)', 'VECTOR_STATE_ADVANCE: bool = false'),
                 (r'QUEUED_DENSE_MIN: usize = \d+ \* 1024', f'QUEUED_DENSE_MIN: usize = {dense} * 1024')]
        for pattern, replacement in edits:
            text, count = re.subn(pattern, replacement, text)
            assert count == 1, pattern
        p.write_text(text)
    variant_env = env | {'RUSTFLAGS': flags}
    run(['cargo', 'fmt', '--manifest-path', source / 'Cargo.toml', '--check'], env=variant_env)
    run(['cargo', 'clippy', '--manifest-path', source / 'Cargo.toml', '--all-targets', '--locked', '--', '-D', 'warnings'], env=variant_env)
    run(['cargo', 'test', '--manifest-path', source / 'Cargo.toml', '--release', '--locked', '--', '--test-threads=1'], env=variant_env)
    run(['cargo', 'build', '--manifest-path', source / 'Cargo.toml', '--release', '--locked', '--bin', 'primeroute'], env=variant_env)
    sources[name] = source
    programs[name] = source / 'target/release/primeroute'
    report['variants'][name] = {'dense_kib': dense, 'block_kib': block, 'fill_group': fill, 'compact_dense': False, 'cache_dense_kib': cache_limit if dense is not None else None, 'dense_backend': 'sorted_medium_canonical_bulk' if dense is not None else None, 'compact_fill': False, 'segment_kib': segment if dense is not None else 128, 'rustflags': flags,
                                'source_sha256': hashes(source), 'binary_sha256': sha(programs[name])}
    save()

profile_source = OUT / 'sources/profile_baseline'
project(BASE, profile_source)
profile_project(profile_source)
run(['cargo', 'fmt', '--manifest-path', profile_source / 'Cargo.toml'], env=env)
run(['cargo', 'build', '--manifest-path', profile_source / 'Cargo.toml', '--release', '--locked', '--example', 'profile_count'], env=env)

# Pin only timed work; compilation above is excluded and can use the VM cores.
allowed = sorted(os.sched_getaffinity(0))
os.sched_setaffinity(0, {allowed[0]})
report['allowed_cpus_before'] = allowed
report['affinity'] = sorted(os.sched_getaffinity(0))
cpp = CPP / 'build/primesieve'
report['cpu_info'] = subprocess.check_output([cpp, '--cpu-info'], text=True)
report['primesieve_version'] = subprocess.check_output([cpp, '--version'], text=True)
profile = json.loads(subprocess.check_output([profile_source / 'target/release/examples/profile_count', str(10**11)], text=True))
assert profile['count'] == 4_118_054_813 and profile['threads'] == 1
report['instrumented_baseline'] = {'kind': 'instrumented component selection evidence, not final timing',
                                  'stage_names': ['setup', 'fill', 'dense', 'medium', 'buckets', 'finish_and_count'],
                                  'source_sha256': hashes(profile_source), **profile}
save()
print('Instrumented baseline stage milliseconds:', dict(zip(report['instrumented_baseline']['stage_names'],
      [round(ns / 1e6, 3) for ns in profile['stages_ns']])), flush=True)

def measure(label, limit, names, runs, warmups):
    commands = {name: [str(programs[name]), str(limit)] for name in names}
    commands['cpp_default'] = [str(cpp), str(limit), '--threads=1', '--count=1', '--quiet', '--no-status']
    samples = {name: [] for name in commands}
    expected = {10**9: 50_847_534, 10**10: 455_052_511, 10**11: 4_118_054_813}[limit]
    for round_index in range(runs + warmups):
        order = list(commands)
        shift = round_index % len(order)
        order = order[shift:] + order[:shift]
        if round_index % 2: order.reverse()
        for name in order:
            before = time.perf_counter_ns()
            result = subprocess.run(commands[name], check=True, capture_output=True, text=True)
            elapsed = (time.perf_counter_ns() - before) / 1e6
            assert int(result.stdout.strip()) == expected, (label, name, result.stdout)
            if round_index >= warmups: samples[name].append(elapsed)
    record = {'label': label, 'limit': limit, 'prime_count': expected, 'runs': runs, 'warmups': warmups,
              'commands': commands, 'results': {name: {'median_ms': statistics.median(values), 'samples_ms': values}
                                              for name, values in samples.items()}}
    report['screening'].append(record)
    save()
    print(label, {name: round(data['median_ms'], 3) for name, data in record['results'].items()}, flush=True)
    return record

measure('1B screening', 10**9, list(programs), 3, 1)
small = measure('10B screening', 10**10, list(programs), 5, 1)
candidates = [name for name, dense, *_ in settings if dense is not None]
top = candidates  # Select on the actual target workload, not a smaller prefix.
large = measure('100B screening; one sample is selection evidence only', 10**11, [name for name in ['baseline', 'baseline_v3'] if name in sources] + top, 1, 0)
winner = min(top, key=lambda name: large['results'][name]['median_ms'])
report['selected_variant'] = winner
report['selection_rule'] = 'Fastest candidate on a one-sample 100B pilot over all variants; 10B is diagnostic only. Independent fixed-sample confirmation follows.'
save()

report['status'] = 'screened'
report['screen_completed_at_utc'] = datetime.now(timezone.utc).isoformat()
save()
if '--screen-only' not in sys.argv[3:]:
    run([sys.executable, ROOT / 'benchmarks/confirm_count26.py', CPP, OUT])
