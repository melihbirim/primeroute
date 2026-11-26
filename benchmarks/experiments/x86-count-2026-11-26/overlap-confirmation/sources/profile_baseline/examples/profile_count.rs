fn main() {
    let limit: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let start = std::time::Instant::now();
    let count = primeroute::count_primes(limit);
    println!(
        "{{\"count\":{},\"total_ns\":{},\"stages_ns\":{:?},\"threads\":1}}",
        count,
        start.elapsed().as_nanos(),
        primeroute::profile_times()
    );
}
