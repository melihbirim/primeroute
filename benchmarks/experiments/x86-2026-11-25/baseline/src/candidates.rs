// Shared wheel and small-prime filtering for direct next-prime searches.
pub(crate) const FILTER_PRIMES: [u64; 18] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
];
const RESIDUES: [u64; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
const GAPS: [u64; 8] = [6, 4, 2, 4, 2, 4, 6, 2];
const FILTER: [bool; 1_001] = {
    let mut table = [false; 1_001];
    let mut i = 0usize;
    while i < table.len() {
        table[i] = !i.is_multiple_of(7) && !i.is_multiple_of(11) && !i.is_multiple_of(13);
        i += 1;
    }
    table
};

fn passes_filter(n: u64) -> bool {
    FILTER[(n % 1_001) as usize] && FILTER_PRIMES[6..].iter().all(|&p| !n.is_multiple_of(p))
}

pub(crate) fn next_by(n: u64, mut test: impl FnMut(u64) -> bool) -> Option<u64> {
    if n < 61 {
        return FILTER_PRIMES.into_iter().find(|&p| p > n);
    }
    let mut candidate = n.checked_add(1)?;
    let residue = candidate % 30;
    let mut index = RESIDUES.partition_point(|&r| r < residue);
    // Every remainder is <=29, so index is always in 0..8.
    candidate = candidate.checked_add(RESIDUES[index] - residue)?;
    loop {
        if passes_filter(candidate) && test(candidate) {
            return Some(candidate);
        }
        candidate = candidate.checked_add(GAPS[index])?;
        index = (index + 1) & 7;
    }
}
