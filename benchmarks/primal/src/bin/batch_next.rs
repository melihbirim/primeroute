use primeroute::{BatchOptions, BatchResult, BatchStats, BatchStrategy};
use std::io::{self, Read};
use std::time::Instant;

#[path = "../../../../src/candidates.rs"]
mod candidates;

#[path = "../../../../src/batch_order.rs"]
mod batch_order;

fn primal_batch(numbers: &[u64]) -> BatchResult {
    let order = batch_order::sorted_queries(numbers);
    let mut answers = vec![None; numbers.len()];
    let mut previous = None;
    let mut previous_n = None;
    let mut unique = 0;
    for (n, original) in order {
        if previous_n != Some(n) {
            unique += 1;
        }
        let result = if previous_n == Some(n) || previous.is_some_and(|p| p > n) {
            previous
        } else {
            candidates::next_by(n, primal::is_prime)
        };
        answers[original] = result;
        previous_n = Some(n);
        previous = result;
    }
    BatchResult {
        answers,
        stats: BatchStats {
            unique_queries: unique,
            ..BatchStats::default()
        },
    }
}

fn verify() {
    let mut state = 42u64;
    for n in 0..200_000 {
        assert_eq!(primeroute::is_prime(n), primal::is_prime(n), "n={n}");
    }
    for _ in 0..100_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        assert_eq!(
            primeroute::is_prime(state),
            primal::is_prime(state),
            "n={state}"
        );
    }
    for _ in 0..10_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let n = state as u32 as u64;
        assert_eq!(
            primeroute::is_prime(n),
            primal::is_prime(n),
            "32-bit n={n}"
        );
    }
    for base in [
        10u64.pow(8),
        10u64.pow(12),
        10u64.pow(14),
        10u64.pow(18),
        u64::MAX - 100_000,
    ] {
        for i in 0..500 {
            let n = base.saturating_add(i * 137);
            assert_eq!(
                primeroute::next_prime(n),
                candidates::next_by(n, primal::is_prime),
                "next({n})"
            );
        }
    }
    eprintln!(
        "Verified primality for 200,000 small and 100,000 random u64 values; 2,500 high next-prime queries agree with primal."
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = std::env::args()
        .nth(1)
        .ok_or("Expected auto, direct, sieve, primal or verify")?;
    if mode == "verify" {
        verify();
        return Ok(());
    }
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    let numbers = text
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;
    let timer = Instant::now();
    let result = match mode.as_str() {
        "primal" => primal_batch(&numbers),
        "auto" | "direct" | "sieve" => primeroute::batch_next_primes_with_options(
            &numbers,
            BatchOptions {
                strategy: match mode.as_str() {
                    "direct" => BatchStrategy::Direct,
                    "sieve" => BatchStrategy::Sieve,
                    _ => BatchStrategy::Auto,
                },
                ..BatchOptions::default()
            },
        ),
        _ => return Err("Unknown mode".into()),
    };
    let elapsed_ns = timer.elapsed().as_nanos();
    let stats = result.stats;
    print!(
        "{{\"elapsed_ns\":{elapsed_ns},\"threads\":1,\"unique_queries\":{},\"direct_groups\":{},\"sieve_groups\":{},\"direct_queries\":{},\"sieve_queries\":{},\"tail_fallbacks\":{},\"max_estimated_sieve_bytes\":{},\"answers\":[",
        stats.unique_queries,
        stats.direct_groups,
        stats.sieve_groups,
        stats.direct_queries,
        stats.sieve_queries,
        stats.tail_fallbacks,
        stats.max_estimated_sieve_bytes
    );
    for (i, answer) in result.answers.iter().enumerate() {
        if i != 0 {
            print!(",");
        }
        match answer {
            Some(p) => print!("{p}"),
            None => print!("null"),
        }
    }
    println!("]}}");
    Ok(())
}
