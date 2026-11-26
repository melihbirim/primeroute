"""Fixed 31-sample repeat of the marginal M2 scattered-1e14 comparison."""
import hashlib
import json
import statistics
import subprocess
from datetime import datetime, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
reference = HERE / 'apple-final-paired-results.json'
report = json.loads(reference.read_text())
case = next(c for c in report['cases'] if c['label'] == 'scattered_1e14')
commands = case['commands']
payload = '\n'.join(map(str, case['queries'])) + '\n'
before = {name: sha(Path(argv[0])) for name, argv in commands.items()}
samples = {name: [] for name in commands}
for round_index in range(36):
    order = list(commands)
    shift = round_index % len(order)
    order = order[shift:] + order[:shift]
    if round_index % 2:
        order.reverse()
    for name in order:
        data = json.loads(subprocess.check_output(commands[name], input=payload, text=True))
        assert data['threads'] == 1 and data['answers'] == case['answers'], name
        if round_index >= 5:
            samples[name].append(data['elapsed_ns'] / 1e6)
assert all(sha(Path(argv[0])) == before[name] for name, argv in commands.items())
assert all(sha(ROOT / rel) == digest for rel, digest in report['source_sha256']['ours'].items())
result = {'kind': 'Targeted stability repeat; original 26-case report is unchanged',
          'timestamp_utc': datetime.now(timezone.utc).isoformat(), 'threads': 1,
          'runs': 31, 'warmups': 5, 'case': case['label'], 'commands': commands,
          'reference_sha256': sha(reference), 'binary_sha256': before,
          'driver_sha256': sha(Path(__file__)),
          'results': {name: {'samples_ms': values, 'median_ms': statistics.median(values),
                             'min_ms': min(values), 'max_ms': max(values)}
                      for name, values in samples.items()}}
(HERE / 'apple-scattered-stability.json').write_text(json.dumps(result, indent=2) + '\n')
print({name: round(data['median_ms'], 5) for name, data in result['results'].items()})
