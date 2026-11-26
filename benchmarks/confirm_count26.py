"""Independently confirm the candidate selected by counting26.py."""
import hashlib
import json
import os
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
CPP, OUT = map(Path, sys.argv[1:3])
report = json.loads((OUT / 'pilot.json').read_text())
assert report['status'] == 'screened'
winner = report['selected_variant']
sources = {name: OUT / 'sources' / name for name in report['variants']}
programs = {name: source / 'target/release/primeroute' for name, source in sources.items()}
profile_source = OUT / 'sources/profile_baseline'
allowed = report['allowed_cpus_before']
cpp = CPP / 'build/primesieve'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
env = os.environ.copy()
env['RUSTFLAGS'] = ''
env.pop('CARGO_ENCODED_RUSTFLAGS', None)
env.pop('CARGO_TARGET_DIR', None)
def run(argv, **kwargs): subprocess.run(list(map(str, argv)), check=True, **kwargs)
def save(): (OUT / 'pilot.json').write_text(json.dumps(report, indent=2) + '\n')
def hashes(source):
    files = [*sorted((source / 'src').glob('*.rs')), *sorted((source / 'examples').glob('*.rs')),
             source / 'Cargo.toml', source / 'Cargo.lock', source / 'benchmarks/primal/Cargo.toml',
             source / 'benchmarks/primal/Cargo.lock', *sorted((source / 'benchmarks/primal/src').rglob('*.rs'))]
    return {str(p.relative_to(source)): sha(p) for p in files}
report['confirmation_driver_sha256'] = sha(Path(__file__))
# Compile helpers after restoring the build affinity. No benchmark runs while
# compilation runs. The final driver pins all compared work to one CPU again.
os.sched_setaffinity(0, set(allowed))
for name in ['baseline', winner]:
    source = sources[name]
    run(['cargo', 'build', '--manifest-path', source / 'Cargo.toml', '--release', '--locked', '--example', 'range_bench'], env=env)
    run(['cargo', 'build', '--manifest-path', source / 'benchmarks/primal/Cargo.toml', '--release', '--locked', '--bins'], env=env)
for name in ['range_bench', 'batch_next']:
    run(['clang++', '-O3', '-std=c++14', '-I' + str(CPP / 'include'), ROOT / 'benchmarks' / (name + '.cpp'),
         CPP / 'build/libprimesieve.a', '-pthread', '-o', OUT / (name + '-cpp')])
run([sys.executable, sources[winner] / 'benchmarks/platform_compare.py', '--baseline', sources['baseline'],
     '--primesieve-dir', CPP, '--cpp-range', OUT / 'range_bench-cpp', '--cpp-batch', OUT / 'batch_next-cpp',
     '--output', OUT / 'paired-results.json', '--runs', '9', '--warmups', '2', '--large', '--large-runs', '5',
     '--large-cpp-sizes', '256', '1024'], env=env)
for name, source in sources.items():
    assert hashes(source) == report['variants'][name]['source_sha256'], name
    assert sha(programs[name]) == report['variants'][name]['binary_sha256'], name
report['status'] = 'complete'
report['completed_at_utc'] = datetime.now(timezone.utc).isoformat()
binary_directory = OUT / 'binaries'
binary_directory.mkdir()
for name, path in [('baseline', programs['baseline']), ('selected', programs[winner]),
                   ('primesieve', cpp), ('primal', sources[winner] / 'benchmarks/primal/target/release/primal-benchmark')]:
    shutil.copy2(path, binary_directory / name)
report['retained_counter_binaries_sha256'] = {p.name: sha(p) for p in binary_directory.iterdir()}
save()

# Retain source and hashes, not seven build directories or repeated inputs.
for source in sources.values():
    shutil.rmtree(source / 'target')
    shutil.rmtree(source / 'benchmarks/primal/target', ignore_errors=True)
    (source / 'benchmarks/batch-next-results.json').unlink()
shutil.rmtree(profile_source / 'target')
(profile_source / 'benchmarks/batch-next-results.json').unlink()
print('Independent full comparison complete; counts, checksums, answers and source/binary freezes verified.', flush=True)
