"""Paired native benchmarks. Keeps old reports immutable and records every sample."""
import argparse
import hashlib
import json
import os
import platform
import statistics
import subprocess
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--baseline', type=Path, required=True)
p.add_argument('--v3', type=Path)
p.add_argument('--primesieve-dir', type=Path, required=True)
p.add_argument('--cpp-range', type=Path, required=True)
p.add_argument('--cpp-batch', type=Path, required=True)
p.add_argument('--output', type=Path, required=True)
p.add_argument('--runs', type=int, default=15)
p.add_argument('--warmups', type=int, default=3)
p.add_argument('--large-runs', type=int, default=3)
p.add_argument('--large', action='store_true')
a = p.parse_args()
assert a.runs > 0 and a.warmups >= 0 and a.large_runs > 0
affinity_before = sorted(os.sched_getaffinity(0)) if hasattr(os,'sched_getaffinity') else None
if affinity_before:
    os.sched_setaffinity(0, {affinity_before[0]})
roots = {'baseline': a.baseline.resolve(), 'ours': ROOT}
if a.v3: roots['baseline_v3'] = a.v3.resolve()
cpp = a.primesieve_dir.resolve() / 'build/primesieve'
cpp_range, cpp_batch = a.cpp_range.resolve(), a.cpp_batch.resolve()
primal = ROOT / 'benchmarks/primal/target/release'
programs = {'cpp': cpp, 'cpp_range': cpp_range, 'cpp_batch': cpp_batch,
            'primal_count': primal/'primal-benchmark', 'primal_range': primal/'range_bench', 'primal_batch': primal/'batch_next'}
for name, root in roots.items():
    programs[name+'_count'] = root/'target/release/primeroute'
    programs[name+'_range'] = root/'target/release/examples/range_bench'
    programs[name+'_batch'] = root/'benchmarks/primal/target/release/batch_next'
if a.v3:
    programs['primal_v3_count'] = a.v3.resolve()/'benchmarks/primal/target/release/primal-benchmark'
    programs['primal_v3_range'] = a.v3.resolve()/'benchmarks/primal/target/release/range_bench'
    programs['primal_v3_batch'] = a.v3.resolve()/'benchmarks/primal/target/release/batch_next'
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
source_paths = {}
for name, root in roots.items():
    files = [*sorted((root/'src').glob('*.rs')),root/'Cargo.toml',root/'Cargo.lock',root/'examples/range_bench.rs',
             root/'benchmarks/primal/Cargo.toml',root/'benchmarks/primal/Cargo.lock',*sorted((root/'benchmarks/primal/src').rglob('*.rs'))]
    source_paths[name] = {str(f.relative_to(root)): f for f in files}
report = {'status':'running','started_at_utc':datetime.now(timezone.utc).isoformat(),
          'platform':platform.platform(),'machine':platform.machine(),'threads':1,
          'runs':a.runs,'warmups':a.warmups,'large_runs':a.large_runs,
          'source_sha256':{n:{key:sha(path) for key,path in files.items()} for n,files in source_paths.items()},
          'runner_sha256':sha(Path(__file__)),
          'input_sha256':sha(ROOT/'benchmarks/batch-next-results.json'),
          'cpp_sources_sha256':{f:sha(ROOT/'benchmarks'/f) for f in ['range_bench.cpp','batch_next.cpp']},
          'clang':subprocess.check_output(['clang++','--version'],text=True),'binary_sha256':{n:sha(f) for n,f in programs.items()},
          'programs':{n:str(f) for n,f in programs.items()},
          'build_flags':{n:('-C target-cpu=x86-64-v3' if n=='baseline_v3' else 'default portable target') for n in roots},
          'primal_v3_flags':'-C target-cpu=x86-64-v3' if a.v3 else None,
          'rustc':subprocess.check_output(['rustc','-Vv'],text=True),
          'cpu_info':subprocess.check_output([str(cpp),'--cpu-info'],text=True),
          'primesieve_version':subprocess.check_output([str(cpp),'--version'],text=True).strip(),
          'affinity_before':affinity_before,
          'affinity':sorted(os.sched_getaffinity(0)) if hasattr(os,'sched_getaffinity') else None,
          'cases':[]}
if platform.system()=='Linux':report['lscpu']=subprocess.check_output(['lscpu','--json'],text=True)
a.output.parent.mkdir(parents=True,exist_ok=True)
progress=a.output.with_name(a.output.stem+'.progress.json')
def save(path):
    tmp=path.with_name(path.name+'.tmp');tmp.write_text(json.dumps(report,indent=2)+'\n');tmp.replace(path)
def measure(kind, label, commands, metadata, expected, repeats=1, runs=None, warmups=None, inputs=None):
    runs=a.runs if runs is None else runs;warmups=a.warmups if warmups is None else warmups
    samples={n:[] for n in commands};stats={}
    for i in range(warmups+runs):
        names=list(commands);shift=i%len(names);names=names[shift:]+names[:shift]
        if i%2:names.reverse()
        for name in names:
            t=time.perf_counter_ns()
            proc=subprocess.run(commands[name],input=inputs,capture_output=True,text=True,check=True)
            wall=(time.perf_counter_ns()-t)/1e6
            if kind=='prefix':answer=int(proc.stdout.strip());elapsed=wall
            else:
                data=json.loads(proc.stdout);assert data['threads']==1,(label,name)
                if kind=='range':answer=(data['count'],data['checksum']);elapsed=data['elapsed_ns']/data['repeats']/1e6
                else:answer=data['answers'];elapsed=data['elapsed_ns']/1e6;stats[name]={k:v for k,v in data.items() if k not in ['answers','elapsed_ns']}
            assert answer==expected,(label,name,'answer mismatch')
            if i>=warmups:samples[name].append(elapsed)
    case={'kind':kind,'label':label,**metadata,'runs':runs,'warmups':warmups,'fresh_queries_per_sample':repeats,
          'commands':commands,'results':{n:{'median_ms':statistics.median(v),'min_ms':min(v),'max_ms':max(v),'samples_ms':v} for n,v in samples.items()}}
    if kind=='prefix':case['prime_count']=expected
    elif kind=='range':case['count'],case['checksum']=expected
    else:case['queries']=metadata['queries'];case['answers']=expected;case['stats']=stats
    report['cases'].append(case);save(progress)
    print(label+': '+', '.join(f"{n}={v['median_ms']:.3f}ms" for n,v in case['results'].items()),flush=True)

for limit,count in [(10**8,5_761_455),(10**9,50_847_534),(10**10,455_052_511)]+([(10**11,4_118_054_813)] if a.large else []):
    large=limit==10**11
    commands={n:[str(programs[n+'_count']),str(limit)] for n in roots if not large or n!='baseline_v3'}
    commands['cpp_default']=[str(cpp),str(limit),'--threads=1','--count=1','--quiet','--no-status']
    if not large:commands['cpp_1024k']=commands['cpp_default']+['--size=1024']
    commands['primal']=[str(programs['primal_count']),str(limit)]
    if a.v3 and not large:commands['primal_v3']=[str(programs['primal_v3_count']),str(limit)]
    measure('prefix',f'count through {limit}',commands,{'limit':limit,'metric':'process wall time ms'},count,
            runs=a.large_runs if large else None,warmups=1 if large else None)
for start in [10**9,10**11,10**12]:
    for width in [10**6,10**8]:
        end=start+width-1
        for mode in ['count','generate']:
            common=[str(start),str(end),mode,'3']
            commands={n:[str(programs[n+'_range']),*common] for n in roots}
            commands.update({f'cpp_{size}':[str(cpp_range),*common,str(size)] for size in [0,1024]})
            commands['primal']=[str(programs['primal_range']),*common,'segmented']
            if a.v3:commands['primal_v3']=[str(programs['primal_v3_range']),*common,'segmented']
            reference=json.loads(subprocess.check_output(commands['cpp_0'],text=True))
            measure('range',f'{mode} [{start}, {end}]',commands,{'start':start,'end':end,'width':width,'mode':mode,
                'metric':'in-process ms per fresh query, setup and checksum included'},(reference['count'],reference['checksum']),repeats=3)
for case in json.loads((ROOT/'benchmarks/batch-next-results.json').read_text())['cases']:
    queries=case['queries'];inputs='\n'.join(map(str,queries))+'\n'
    commands={n:[str(programs[n+'_batch']),'auto'] for n in roots}
    commands['cpp_adaptive']=[str(cpp_batch),'adaptive']
    if max(queries)-min(queries)<=200_000_000:commands['cpp_stream']=[str(cpp_batch),'stream']
    commands['primal']=[str(programs['primal_batch']),'primal']
    if a.v3:commands['primal_v3']=[str(programs['primal_v3_batch']),'primal']
    measure('batch',case['name'],commands,{'queries':queries,'query_count':len(queries),'metric':'in-process whole batch ms'},case['answers'],runs=5,warmups=1,inputs=inputs)
for name,files in source_paths.items():
    assert all(sha(path)==report['source_sha256'][name][key] for key,path in files.items()),('source changed',name)
for name,path in programs.items():assert sha(path)==report['binary_sha256'][name],('binary changed',name)
assert sha(Path(__file__))==report['runner_sha256']
assert sha(ROOT/'benchmarks/batch-next-results.json')==report['input_sha256']
report['status']='complete';report['completed_at_utc']=datetime.now(timezone.utc).isoformat();save(a.output);progress.unlink()
print('All answers/counts/checksums and source/binary freezes verified.',flush=True)
