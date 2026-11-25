"""Exploratory C++ cache sweep; final comparisons use fresh samples."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,statistics,subprocess,time
exe=Path('/private/tmp/prime-numbers-primesieve-v12.18/build/primesieve');before=hashlib.sha256(exe.read_bytes()).hexdigest()
r={'kind':'exploratory cache sweep; settings selected for fresh final comparisons','threads':1,'started_at_utc':datetime.now(timezone.utc).isoformat(),'binary_sha256':before,'cases':[]}
for limit,count,runs in [(100_000_000,5_761_455,10),(1_000_000_000,50_847_534,3),(10_000_000_000,455_052_511,2),(100_000_000_000,4_118_054_813,1)]:
    settings=[512,128,1024,64,256,2048];samples={s:[] for s in settings};commands={s:[str(exe),str(limit),'--threads=1','--count=1','--quiet','--no-status',f'--size={s}'] for s in settings}
    for i in range(runs):
        order=settings[i%len(settings):]+settings[:i%len(settings)]
        if i%2:order.reverse()
        for s in order:
            start=time.perf_counter_ns();out=subprocess.run(commands[s],text=True,capture_output=True,check=True);elapsed=(time.perf_counter_ns()-start)/1e6;assert int(out.stdout.strip())==count;samples[s].append(elapsed)
    results={str(s):{'median_ms':statistics.median(t),'samples_ms':t,'command':commands[s]} for s,t in samples.items()}
    best=min(results,key=lambda s:results[s]['median_ms']);r['cases'].append({'limit':limit,'count':count,'results':results,'best_size_kib':int(best)})
    print(limit,'best',best,'KiB',round(results[best]['median_ms'],3),'ms',flush=True)
assert hashlib.sha256(exe.read_bytes()).hexdigest()==before
r['status']='complete';r['completed_at_utc']=datetime.now(timezone.utc).isoformat();Path(__file__).with_name('cpp-cache-screen.json').write_text(json.dumps(r,indent=2)+'\n')
