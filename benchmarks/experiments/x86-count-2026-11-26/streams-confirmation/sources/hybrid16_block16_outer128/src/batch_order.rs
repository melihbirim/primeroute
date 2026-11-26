/// Narrow batches need only a few byte-wise radix passes. Keep comparison
/// sorting for small batches and widely separated 64-bit inputs.
pub fn sorted_queries(numbers: &[u64]) -> Vec<(u64, usize)> {
    let mut order: Vec<_> = numbers
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();
    if numbers.len() < 4_096 {
        order.sort_unstable();
        return order;
    }
    let (min, max) = numbers
        .iter()
        .fold((u64::MAX, 0), |(lo, hi), &n| (lo.min(n), hi.max(n)));
    let span = max - min;
    if span > u32::MAX as u64 {
        order.sort_unstable();
        return order;
    }
    let mut scratch = vec![(0, 0); order.len()];
    let mut shift = 0;
    while span >> shift != 0 {
        let mut offsets = [0usize; 256];
        for &(n, _) in &order {
            offsets[(((n - min) >> shift) & 255) as usize] += 1;
        }
        let mut sum = 0;
        for count in &mut offsets {
            let next = sum + *count;
            *count = sum;
            sum = next;
        }
        for &entry in &order {
            let bucket = (((entry.0 - min) >> shift) & 255) as usize;
            scratch[offsets[bucket]] = entry;
            offsets[bucket] += 1;
        }
        std::mem::swap(&mut order, &mut scratch);
        shift += 8;
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radix_matches_tuple_sort_including_duplicates_and_u64_edges() {
        let mut state = 17u64;
        for len in [0, 1, 4_095, 4_096, 8_192] {
            for (base, width) in [(0, 1), (0, 257), (u64::MAX - 65_535, 65_536), (0, u64::MAX)] {
                let inputs: Vec<_> = (0..len)
                    .map(|_| {
                        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                        base + state % width
                    })
                    .collect();
                let mut expected: Vec<_> = inputs
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, n)| (n, i))
                    .collect();
                expected.sort_unstable();
                assert_eq!(sorted_queries(&inputs), expected);
            }
        }
    }
}
