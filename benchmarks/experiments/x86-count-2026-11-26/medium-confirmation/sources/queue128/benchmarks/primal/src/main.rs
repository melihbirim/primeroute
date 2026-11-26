use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limit: usize = std::env::args().nth(1).ok_or("Missing limit")?.parse()?;
    let start = Instant::now();
    let count = primal::StreamingSieve::prime_pi(limit);
    println!("{count}");
    eprintln!(
        "{count} primes <= {limit} in {:.3}s",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}
