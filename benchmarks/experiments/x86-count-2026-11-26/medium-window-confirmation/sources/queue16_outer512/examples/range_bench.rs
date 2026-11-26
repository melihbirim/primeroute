use std::hint::black_box;
use std::time::Instant;

fn query(start: usize, end: usize, generate: bool) -> (usize, u64) {
    let (start, end) = (black_box(start), black_box(end));
    if generate {
        let primes = primeroute::primes_in_range(start, end);
        let checksum = primes
            .iter()
            .fold(0u64, |sum, &p| sum.wrapping_add(p as u64));
        (primes.len(), checksum)
    } else {
        (primeroute::count_primes_in_range(start, end), 0)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        return Err("Usage: range_bench <start> <end> <count|generate> <repeats>".into());
    }
    let start: usize = args[0].parse()?;
    let end: usize = args[1].parse()?;
    let generate = match args[2].as_str() {
        "count" => false,
        "generate" => true,
        _ => return Err("Expected count or generate".into()),
    };
    let repeats: usize = args[3].parse()?;
    if start > end || end == usize::MAX || repeats == 0 {
        return Err("Invalid range or repeat count".into());
    }
    // Warm code pages, while every subsequent query still creates its own sieve.
    let expected = black_box(query(start, end, generate));
    let timer = Instant::now();
    for _ in 0..repeats {
        assert_eq!(black_box(query(start, end, generate)), expected);
    }
    let ns = timer.elapsed().as_nanos();
    println!(
        "{{\"elapsed_ns\":{ns},\"repeats\":{repeats},\"count\":{},\"checksum\":{},\"threads\":1}}",
        expected.0, expected.1
    );
    Ok(())
}
