from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,shutil,subprocess
root=Path('/Users/melihbirim/code/prime-numbers');os.chdir(root)
cpp='/private/tmp/prime-numbers-primesieve-v12.18';cpp_range='/private/tmp/prime-numbers-cpp-range-bench'
base=root/'benchmarks/experiments/short-range-2026-11-25/baseline';primal=str(root/'benchmarks/primal/target/release/range_bench')
env=os.environ.copy();env['CARGO_HOME']='/private/tmp/prime-numbers-benchmark-cargo'
sources=sorted(Path('src').glob('*.rs'))+[Path(p) for p in ['Cargo.toml','Cargo.lock','benchmarks/compare.py','benchmarks/ranges.py','benchmarks/batch_next.py','benchmarks/batch_next.cpp','benchmarks/range_bench.cpp','examples/range_bench.rs','benchmarks/primal/Cargo.toml','benchmarks/primal/Cargo.lock','benchmarks/primal/src/main.rs','benchmarks/primal/src/bin/batch_next.rs','benchmarks/primal/src/bin/range_bench.rs']]
sources += [Path('benchmarks/experiments/short-range-2026-11-25/final-run.py'), Path('benchmarks/experiments/short-range-2026-11-25/memory.py')]
frozen={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
snapshot=Path('benchmarks/snapshots/2026-11-25-short-ranges');snapshot.mkdir(parents=True,exist_ok=True)
for p in sources:
 dest=snapshot/p;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,dest)
commands=[]
def add(name,argv,output):commands.append({'name':name,'command':argv,'output':output})
range_args=['python3','benchmarks/ranges.py','--cpp',cpp_range,'--primesieve',cpp+'/build/primesieve','--baseline',str(base/'range_bench')]
add('Standard ranges including primal adapter',range_args+['--primal',primal,'--runs','30','--warmups','5','--repeats','3','--cpp-sizes','64','256','1024'],'benchmarks/range-results.json')
for limit,out in [(100_000_000_000,'results-100b.json'),(100_000_000,'results.json'),(1_000_000_000,'results-1b.json'),(10_000_000_000,'results-10b.json')]:
 argv=['python3','benchmarks/compare.py','--primesieve',cpp+'/build/primesieve','--cpp-size','256' if limit==100_000_000 else '1024','--progress','--limit',str(limit),'--output','benchmarks/'+out]
 if limit==100_000_000_000:argv+=['--warmups','1','--runs','5','--baseline',str(base/'prime-numbers')]
 add('Prefix count '+str(limit),argv,'benchmarks/'+out)
add('Short ranges and primal public primality API',range_args+['--primal',primal,'--primal-direct','--runs','7','--warmups','1','--repeats','1','--widths','1000','10000','100000','1000000','--cpp-sizes','64','256','1024','--output','benchmarks/short-range-results.json'],'benchmarks/short-range-results.json')
add('Unaligned range holdout',range_args+['--primal',primal,'--runs','15','--warmups','3','--repeats','2','--starts','1001234567','100007654321','1000023456789','--widths','137','65537','1000033','4194319','--cpp-sizes','64','1024','--output','benchmarks/short-range-holdout-results.json'],'benchmarks/short-range-holdout-results.json')
add('High-start range regression check',range_args+['--runs','7','--warmups','1','--repeats','1','--starts','100000000000000','1000000000000000','1000000000000000000','--widths','1000000','40000000','--output','benchmarks/high-range-results.json'],'benchmarks/high-range-results.json')
add('Batch next-prime',['python3','benchmarks/batch_next.py','--primesieve-dir',cpp,'--seed','12648431','--runs','7','--warmups','1','--include-sieve'],'benchmarks/batch-next-results.json')
add('Second batch seed',['python3','benchmarks/batch_next.py','--primesieve-dir',cpp,'--seed','12648432','--runs','7','--warmups','1','--include-sieve','--only','scattered_1e8,clustered_1e8,clustered_1e12,clustered_1e14,mixed_1e12','--output','benchmarks/batch-next-cluster-holdout-results.json'],'benchmarks/batch-next-cluster-holdout-results.json')
add('High-range memory',['python3','benchmarks/experiments/short-range-2026-11-25/memory.py'],'benchmarks/high-range-memory.json')
manifest={'status':'running','started_at_utc':datetime.now(timezone.utc).isoformat(),'source_sha256':frozen,'source_snapshot':str(snapshot),'runtime_threads':1,'optimization':'bounded static seed metadata, exact integer reciprocal alignment, fused cyclic masks, NEON ordered short streams and read-only final-window marking','fixed_seed_metadata_bytes':82025*12+23000*48,'baseline_source_snapshot':str(base.relative_to(root)),'runs':commands,'completed_runs':[],'validation':{'debug_tests':28,'release_tests':28,'clippy':'both crates, all targets, warnings denied','primal_crosscheck':'passed'},'build_commands':[['cargo','build','--release','--locked','--example','range_bench','--bin','prime-numbers'],['cargo','build','--offline','--release','--locked','--manifest-path','benchmarks/primal/Cargo.toml','--bins']]}
mp=Path('benchmarks/measurement-2026-11-25.json')
def save():
 temp=mp.with_suffix('.json.tmp');temp.write_text(json.dumps(manifest,indent=2)+'\n');temp.replace(mp)
save()
for task in commands:
 print('\nStarting '+task['name'],flush=True);subprocess.run(task['command'],env=env,check=True)
 assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in frozen.items()),'Source changed during final run'
 report=json.loads(Path(task['output']).read_text());assert report['status']=='complete';assert report['source_sha256']['src/lib.rs']==frozen['src/lib.rs']
 manifest['completed_runs'].append(task['name']);save()
manifest['status']='complete';manifest['completed_at_utc']=datetime.now(timezone.utc).isoformat();save()
print('\nAll benchmark suites complete and source hashes verified.',flush=True)
