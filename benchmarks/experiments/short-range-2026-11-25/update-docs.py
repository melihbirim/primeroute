from pathlib import Path
import json,re,hashlib,statistics
root=Path('/Users/melihbirim/code/prime-numbers'); b=root/'benchmarks'; exp=b/'experiments/short-range-2026-11-25'; base=exp/'baseline'
load=lambda f:json.loads((b/f).read_text())
manifest=load('measurement-2026-11-25.json');assert manifest['status']=='complete'
sha=hashlib.sha256((root/'src/lib.rs').read_bytes()).hexdigest();assert manifest['source_sha256']['src/lib.rs']==sha
ranges=load('range-results.json');short=load('short-range-results.json');hold=load('short-range-holdout-results.json');high=load('high-range-results.json');batch=load('batch-next-results.json');second=load('batch-next-cluster-holdout-results.json');mem=load('high-range-memory.json')
for r in [ranges,short,hold,high,batch,second,mem]:assert r['status']=='complete' and r['source_sha256']['src/lib.rs']==sha
prefix=[load(f) for f in ['results.json','results-1b.json','results-10b.json','results-100b.json']]
for r in prefix:assert r['source_sha256']['src/lib.rs']==sha
median=lambda c,n:c['results'][n]['median_ms']
cpp=lambda c:min(v['median_ms'] for n,v in c['results'].items() if n.startswith(('cpp_','primesieve_')))
bcpp=lambda c:min(v['median_ms'] for n,v in c['results'].items() if n.startswith('primesieve'))
table=lambda h,rows:'| '+' | '.join(h)+' |\n| '+' | '.join(['---']+['---:']*(len(h)-1))+' |\n'+''.join('| '+' | '.join(map(str,row))+' |\n' for row in rows)
lookup=lambda r,s,w,m:next(c for c in r['cases'] if c['start']==s and c['width']==w and c['mode']==m)
percent=lambda a,z:100*(1-a/z)

# Prefix report, retaining the existing build instructions.
p=b/'README.md';reproduce=p.read_text().split('## Reproduce\n',1)[1]
rows=[]
for name,label in [('ours','Our Rust'),('primesieve_cpp','C++ default'),('primesieve_cpp_tuned','C++ selected cache size'),('primal_rust','Rust primal 0.3.3')]:
 rows.append([label]+[f"{median(r,name):.3f} ms" for r in prefix[:3]])
r=prefix[3];ours=median(r,'ours');cp=cpp(r);old=median(r,'ours_baseline')
text='# Single-threaded prefix-counting benchmarks\n\nMeasured on **2026-11-25, Apple M2 Pro**, with the frozen short-range-optimized source.\nAll programs perform full sieving to the same inclusive limit. Times include\nprocess startup, setup, computation and output capture; compilation is excluded.\n\n'+table(['Implementation','100 million','1 billion','10 billion'],rows)
text+='\n## 100 billion\n\n'+table(['Implementation','Median'],[[label,f"{median(r,name)/1000:.3f} s"] for name,label in [('ours','Our Rust'),('primesieve_cpp','C++ default'),('primesieve_cpp_tuned','C++ selected cache size'),('primal_rust','Rust primal 0.3.3'),('ours_baseline','Previous Rust (before short-range changes)')]])
text+=f'\nRust uses **{percent(ours,cp):.1f}% less time than the faster C++ median**.\nThe preserved previous Rust executable is measured in the same rounds. These\nchanges target short ranges; the 100-billion comparison checks that their\nlong-counting advantage is retained.\n\n'
text+='## Method and validation\n\nThrough ten billion, five warm-up rounds precede 30 measured rounds. At\n100 billion there is one warm-up and five measurements. Commands rotate and\nreverse order between rounds. C++ explicitly uses `--threads=1`; both Rust\nprograms spawn no workers. No CPU affinity is set, and native CPU/cache detection\nis available. Both C++ default and an explicit cache size are timed: 256 KiB at\n100 million, 1024 KiB at larger limits. Comparisons use the faster **median**,\nnot the fastest individual sample.\n\n'+table(['Inclusive limit','Verified prime count'],[[f"{r['limit_inclusive']:,}",f"{r['prime_count']:,}"] for r in prefix])
text+='\nKnown counts are validation data only. No timed implementation returns a stored\ncount. The new library embeds approximately **1.99 MiB of fixed seed/alignment\nmetadata** (primes through 2^20, exact integer reciprocals, and dense templates\nthrough 2^18). This moves bounded seed preprocessing to compilation and enlarges\nthe binary. Runtime marking, result production, and all allocations are timed.\nThere is no computed-answer cache. See [RANGES.md](RANGES.md) for this tradeoff and\nthe short-range implementation; the long dense/medium loops retain the previous\noptimized design.\n\nAll 28 tests pass in debug and release; formatting and Clippy pass for both\ncrates. Close medians on one ARM machine do not establish an all-input or\nall-platform win. Several wide range/list cases still favor C++; the full range\nreport keeps those results visible.\n\n## Raw data and source identity\n\n'
for f,label in [('results.json','100 million'),('results-1b.json','one billion'),('results-10b.json','ten billion'),('results-100b.json','100 billion')]:text+=f'- [{f}]({f}): {label}.\n'
text+=f'\nAll four measure `src/lib.rs` SHA-256 `{sha}`.\nThe [manifest](measurement-2026-11-25.json) records every command and the complete\nsource snapshot. Sources and binaries are frozen and checked before completed\nreports are published atomically. The immediate previous source, executable,\nreports and documentation are retained in\n[the short-range experiment archive](experiments/short-range-2026-11-25/README.md).\n\n## Reproduce\n'+reproduce
p.write_text(text)

# Ranges: all ordinary count/list cases and primal comparators.
listrows=[];countrows=[]
for start in [10**9,10**11,10**12]:
 for width in [10**6,10**8]:
  for mode,rows in [('generate',listrows),('count',countrows)]:
   c=lookup(ranges,start,width,mode)
   rows.append([f'{start:,}',f'{width:,}',f"{median(c,'ours_baseline'):.3f}",f"{median(c,'ours'):.3f}",f'{cpp(c):.3f}',f"{median(c,'primal_segmented_adapter'):.3f}"])
wins=lambda r,mode=None:sum(median(c,'ours')<cpp(c) for c in r['cases'] if mode is None or c['mode']==mode)
text='# Fresh range-query benchmarks\n\nMeasured on **2026-11-25, Apple M2 Pro**, single threaded, against primesieve\n12.18 and primal 0.3.3. Each query creates its own bitmap and marking state.\nThe tables show milliseconds per fresh query, including setup, allocation,\nresult checksums and vector destruction. Compilation and process startup are\nexcluded from these native API timings.\n\n## Standard ranges\n\nThe previous Rust executable is remeasured in the same interleaved rounds, so\nthese before/after columns do not reuse historical timing numbers.\n\n### Materialized prime lists\n\n'+table(['Start','Width','Previous Rust','New Rust','Best C++','Primal adapter'],listrows)
text+='\n### Counts\n\n'+table(['Start','Width','Previous Rust','New Rust','Best C++','Primal adapter'],countrows)
text+=f'\nRust has lower medians in **{wins(ranges)}/12 standard cases**: '
text+=f"**{wins(ranges,'count')}/6 counts** and **{wins(ranges,'generate')}/6 lists**.\n"
narrow=[c for c in ranges['cases'] if c['width']==10**6]
text+=f"The three million-number intervals account for {sum(median(c,'ours')<cpp(c) for c in narrow)}/6 wins across count and list operations.\n"
text+='The earlier short-range regressions are compared with the preserved kernel in\nthe tables above. Wide intervals remain a separate limitation; all losses are\nretained here. Small gaps should be interpreted with the raw sample ranges.\n\nThirty measurements follow five discarded warm-up batches. Each sample times\nthree fresh queries, after one untimed query inside its process. C++ default,\n64, 256 and 1024 KiB settings are all timed; **Best C++** is the fastest median\namong those settings. The benchmark does not select the fastest sample.\n\n## Primal comparison\n\nPrimal 0.3.3 has no public start/end segmented-sieve API. The **Primal adapter**\nis an independent conventional odd-only segmented range implementation that\nuses `primal::Sieve::new(sqrt(end))` to generate seeds. Its seed generation,\nalignment and marking are all timed. It is benchmark code, not a native primal\nrange-sieve feature or a claim about the best possible primal-based adapter.\nThe adapter is limited to roots at most 2^24 to bound benchmark resources.\n\nThe separate short suite also scans candidates using the public exact\n`primal::is_prime` API, skipping multiples of 2, 3 and 5. It uses seven measured\nsamples after one discarded warm-up, one fresh query per sample, and the same\ninterleaving and count/checksum validation. This exposes both the public-API\ncomparison and the faster segmented adapter instead of conflating them.\n\n'
shortrows=[]
for s in [10**9,10**11,10**12]:
 for w in [1000,10000,100000,1000000]:
  c=lookup(short,s,w,'count');g=lookup(short,s,w,'generate')
  shortrows.append([f'{s:,}',f'{w:,}',f"{median(c,'ours'):.3f}",f'{cpp(c):.3f}',f"{median(g,'ours'):.3f}",f'{cpp(g):.3f}',f"{median(g,'primal_segmented_adapter'):.3f}",f"{median(g,'primal_direct'):.3f}"])
text+=table(['Start','Width','Rust count','C++ count','Rust list','C++ list','Adapter list','Primal direct list'],shortrows)
text+=f'\nRust has lower medians than C++ in **{wins(short)}/{len(short["cases"])} short-suite cases**.\n'
text+='All direct primal and adapter count/list samples agree with both sieves.\n\n## Unaligned holdout\n\nDifferent starts and widths (137, 65,537, 1,000,033 and 4,194,319) check both\nsmall intervals and the transition beyond one bitmap segment. These use fifteen\nmeasurements after three warm-ups, with two fresh queries per sample. C++\ndefault, 64 and 1024 KiB settings are measured.\n\n'
text+=f'Rust has lower medians in **{wins(hold)}/{len(hold["cases"])} holdout cases**.\n'
loss=[c for c in hold['cases'] if median(c,'ours')>=cpp(c)]
if loss:text+='\nAll holdout cases still behind C++:\n\n'+table(['Start','Width','Operation','Rust','Best C++'],[[f"{c['start']:,}",f"{c['width']:,}",c['mode'],f"{median(c,'ours'):.3f}",f'{cpp(c):.3f}'] for c in loss])
else:text+='There are no C++ losses in this measured holdout; this is not a universal guarantee.\n'
text+='\n## High-start regression check\n\nSeven measurements follow one warm-up; one fresh query is timed per sample.\nC++ default and 1024 KiB settings are compared. These include the bucketed path\nas well as single-window queries. Primal is omitted here because the adapter\nresource limit does not cover all starts, and scanning forty million numbers\nwith primality tests is a different workload.\n\n'
rows=[]
for s in [10**14,10**15,10**18]:
 for w in [10**6,40*10**6]:
  c=lookup(high,s,w,'count');g=lookup(high,s,w,'generate')
  rows.append([f'{s:,}',f'{w:,}',f"{median(c,'ours'):.3f}",f'{cpp(c):.3f}',f"{median(g,'ours'):.3f}",f'{cpp(g):.3f}'])
text+=table(['Start','Width','Rust count','Best C++ count','Rust list','Best C++ list'],rows)
text+='\nThese measurements keep high-start and wide-range losses visible; they do not\nestablish an advantage at every magnitude or width.\n\n## Implementation and memory tradeoff\n\nThe long counting kernel remains the segmented mod-30 sieve with 128 KiB\nwindows, expanded masks for requests of at least eight windows, ordered dense\nstreams, packed medium wheel steps and bucketed large primes.\n\nThe single-window path adds:\n\n- A fused pass over the seventeen compact pre-sieve patterns.\n- Compiler-generated seed primes through 2^20 and exact integer reciprocal\n  metadata. Multiply-high estimates plus remainder corrections replace repeated\n  seed alignment divisions; no floating-point approximation is used.\n- Dense-prime templates through 2^18, aligned per query. On AArch64, two NEON\n  table lookups rotate the eight offsets into increasing order; portable slice\n  rotation remains available. Short requests retain more dense streams where\n  that is cheaper than walking sparse transitions.\n- Read-only final-window marking: sorted tails are unrolled, and the state is\n  not advanced for a nonexistent next segment. Large primes with at most one or\n  two remaining hits avoid preparing further transitions.\n\nThe fixed seed/alignment arrays occupy **2,088,300 bytes (1.99 MiB)** before\nlinker alignment, in addition to existing mask and decoder tables. They enlarge\nthe executable and move bounded preprocessing to compilation. They contain seed\nmetadata, not benchmark answers. Bitmap construction, prime elimination, counting\nand list generation still run for every call. Roots above 2^20 still stream\nseeds. No computed answer or mutable prepared-state cache persists between calls.\n\nSeparate macOS `/usr/bin/time -l` observations near 10^18 measure process peak\nRSS. Each process makes one untimed query and one timed fresh count; these are\nsingle observations, not medians or allocator caps.\n\n'
rows=[]
for w in [10**6,40*10**6]:
 vals={c['implementation']:c['peak_rss_bytes']/1024**2 for c in mem['cases'] if c['width']==w}
 rows.append([f'{w:,}',f"{vals['ours']:.2f} MiB",f"{vals['cpp_default']:.2f} MiB"])
text+=table(['Width','Rust peak RSS','C++ default peak RSS'],rows)
text+='\n## Validation and reproduction\n\nAll **28 tests** pass in debug and release, with formatting and Clippy passing\nfor both crates. Tests compare static seeds to an independent odd sieve, check\nreciprocal quotients against integer division, compare aligned/rotated offsets\nand final-window bitmaps with independent streams, and use u128 products to\ncheck marking near usize::MAX. Existing prefix, bucket, range, decoder and batch\ntests remain. Native range benchmarks compare exact counts and wrapping-u64\nchecksums, not every output element; batch benchmarks compare full answer arrays.\n\nC++ explicitly sets and reports one thread; Rust and primal spawn no workers.\nRust uses release LTO and one codegen unit. The main crate remains dependency-free.\nNo affinity is set; only this ARM machine was measured. Both list helpers use\nthe same capacity estimate. Primal adapter code is at\n[primal/src/bin/range_bench.rs](primal/src/bin/range_bench.rs).\n\nBuild the helpers as described in [README.md](README.md), then run:\n\n```sh\ncargo build --release --locked --example range_bench\ncargo build --release --locked --manifest-path benchmarks/primal/Cargo.toml --bin range_bench\nclang++ -O3 -std=c++14 -I/path/to/primesieve/include \\\n  benchmarks/range_bench.cpp /path/to/primesieve/build/libprimesieve.a \\\n  -pthread -o /tmp/prime-numbers-cpp-range-bench\npython3 benchmarks/ranges.py --cpp /tmp/prime-numbers-cpp-range-bench \\\n  --primesieve /path/to/primesieve/build/primesieve --primal benchmarks/primal/target/release/range_bench \\\n  --baseline /path/to/previous-range-helper --cpp-sizes 64 256 1024 \\\n  --runs 30 --warmups 5 --repeats 3\n```\n\n'
for f in ['range-results.json','short-range-results.json','short-range-holdout-results.json','high-range-results.json','high-range-memory.json']:text+=f'- [{f}]({f})\n'
text+=f'\nAll reports measure `src/lib.rs` SHA-256 `{sha}`.\nThe [manifest](measurement-2026-11-25.json) records exact commands, versions,\nsource snapshot and validation. Completed reports publish atomically only after\nsource and binary hashes are rechecked. Previous reports and exploratory\nvariants are retained in [the archive](experiments/short-range-2026-11-25/README.md).\n'
(b/'RANGES.md').write_text(text)

# Root README: replace benchmark sections, retain API/build/limits.
p=root/'README.md';s=p.read_text();a=s.index('### Batch next-prime');z=s.index('## How it works') if '## How it works' in s else s.index('## Implementation')
labels={'scattered_1e8':'Scattered near 10⁸','clustered_1e8':'Clustered near 10⁸','scattered_1e12':'Scattered near 10¹²','clustered_1e12':'Clustered near 10¹²','scattered_1e14':'Scattered near 10¹⁴','clustered_1e14':'Clustered near 10¹⁴','packed_1e14':'Packed near 10¹⁴','mixed_1e12':'Mixed near 10¹²','spaced_1e14':'Spaced near 10¹⁴','isolated_1e14':'Isolated near 10¹⁴'}
bwins=sum(median(c,'ours_auto')<bcpp(c) for c in batch['cases']);pwins=sum(median(c,'ours_auto')<median(c,'primal') for c in batch['cases'])
new='### Batch next-prime\n\nMedian milliseconds for the whole batch: sorting, deduplication, routing,\ncomputation and restoring order are timed; parsing, startup and serialization\nare excluded. Seven measurements follow one warm-up.\n\n'+table(['Workload','Queries','Ours','primesieve','primal'],[[labels[c['name']],f"{c['query_count']:,}",f"{median(c,'ours_auto'):.3f}",f'{bcpp(c):.3f}',f"{median(c,'primal'):.3f}"] for c in batch['cases']])
new+=f'\nOurs has lower medians in **{bwins}/10 workloads against primesieve** and\n**{pwins}/10 against primal**. C++ shares continuing iterators across queries; the\nfaster adaptive/continuous median is shown where practical. Primal uses its\npublic exact primality API with the same filter, sorting and answer reuse as\nour direct path. All answer arrays agree. See [the batch report](benchmarks/BATCH_NEXT.md)\nfor workload definitions, controls, the second seed and raw results.\n\n### Short inclusive ranges\n\nEach interval below contains **one million numbers**. Medians are milliseconds\nper fresh query, including setup, allocation and list checksums; 30 measurements\nfollow five warm-ups. C++ is the fastest default/64/256/1024 KiB median.\n\n'
rows=[]
for st in [10**9,10**11,10**12]:
 c=lookup(ranges,st,10**6,'count');g=lookup(ranges,st,10**6,'generate');rows.append([f'{st:,}',f"{median(c,'ours'):.3f}",f'{cpp(c):.3f}',f"{median(g,'ours'):.3f}",f'{cpp(g):.3f}',f"{median(g,'primal_segmented_adapter'):.3f}"])
new+=table(['Start','Rust count','C++ count','Rust list','C++ list','Primal adapter list'],rows)
new+='\nPrimal has no public range-sieve API: the adapter independently segments ranges\nusing its public seed sieve. Its public `is_prime` scanning path is also measured\nin the [full range report](benchmarks/RANGES.md). That report includes before/after\nmedians, additional widths, unaligned holdouts, high starts, and all remaining\nwide-range losses.\n\n### Counting primes up to N\n\nWhole-process medians include startup and output capture. There are five warm-ups\nand 30 measurements through ten billion; at 100 billion, one warm-up and five\nmeasurements. C++ is the faster default/tuned median.\n\n'
rows=[]
for r in prefix:
 scale=1000 if r['limit_inclusive']==100_000_000_000 else 1;unit='s' if scale==1000 else 'ms'
 rows.append([f"{r['limit_inclusive']:,}",f"{median(r,'ours')/scale:.3f} {unit}",f'{cpp(r)/scale:.3f} {unit}',f"{median(r,'primal_rust')/scale:.3f} {unit}"])
new+=table(['Limit','Ours','primesieve','primal'],rows)
new+=f'\nAt 100 billion, Rust uses **{percent(ours,cp):.1f}% less time than primesieve**.\nThe previous Rust executable is remeasured in the same rounds. This is still a\nfull sieve count, with every run returning **4,118,054,813**. See the\n[counting report](benchmarks/README.md) and [raw samples](benchmarks/results-100b.json).\nClose medians and results on one ARM CPU do not establish a universal win.\n\n'
s=s[:a]+new+s[z:];s=s.replace('All **24 tests**','All **28 tests**')
s=s.replace('prepared set; shorter requests cycle directly through the compact patterns.','prepared set. Single-window requests fuse the cyclic masks into one pass;\nother short requests cycle through the compact patterns.')
s=s.replace('mark large primes directly without retaining their state vector.','use read-only sorted dense streams and mark large primes directly without\nretaining their state vector. NEON table lookups rotate the short streams; a\nportable rotation remains available.')
s=s.replace('  They stream sieving primes through `sqrt(limit)`, so very high starts still','  Roots up to 2^20 use compiler-generated seed data. Larger roots stream\n  sieving primes through `sqrt(limit)`, so very high starts still')
s=s.replace('## Limits and tradeoffs\n','## Limits and tradeoffs\n\n- Fixed compiler-generated seed/alignment metadata adds **1.99 MiB** before\n  linker alignment, on top of existing mask and decoder tables. It trades binary\n  size for less per-query setup. These are seed data, not stored query answers;\n  each range is still marked and counted. No mutable answer cache is retained.\n')
s=s.replace('expanded masks are compared with cyclic masks at offsets and tails.','expanded and fused masks are compared with cyclic masks at offsets and tails.\nStatic seeds, exact reciprocal quotients, NEON stream rotation and final-window\nmarking are independently checked, including wide products near usize::MAX.')
p.write_text(s)

# Batch report: replace results section, retain workload/API detail with corrected storage language.
p=b/'BATCH_NEXT.md';s=p.read_text();a=s.index('## Results');z=s.index('## Workloads')
new='## Results\n\nMeasured on **2026-11-25, Apple M2 Pro**, against primesieve 12.18 and primal\n0.3.3, using the frozen short-range-optimized source. Times are milliseconds for\nthe whole batch, including routing and order restoration. Seven measurements\nfollow one warm-up. C++ is the faster adaptive/continuous iterator median.\n\n'
rows=[]
for c in batch['cases']:
 stats=c['results']['ours_auto']['stats'];routes=f"{stats['direct_groups']}/{stats['sieve_groups']}"
 rows.append([c['name'],f"{c['query_count']:,}",f"{median(c,'ours_auto'):.3f}",f"{median(c,'ours_direct'):.3f}",f"{median(c,'primal'):.3f}",f'{bcpp(c):.3f}',routes])
new+=table(['Workload','Queries','Our router','Direct control','primal','primesieve','Routes D/S'],rows)
new+=f'\nThe router has lower medians than C++ in **{bwins}/10 workloads** and primal\nin **{pwins}/10**. Close medians should be read with their sample ranges; these\nare measurements on one ARM machine, not a guarantee for every input or CPU.\n\n### Previous kernel\n\nThe previous frozen source and reports are retained in\n[the archive](experiments/short-range-2026-11-25/README.md). Query and expected\nanswer arrays are identical. These batch comparisons are separate runs, so small\ndifferences include system variation. The range suite remeasures the old range\nexecutable in the same rounds.\n\n'
prev=json.loads((base/'batch-next-results.json').read_text());pm={c['name']:c for c in prev['cases']}
new+=table(['Workload','Previous Rust','Current Rust'],[[c['name'],f"{median(pm[c['name']],'ours_auto'):.3f}",f"{median(c,'ours_auto'):.3f}"] for c in batch['cases'] if c['name'] in ['clustered_1e8','clustered_1e12','clustered_1e14','mixed_1e12']])
new+='\n### Second query seed\n\nSeed 12648432 uses different query values, seven measurements and one warm-up.\n\n'+table(['Workload','Our router','primal','primesieve'],[[c['name'],f"{median(c,'ours_auto'):.3f}",f"{median(c,'primal'):.3f}",f'{bcpp(c):.3f}'] for c in second['cases']])
new+='\nAll full answer arrays agree across Rust, C++ and primal.\n[batch-next-results.json](batch-next-results.json) and\n[batch-next-cluster-holdout-results.json](batch-next-cluster-holdout-results.json)\ncontain all inputs, answers, samples, route stats and source/binary hashes.\nThe [manifest](measurement-2026-11-25.json) records the frozen source snapshot\nand commands. [RANGES.md](RANGES.md) measures the shared range backend separately.\n\n'
s=s[:a]+new+s[z:];s=s.replace('All 24 tests','All 28 tests')
s=s.replace('Code/data pages can be cached, but no computed answers or prepared seed tables\npersist between batches.','Code/data pages can be cached. Computed answers and per-call seed/marking\nstates are discarded; fixed compiler-generated seed metadata is shared read-only.')
s=s.replace('materialized, and no prepared data survives the call.','materialized. Per-call prepared data is discarded; the fixed compiler-generated\nseed metadata remains read-only.')
s=s.replace('Standalone range calls stream seed primes, using a bounded odd sieve for roots\nup to 65,536. Batch groups retain one prepared table per call.','Standalone range calls use static seed primes through 2^20 and stream larger\nroots. Fixed seeds, reciprocals and dense templates occupy about 1.99 MiB.\nBatch groups retain one per-call prepared table. Single-window marking uses\nrotated sorted offsets and discards final state updates; details are in [RANGES.md](RANGES.md).')
s=s.replace('Short requests apply the compact static masks cyclically.','Single-window requests fuse the compact cyclic masks in one pass. Other short\nrequests apply them cyclically.')
p.write_text(s)

(exp/'README.md').write_text('''# Short-range optimization experiments

The baseline directory preserves the previous 100-billion-winning source
(`09abee545c4ac726fcc68c98cb12c0fe3514c7558b760f0bd4500f71db9f906f`),
range/counter executables, reports and documentation. Both executables were
remeasured in the final interleaved runs.

All screening reports are exploratory, not the final performance comparison.
They retain all samples and hashes; saved candidate sources, profile crates and
logs show rejected variants. Some early intermediate variants are represented by
their hashed reports and later saved sources rather than complete build trees.

Selected: bounded compiler-generated seed metadata, exact integer reciprocal
alignment, fused cyclic masks, NEON rotation during dense initialization, a wider
single-window dense crossover, read-only sorted tail marking, and bounded sparse
hit tails. The existing NEON word decoder and long counting loops remain.

Rejected: floating quotient estimates, simple noninterleaved streams, enlarged
210/2310 wheels, larger/fused dense variants that slowed cases, byte decoders,
and scalar word-decoder replacements. Profile timings are selection evidence,
not final benchmark claims.

The fixed extra metadata is 2,088,300 bytes before linker alignment. No query
counts or output lists are precomputed. All timed requests build and mark their
own interval. This is a disclosed binary-size/preprocessing tradeoff.

`final-run.py` reproduces the exact sequential suite commands with local tool
paths adjusted as needed. `memory.py` records separate native peak-RSS points.
Completed canonical reports and source identity are in
[the manifest](../../measurement-2026-11-25.json),
[the range report](../../RANGES.md), and
[the counting report](../../README.md). Previous reports retain their original
identities; no pilot sample was promoted into a final timing result.
''')
print('Updated all reports and project README from frozen final JSON.')
