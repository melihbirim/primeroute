"""Exploratory variant timings, separate from the final comparison."""
import argparse,hashlib,json,statistics,subprocess,time
from pathlib import Path
from datetime import datetime,timezone
p=argparse.ArgumentParser();p.add_argument('--limit',type=int,default=10_000_000_000);p.add_argument('--runs',type=int,default=3);p.add_argument('--warmups',type=int,default=1);p.add_argument('--names',default='baseline,u32-mark,expanded,u32-expanded,u32-branchless,u32-expanded-branchless');p.add_argument('--output',required=True);a=p.parse_args()
folder=Path(__file__).resolve().parent
names=a.names.split(',')
commands={n:[str(folder/n/('prime-numbers' if n=='baseline' else 'target/release/prime-numbers')),str(a.limit)] for n in names}
sources={n:{str(f.relative_to(folder/n)):hashlib.sha256(f.read_bytes()).hexdigest() for f in [*(folder/n/'src').glob('*.rs'),folder/n/'Cargo.toml',folder/n/'Cargo.lock']} for n in names}
binaries={n:hashlib.sha256(Path(c[0]).read_bytes()).hexdigest() for n,c in commands.items()}
samples={n:[] for n in names};count={100_000_000:5_761_455,1_000_000_000:50_847_534,10_000_000_000:455_052_511,100_000_000_000:4_118_054_813}[a.limit]
r={'status':'running','started_at_utc':datetime.now(timezone.utc).isoformat(),'kind':'exploratory; candidate selection, not final performance claim','limit_inclusive':a.limit,'threads':1,'runs':a.runs,'warmups':a.warmups,'commands':commands,'source_sha256':sources,'binary_sha256':binaries,'prime_count':count,'metric':'Process wall time ms, including startup/output capture'}
for i in range(a.warmups+a.runs):
    order=names[i%len(names):]+names[:i%len(names)]
    if i%2:order.reverse()
    for n in order:
        t=time.perf_counter_ns();out=subprocess.run(commands[n],capture_output=True,text=True,check=True);elapsed=(time.perf_counter_ns()-t)/1e6
        assert int(out.stdout.strip())==count,(n,out.stdout)
        if i>=a.warmups:samples[n].append(elapsed)
        print(f'{i+1}/{a.warmups+a.runs} {n}: {elapsed:.3f} ms',flush=True)
for n in names:
    assert hashlib.sha256(Path(commands[n][0]).read_bytes()).hexdigest()==binaries[n]
    for f,h in sources[n].items():assert hashlib.sha256((folder/n/f).read_bytes()).hexdigest()==h
r['results']={n:{'median_ms':statistics.median(v),'min_ms':min(v),'max_ms':max(v),'samples_ms':v} for n,v in samples.items()};r['status']='complete';r['completed_at_utc']=datetime.now(timezone.utc).isoformat()
Path(a.output).write_text(json.dumps(r,indent=2)+'\n')
for n,v in r['results'].items():print(n,round(v['median_ms'],3),'ms',flush=True)
