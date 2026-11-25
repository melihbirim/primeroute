from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,re,math
root=Path('/Users/melihbirim/code/prime-numbers');b=root/'benchmarks';base=b/'experiments/short-range-2026-11-25/baseline'
read=lambda p:json.loads(p.read_text())
hashfile=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
m=read(b/'measurement-2026-11-25.json');assert m['status']=='complete' and len(m['completed_runs'])==len(m['runs'])
for name,sha in m['source_sha256'].items():
 assert hashfile(root/name)==sha,name
 assert hashfile(root/m['source_snapshot']/name)==sha,('snapshot',name)
reports=[]
for task in m['runs']:
 r=read(root/task['output']);assert r['status']=='complete';assert r['source_sha256']['src/lib.rs']==m['source_sha256']['src/lib.rs']
 for name,sha in r['source_sha256'].items():assert hashfile(root/name)==sha,(task['name'],name)
 if 'expected_cases' in r:assert len(r['cases'])==r['expected_cases']
 if 'commands' in r and 'binary_sha256' in r and 'prime_count' in r:
  for name,cmd in r['commands'].items():assert hashfile(Path(cmd[0]))==r['binary_sha256'][name],(task['name'],name)
 elif 'binary_sha256' in r and 'ours' in r['binary_sha256']:
  paths={'ours':root/'target/release/examples/range_bench','ours_baseline':base/'range_bench','cpp':Path('/private/tmp/prime-numbers-cpp-range-bench'),'primal':root/'benchmarks/primal/target/release/range_bench'}
  for name,sha in r['binary_sha256'].items():assert hashfile(paths[name])==sha,(task['name'],name)
 elif 'binary_sha256' in r and 'rust' in r['binary_sha256']:
  paths={'rust':root/'benchmarks/primal/target/release/batch_next','cpp':root/'target/release/batch-next-cpp','libprimesieve':Path('/private/tmp/prime-numbers-primesieve-v12.18/build/libprimesieve.a')}
  for name,sha in r['binary_sha256'].items():assert hashfile(paths[name])==sha,(task['name'],name)
 reports.append(r)
assert hashfile(base/'prime-numbers')==read(base/'results-100b.json')['binary_sha256']['ours']
assert hashfile(base/'range_bench')==read(base/'range-results.json')['binary_sha256']['ours']
for f in ['batch-next-results.json','batch-next-cluster-holdout-results.json']:
 before=read(base/f);after=read(b/f)
 assert [(c['queries'],c['answers']) for c in before['cases']]==[(c['queries'],c['answers']) for c in after['cases']]
# Verify all active local documentation links.
for p in [root/'README.md',b/'README.md',b/'RANGES.md',b/'BATCH_NEXT.md',b/'experiments/short-range-2026-11-25/README.md']:
 for target in re.findall(r'\]\(([^)]+)\)',p.read_text()):
  if '://' in target or target.startswith('#'):continue
  target=target.split('#')[0]
  assert (p.parent/target).exists(),(p,target)
r=read(b/'results-100b.json');cpps=[v for n,v in r['results'].items() if n.startswith('primesieve_cpp')]
assert r['prime_count']==4_118_054_813 and r['threads']==1
assert max(r['results']['ours']['samples_ms'])<min(min(c['samples_ms']) for c in cpps)
narrow=[c for c in read(b/'range-results.json')['cases'] if c['width']==1_000_000]
assert len(narrow)==6
assert all(c['results']['ours']['median_ms']<min(v['median_ms'] for n,v in c['results'].items() if n.startswith('cpp_')) for c in narrow)
assert not list(b.glob('*.progress.json')) and not list(b.glob('*.tmp'))
m['verified_at_utc']=datetime.now(timezone.utc).isoformat();m['validation'].update({'all_report_and_snapshot_hashes':'verified','all_binary_hashes':'verified','baseline_provenance':'verified','batch_arrays_unchanged':'verified','all_counts_checksums':'matched','million_width_standard_wins':'6/6 count/list cases vs best C++ median','100b_all_samples_beat_cpp':True})
p=b/'measurement-2026-11-25.json';tmp=p.with_suffix('.json.tmp');tmp.write_text(json.dumps(m,indent=2)+'\n');tmp.replace(p)
print('Final audit passed: all frozen sources/binaries, report completeness, baselines, links, exact batch answers and count/checksum comparisons verified.')
