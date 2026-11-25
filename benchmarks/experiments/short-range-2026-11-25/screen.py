import argparse,hashlib,json,statistics,subprocess
from datetime import datetime,timezone
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--output',required=True);p.add_argument('--runs',type=int,default=9);p.add_argument('--repeats',type=int,default=10);p.add_argument('--wide',action='store_true');a=p.parse_args()
root=Path('/Users/melihbirim/code/prime-numbers');exp=root/'benchmarks/experiments/short-range-2026-11-25';cpp='/private/tmp/prime-numbers-cpp-range-bench';ours=str(root/'target/release/examples/range_bench');base=str(exp/'baseline/range_bench')
sources={str(f.relative_to(root)):hashlib.sha256(f.read_bytes()).hexdigest() for f in (root/'src').glob('*.rs')};binary={n:hashlib.sha256(Path(f).read_bytes()).hexdigest() for n,f in [('ours',ours),('baseline',base),('cpp',cpp)]}
r={'status':'running','kind':'exploratory variant screen','timestamp':datetime.now(timezone.utc).isoformat(),'source_sha256':sources,'binary_sha256':binary,'runs':a.runs,'repeats':a.repeats,'threads':1,'cases':[]}
for start in [10**9,10**11,10**12]:
 for width in ([10**6,10**8] if a.wide else [10**6]):
  for mode in ['count','generate']:
   common=[str(start),str(start+width-1),mode,str(a.repeats)];commands={'ours':[ours,*common],'baseline':[base,*common],'cpp':[cpp,*common,'0']};samples={n:[] for n in commands};expected=None
   for i in range(a.runs+2):
    names=list(commands);names=names[i%3:]+names[:i%3]
    if i%2:names.reverse()
    for n in names:
     v=json.loads(subprocess.check_output(commands[n],text=True));answer=(v['count'],v['checksum']);expected=answer if expected is None else expected;assert expected==answer and v['threads']==1
     if i>=2:samples[n].append(v['elapsed_ns']/v['repeats']/1e6)
   case={'start':start,'width':width,'mode':mode,'count':expected[0],'checksum':expected[1],'commands':commands,'results':{n:{'median_ms':statistics.median(v),'samples_ms':v} for n,v in samples.items()}};r['cases'].append(case);print(start,width,mode,{n:round(v['median_ms'],4) for n,v in case['results'].items()},flush=True)
for f,h in sources.items():assert hashlib.sha256((root/f).read_bytes()).hexdigest()==h
r['status']='complete';Path(a.output).write_text(json.dumps(r,indent=2)+'\n')
