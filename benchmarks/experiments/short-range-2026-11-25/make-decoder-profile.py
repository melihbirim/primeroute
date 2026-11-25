from pathlib import Path
import shutil
base=Path('/Users/melihbirim/code/prime-numbers');d=base/'benchmarks/experiments/short-range-2026-11-25/decoder-profile';d.mkdir(exist_ok=True)
for p in [*Path('src').glob('*.rs'),Path('Cargo.toml'),Path('Cargo.lock')]:
 dest=d/p;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(base/p,dest)
p=d/'src/lib.rs';s=p.read_text();old=Path('/private/tmp/prime-short-bounded.rs').read_text();a=old.index('fn append_primes(primes: &mut Vec<usize>');b=old.index('\n#[cfg(all(target_arch = "aarch64"',a);s+='\n'+old[a:b].replace('fn append_primes(','fn append_unreserved(')
s+='''
fn append_plain(primes: &mut Vec<usize>, low:usize, sieve:&[u8]) {
    for(i,&byte) in sieve.iter().enumerate() {
        let mut bits=byte;
        while bits!=0 { primes.push(30*(low+i)+RESIDUES[bits.trailing_zeros() as usize]); bits&=bits-1; }
    }
}
fn append_linear(primes: &mut Vec<usize>, low:usize, sieve:&[u8]) {
    let total=count_bits(sieve); let initial=primes.len();primes.reserve(total);
    let mut written=0; let mut chunks=sieve.chunks_exact(8);
    for(i,chunk) in chunks.by_ref().enumerate() {
        let mut bits=u64::from_le_bytes(chunk.try_into().unwrap());let base=30*(low+8*i);
        while bits!=0 {
            unsafe{*primes.as_mut_ptr().add(initial+written)=base+WORD_OFFSETS[bits.trailing_zeros() as usize];}
            written+=1;bits&=bits-1;
        }
    }
    unsafe{primes.set_len(initial+written);}
    append_plain(primes,low+8*(sieve.len()/8),chunks.remainder());
}
#[test]
#[ignore]
fn decoder_profile() {
 use std::time::Instant; use std::hint::black_box;
 for start in [1_000_000_000usize,100_000_000_000,1_000_000_000_000] {
   sieve_segments(start,start+999_999,|low,sieve| {
     let mut expected=Vec::new();append_plain(&mut expected,low,sieve);
     let mut out=Vec::with_capacity(expected.len()+64);
     for round in 0..7 {
       for offset in 0..5 {let which=(offset+round)%5;
         out.clear();match which {0=>append_primes(&mut out,low,sieve),1=>append_unreserved(&mut out,low,sieve),2=>unsafe{append_primes_neon(&mut out,low,sieve)},3=>append_plain(&mut out,low,sieve),_=>append_linear(&mut out,low,sieve)}; assert_eq!(out,expected);
         let t=Instant::now();for _ in 0..100 {out.clear();match which{0=>append_primes(&mut out,low,sieve),1=>append_unreserved(&mut out,low,sieve),2=>unsafe{append_primes_neon(&mut out,low,sieve)},3=>append_plain(&mut out,low,sieve),_=>append_linear(&mut out,low,sieve)};black_box(&out);}
         println!("start={} decoder={} round={} us={:.3}",start,which,round,t.elapsed().as_secs_f64()*1e4);
       }
     }
   });
 }
}
''';p.write_text(s)
