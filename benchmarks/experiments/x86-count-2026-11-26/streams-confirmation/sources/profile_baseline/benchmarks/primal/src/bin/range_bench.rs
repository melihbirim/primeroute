//! Primal 0.3.3 exposes prefix sieves and primality tests, but no range sieve.
//! `direct` uses its public primality test; `segmented` is an independent,
//! conventional odd-only adapter using primal's public seed sieve.
use std::hint::black_box;
use std::time::Instant;

fn query(start: usize, end: usize, generate: bool, direct: bool) -> (usize, u64) {
    let (start, end) = (black_box(start), black_box(end));
    let mut output = if generate {
        Vec::with_capacity(
            ((end - start + 1) as f64 / (end.max(3) as f64).ln() * 1.2 + 64.0) as usize,
        )
    } else {
        Vec::new()
    };
    let mut count = 0;
    let mut emit = |p| {
        count += 1;
        if generate {
            output.push(p);
        }
    };
    if start <= 2 && end >= 2 {
        emit(2);
    }
    let first = start.max(3) | 1;
    if first <= end {
        if direct {
            for n in (first..=end).step_by(2) {
                if (n <= 5 || (!n.is_multiple_of(3) && !n.is_multiple_of(5)))
                    && primal::is_prime(n as u64)
                {
                    emit(n);
                }
            }
        } else {
            // Bound benchmark resource use explicitly, never silently replace
            // this adapter with another algorithm at large starts.
            let root = end.isqrt();
            assert!(
                root <= 1 << 24,
                "segmented adapter benchmark supports sqrt(end) <= 2^24"
            );
            let seeds = primal::Sieve::new(root);
            let mut states = Vec::new();
            for p in seeds.primes_from(3) {
                let mut multiple = (p * p).max(first.div_ceil(p) * p);
                if multiple.is_multiple_of(2) {
                    multiple += p;
                }
                if multiple <= end {
                    states.push((p, (multiple - first) / 2));
                }
            }
            let total = (end - first) / 2 + 1;
            let mut segment = vec![1u8; total.min(64 * 1024)];
            let mut low = 0;
            while low < total {
                let len = segment.len().min(total - low);
                let sieve = &mut segment[..len];
                sieve.fill(1);
                for (p, next) in &mut states {
                    let mut i = *next;
                    while i < low + len {
                        sieve[i - low] = 0;
                        i += *p;
                    }
                    *next = i;
                }
                for (i, &is_prime) in sieve.iter().enumerate() {
                    if is_prime != 0 {
                        emit(first + 2 * (low + i));
                    }
                }
                low += len;
            }
        }
    }
    let checksum = output
        .iter()
        .fold(0u64, |sum, &p| sum.wrapping_add(p as u64));
    (count, checksum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 5 {
        return Err("Usage: range_bench START END count|generate REPEATS direct|segmented".into());
    }
    let start: usize = args[0].parse()?;
    let end: usize = args[1].parse()?;
    let generate = match args[2].as_str() {
        "count" => false,
        "generate" => true,
        _ => return Err("Expected count or generate".into()),
    };
    let repeats: usize = args[3].parse()?;
    let direct = match args[4].as_str() {
        "direct" => true,
        "segmented" => false,
        _ => return Err("Expected direct or segmented".into()),
    };
    if start > end || end == usize::MAX || repeats == 0 {
        return Err("Invalid range or repeat count".into());
    }
    let expected = black_box(query(start, end, generate, direct));
    let timer = Instant::now();
    for _ in 0..repeats {
        assert_eq!(black_box(query(start, end, generate, direct)), expected);
    }
    let ns = timer.elapsed().as_nanos();
    println!(
        "{{\"elapsed_ns\":{ns},\"repeats\":{repeats},\"count\":{},\"checksum\":{},\"threads\":1}}",
        expected.0, expected.1
    );
    Ok(())
}
