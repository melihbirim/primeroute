from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,re,subprocess,sys
root=Path('/Users/melihbirim/code/prime-numbers');assert sys.platform=='darwin'
r=json.loads((root/'benchmarks/high-range-results.json').read_text());assert r['status']=='complete'
for name,h in r['source_sha256'].items():assert hashlib.sha256((root/name).read_bytes()).hexdigest()==h
report={'status':'running','started_at_utc':datetime.now(timezone.utc).isoformat(),'platform':r['platform'],'metric':'Peak process RSS in bytes from macOS /usr/bin/time -l; one untimed warm-up query and one timed fresh query per process; single observation per case','source_sha256':r['source_sha256'],'binary_sha256':r['binary_sha256'],'cases':[]}
for width in [1_000_000,40_000_000]:
    case=next(c for c in r['cases'] if c['start']==10**18 and c['width']==width and c['mode']=='count')
    for name in ['ours','cpp_default']:
        cmd=['/usr/bin/time','-l',*case['commands'][name]]
        data=subprocess.run(cmd,text=True,capture_output=True,check=True)
        native=json.loads(data.stdout);assert native['threads']==1 and native['count']==case['count']
        rss=int(re.search(r'(\d+)\s+maximum resident set size',data.stderr).group(1))
        report['cases'].append({'start':case['start'],'end':case['end'],'width':width,'mode':'count','implementation':name,'command':cmd,'peak_rss_bytes':rss,'native_result':native,'time_stderr':data.stderr})
        print(name,width,f'{rss/1024/1024:.2f} MiB peak RSS',flush=True)
for name,h in r['source_sha256'].items():assert hashlib.sha256((root/name).read_bytes()).hexdigest()==h
report['status']='complete';report['completed_at_utc']=datetime.now(timezone.utc).isoformat()
p=root/'benchmarks/high-range-memory.json';tmp=p.with_suffix('.json.tmp');tmp.write_text(json.dumps(report,indent=2)+'\n');tmp.replace(p)
print('Saved high-range-memory.json',flush=True)
