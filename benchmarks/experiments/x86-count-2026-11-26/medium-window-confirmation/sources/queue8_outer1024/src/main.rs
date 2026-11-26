use std::io::{self, BufWriter, Read, Write};
use std::time::Instant;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    let usage = "Usage: primeroute [start] <limit> [--print]\n       primeroute next [numbers ...]  (or read numbers from stdin)";
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("{usage}");
        return Ok(());
    }
    if args.first().is_some_and(|arg| arg == "next") {
        args.remove(0);
        let mut input = String::new();
        if args.is_empty() {
            io::stdin().read_to_string(&mut input)?;
            args = input.split_whitespace().map(str::to_owned).collect();
        }
        let numbers = args
            .iter()
            .map(|arg| arg.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        let timer = Instant::now();
        let results = primeroute::batch_next_primes(&numbers);
        let mut out = BufWriter::new(io::stdout().lock());
        for answer in results {
            match answer {
                Some(prime) => writeln!(out, "{prime}")?,
                None => writeln!(out, "none")?,
            }
        }
        out.flush()?;
        eprintln!(
            "{} next-prime queries in {:.3}s",
            numbers.len(),
            timer.elapsed().as_secs_f64()
        );
        return Ok(());
    }
    let print = args.last().is_some_and(|arg| arg == "--print");
    if print {
        args.pop();
    }
    let (start, limit): (usize, usize) = match args.as_slice() {
        [limit] => (0, limit.parse()?),
        [start, limit] => (start.parse()?, limit.parse()?),
        _ => return Err(usage.into()),
    };
    if start > limit {
        return Err("start must be <= limit".into());
    }

    let timer = Instant::now();
    let count = if print {
        let mut out = BufWriter::new(io::stdout().lock());
        let mut count = 0;
        let mut result = Ok(());
        primeroute::for_each_prime_in_range(start, limit, |p| {
            count += 1;
            if result.is_ok() {
                result = writeln!(out, "{p}");
            }
        });
        result?;
        out.flush()?;
        count
    } else {
        let count = primeroute::count_primes_in_range(start, limit);
        println!("{count}");
        count
    };
    eprintln!(
        "{count} primes in [{start}, {limit}] in {:.3}s",
        timer.elapsed().as_secs_f64()
    );
    Ok(())
}
