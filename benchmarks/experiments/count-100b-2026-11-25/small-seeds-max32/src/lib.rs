mod batch_order;
mod candidates;
mod next;
pub use next::{
    BatchOptions, BatchResult, BatchStats, BatchStrategy, batch_next_primes,
    batch_next_primes_with_options, is_prime, next_prime,
};

// Each byte represents the eight numbers coprime to 30 in a block of 30.
const RESIDUES: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
const SMALL_PRIMES: [usize; 39] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
];
const SEGMENT_SIZE: usize = 128 * 1024;
static PATTERN_A: [u8; 7 * 11 * 13] = pre_sieve([7, 11, 13]);
static PATTERN_B: [u8; 17 * 19 * 23] = pre_sieve([17, 19, 23]);
static PATTERN_C: [u8; 29 * 31] = pre_sieve([29, 31]);
static PATTERN_D: [u8; 37 * 41] = pre_sieve([37, 41]);
static PATTERN_E: [u8; 43 * 47] = pre_sieve([43, 47]);
static PATTERN_F: [u8; 53 * 59] = pre_sieve([53, 59]);
static PATTERN_G: [u8; 61 * 67] = pre_sieve([61, 67]);
static PATTERN_H: [u8; 71 * 73] = pre_sieve([71, 73]);
static PATTERN_I: [u8; 79 * 83] = pre_sieve([79, 83]);
static PATTERN_J: [u8; 89 * 97] = pre_sieve([89, 97]);
static PATTERN_K: [u8; 101 * 103] = pre_sieve([101, 103]);
static PATTERN_L: [u8; 107 * 109] = pre_sieve([107, 109]);
static PATTERN_M: [u8; 113 * 127] = pre_sieve([113, 127]);
static PATTERN_N: [u8; 131 * 137] = pre_sieve([131, 137]);
static PATTERN_O: [u8; 139 * 149] = pre_sieve([139, 149]);
static PATTERN_P: [u8; 151 * 157] = pre_sieve([151, 157]);
static PATTERN_Q: [u8; 163 * 167] = pre_sieve([163, 167]);
const PATTERNS: [&[u8]; 17] = [
    &PATTERN_A, &PATTERN_B, &PATTERN_C, &PATTERN_D, &PATTERN_E, &PATTERN_F, &PATTERN_G, &PATTERN_H,
    &PATTERN_I, &PATTERN_J, &PATTERN_K, &PATTERN_L, &PATTERN_M, &PATTERN_N, &PATTERN_O, &PATTERN_P,
    &PATTERN_Q,
];

const fn pre_sieve<const N: usize, const P: usize>(primes: [usize; P]) -> [u8; N] {
    let mut pattern = [u8::MAX; N];
    let mut prime = 0;
    while prime < P {
        let p = primes[prime];
        let mut residue = 0;
        while residue < 8 {
            let n = p * RESIDUES[residue];
            let mut bit = 0;
            while RESIDUES[bit] != n % 30 {
                bit += 1;
            }
            let mask = !(1 << bit);
            let mut i = n / 30;
            while i < N {
                pattern[i] &= mask;
                i += p;
            }
            residue += 1;
        }
        prime += 1;
    }
    pattern
}

// Apply the compact static patterns directly. Cycling small contiguous slices
// lets LLVM vectorize each AND without allocating seventeen expanded buffers.
fn fill_segment(low: usize, sieve: &mut [u8]) {
    sieve.fill(255);
    for pattern in PATTERNS {
        let mut offset = low % pattern.len();
        let mut remaining = &mut sieve[..];
        while !remaining.is_empty() {
            let count = remaining.len().min(pattern.len() - offset);
            let (chunk, tail) = remaining.split_at_mut(count);
            for (byte, &mask) in chunk.iter_mut().zip(&pattern[offset..offset + count]) {
                *byte &= mask;
            }
            remaining = tail;
            offset = 0;
        }
    }
}

fn repeat_pattern(pattern: &[u8], size: usize) -> Vec<u8> {
    let mut repeated = vec![0; size + pattern.len()];
    for chunk in repeated.chunks_mut(pattern.len()) {
        chunk.copy_from_slice(&pattern[..chunk.len()]);
    }
    repeated
}

fn fill_expanded(low: usize, sieve: &mut [u8], patterns: &[Vec<u8>;17]) {
    let len=sieve.len();
    let [a,b,c,d,e,f,g,h,j,k,m,n,o,p,q,r,t]: [&[u8];17]=std::array::from_fn(|i| {
        let offset=low % PATTERNS[i].len();
        &patterns[i][offset..offset+len]
    });
    for i in 0..len { sieve[i]=a[i]&b[i]&c[i]&d[i]&e[i]&f[i]&g[i]&h[i]&j[i]&k[i]&m[i]&n[i]&o[i]&p[i]&q[i]&r[i]&t[i]; }
}

struct SievingPrime {
    prime: usize,
    next: [u32; 8],
    masks: [u8; 8],
}

const DENSE_PRIME_LIMIT: usize = SEGMENT_SIZE;

const RESIDUE_INDEX: [usize; 30] = {
    let mut indices = [0; 30];
    let mut i = 0;
    while i < 8 {
        indices[RESIDUES[i]] = i;
        i += 1;
    }
    indices
};

#[derive(Clone, Copy)]
struct WheelStep {
    gap: u32,
    carry: u32,
    mask: u8,
    next: u8,
}

const WHEEL_STEPS: [WheelStep; 64] = {
    let mut steps = [WheelStep {
        gap: 0,
        carry: 0,
        mask: 0,
        next: 0,
    }; 64];
    let mut class = 0;
    while class < 8 {
        let p = RESIDUES[class];
        let mut i = 0;
        while i < 8 {
            let r = RESIDUES[i];
            let next = (i + 1) % 8;
            let gap = if next == 0 {
                31 - r
            } else {
                RESIDUES[next] - r
            };
            steps[class * 8 + i] = WheelStep {
                gap: gap as u32,
                carry: (p * (r + gap) / 30 - p * r / 30) as u32,
                mask: !(1 << RESIDUE_INDEX[p * r % 30]),
                next: (class * 8 + next) as u8,
            };
            i += 1;
        }
        class += 1;
    }
    steps
};

const WHEEL_MASKS: [[u8; 8]; 8] = {
    let mut masks = [[0; 8]; 8];
    let mut class = 0;
    while class < 8 {
        let mut i = 0;
        while i < 8 {
            masks[class][i] = WHEEL_STEPS[class * 8 + i].mask;
            i += 1;
        }
        class += 1;
    }
    masks
};

const NEXT_FACTORS: [(usize, usize); 30] = {
    let mut table = [(0, 0); 30];
    let mut r = 0;
    while r < 30 {
        let mut index = 0;
        while RESIDUES[index] < r {
            index += 1;
        }
        table[r] = (index, RESIDUES[index] - r);
        r += 1;
    }
    table
};

// Sixteen bytes on 64-bit platforms, versus forty-eight for the dense streams.
struct SparsePrime {
    next: usize,
    quotient: u32,
    wheel: u8,
}

impl SparsePrime {
    fn new(p: usize, first_byte: usize) -> Self {
        Self::from_factor(p, p.max((first_byte * 30).div_ceil(p)))
    }

    #[inline]
    fn from_factor(p: usize, mut factor: usize) -> Self {
        let (index, delta) = NEXT_FACTORS[factor % 30];
        factor += delta;
        Self {
            // Equivalent to p * factor / 30 without overflowing the product.
            next: p * (factor / 30) + p * (factor % 30) / 30,
            quotient: (p / 30) as u32,
            wheel: (RESIDUE_INDEX[p % 30] * 8 + index) as u8,
        }
    }

    fn prime(&self) -> usize {
        self.quotient as usize * 30 + RESIDUES[self.wheel as usize / 8]
    }
}

fn seed_primes(limit: usize) -> Vec<u32> {
    debug_assert!(limit <= u32::MAX as usize);
    // Reuse the packed sieve for larger seed tables. Recursion quickly shrinks
    // the bound by sqrt(); the small odd-only sieve below terminates it.
    if limit >= 4_096 {
        let capacity = (limit as f64 / (limit as f64).ln() * 1.2 + 64.0) as usize;
        let mut primes = Vec::with_capacity(capacity);
        primes.extend(
            SMALL_PRIMES
                .iter()
                .copied()
                .filter(|&p| p <= limit)
                .map(|p| p as u32),
        );
        sieve_segments(0, limit, |low, sieve| append_seeds(&mut primes, low, sieve));
        return primes;
    }
    small_seed_primes(limit)
}

fn small_seed_primes(limit: usize) -> Vec<u32> {
    let mut sieve = vec![true; limit / 2 + 1];
    let mut primes = Vec::new();
    if limit >= 2 {
        primes.push(2);
    }
    for p in (3..=limit).step_by(2) {
        if sieve[p / 2] {
            primes.push(p as u32);
            if p <= limit / p {
                for i in (p * p / 2..sieve.len()).step_by(p) {
                    sieve[i] = false;
                }
            }
        }
    }
    primes
}

#[inline(always)]
fn dense_prime(p: usize, first_byte: usize) -> SievingPrime {
    let mut next = [0; 8];
    let square_byte = p * p / 30;
    let advance = if first_byte > square_byte {
        ((first_byte - square_byte) / p) * p
    } else {
        0
    };
    for (i, &r) in RESIDUES.iter().enumerate() {
        let delta = (r + 30 - p % 30) % 30;
        next[i] = p * (p / 30) + p * (p % 30 + delta) / 30 + advance;
        if next[i] < first_byte {
            next[i] += p;
        }
    }
    SievingPrime {
        prime: p,
        // Dense p <= 131072: initial offsets are at most p*p/30+p,
        // below 2^32. Subsequent relative offsets only get smaller.
        next: next.map(|n| (n - first_byte) as u32),
        masks: WHEEL_MASKS[RESIDUE_INDEX[p % 30]],
    }
}

#[inline(always)]
fn mark_dense(state: &mut SievingPrime, sieve: &mut [u8]) {
    let len = sieve.len();
    let p = state.prime;
    let mut last = state.next.into_iter().max().unwrap() as usize;
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state.next.map(|n| n as usize);
    let [ma, mb, mc, md, me, mf, mg, mh] = state.masks;
    // Interleave independent stores with one bounds check for all eight.
    while last < len {
        // SAFETY: every index <= last < len, and all nine indices
        // advance by the same positive p on each iteration.
        unsafe {
            *sieve.get_unchecked_mut(a) &= ma;
            *sieve.get_unchecked_mut(b) &= mb;
            *sieve.get_unchecked_mut(c) &= mc;
            *sieve.get_unchecked_mut(d) &= md;
            *sieve.get_unchecked_mut(e) &= me;
            *sieve.get_unchecked_mut(f) &= mf;
            *sieve.get_unchecked_mut(g) &= mg;
            *sieve.get_unchecked_mut(h) &= mh;
        }
        a += p;
        b += p;
        c += p;
        d += p;
        e += p;
        f += p;
        g += p;
        h += p;
        last += p;
    }
    state.next = [a, b, c, d, e, f, g, h].map(|n| n as u32);
    for (next, &mask) in state.next.iter_mut().zip(&state.masks) {
        let mut i = *next as usize;
        // The eight positions span less than p bytes, so after the
        // bulk loop each stream has at most one remaining hit.
        if i < len {
            sieve[i] &= mask;
            i += p;
        }
        *next = (i - len) as u32;
    }
}

#[inline(never)]
fn mark_dense_many(states: &mut [SievingPrime], sieve: &mut [u8]) {
    for state in states { mark_dense(state, sieve); }
}

// One window retains only the dense streams. Large sparse primes are
// initialized and marked in one pass, avoiding their state vector entirely.
fn mark_single_segment(low: usize, seeds: &[u32], sieve: &mut [u8]) {
    let len = sieve.len();
    let small = seeds.partition_point(|&p| p <= 167);
    let dense = seeds.partition_point(|&p| p as usize <= len.max(167));
    let mut states: Vec<_> = seeds[small..dense]
        .iter()
        .map(|&p| dense_prime(p as usize, low))
        .collect();
    mark_dense_many(&mut states, sieve);
    let mut chunks = seeds[dense..].chunks_exact(8);
    for chunk in &mut chunks {
        let primes: [u32; 8] = chunk.try_into().unwrap();
        let primes = primes.map(|p| p as usize);
        // Independent divisions can overlap before any marking.
        let factors = primes.map(|p| p.max((low * 30).div_ceil(p)));
        for (p, factor) in primes.into_iter().zip(factors) {
            mark_sparse_once(SparsePrime::from_factor(p, factor), low, sieve);
        }
    }
    for &p in chunks.remainder() {
        mark_sparse_once(SparsePrime::new(p as usize, low), low, sieve);
    }
}

#[inline]
fn mark_sparse_once(state: SparsePrime, low: usize, sieve: &mut [u8]) {
    let mut i = state.next - low;
    let mut wheel = state.wheel;
    while i < sieve.len() {
        let step = WHEEL_STEPS[wheel as usize];
        sieve[i] &= step.mask;
        i += (state.quotient * step.gap + step.carry) as usize;
        wheel = step.next;
    }
}

const PACKED_CARRIES: [u64; 8] = {
    let mut packed=[0u64;8]; let mut class=0;
    while class<8 { let mut j=0; while j<8 { packed[class] |= (WHEEL_STEPS[class*8+j].carry as u64) << (8*j); j+=1; } class+=1; }
    packed
};
const PACKED_MASKS: [u64;8] = {
    let mut packed=[0u64;8]; let mut class=0;
    while class<8 { packed[class]=u64::from_le_bytes(WHEEL_MASKS[class]); class+=1; } packed
};
#[inline]
fn mark_medium(state: &mut SparsePrime, sieve: &mut [u8]) {
    let class=state.wheel as usize / 8; let first=state.wheel as usize % 8;
    let mut masks=PACKED_MASKS[class].rotate_right((first*8) as u32);
    let mut carries=PACKED_CARRIES[class].rotate_right((first*8) as u32);
    let mut gaps=0x0206_0402_0402_0406u64.rotate_right((first*8) as u32);
    let mut i=state.next; let mut advances=0;
    while i<sieve.len() {
        sieve[i] &= masks as u8;
        i += (state.quotient * (gaps as u8 as u32) + carries as u8 as u32) as usize;
        masks=masks.rotate_right(8); carries=carries.rotate_right(8); gaps=gaps.rotate_right(8);
        advances+=1;
    }
    state.next=i-sieve.len(); state.wheel=(class*8 + ((first+advances)&7)) as u8;
}

fn sieve_segments(start: usize, limit: usize, visit: impl FnMut(usize, &[u8])) {
    sieve_segments_with_seeds(start, limit, None, visit);
}

// Primes up to MEDIUM_LIMIT hit nearly every window, so they stay in one flat
// list. Larger primes skip most windows; each waits in the bucket of the next
// window it hits, so a window only touches the primes that mark it.
const MEDIUM_LIMIT: usize = 8 * SEGMENT_SIZE;
const SEGMENT_SHIFT: u32 = SEGMENT_SIZE.trailing_zeros();
const OFFSET_MASK: u32 = (1 << SEGMENT_SHIFT) - 1;

// Eight bytes: the byte offset within its window and the wheel index share a word.
#[derive(Clone, Copy)]
struct BigPrime {
    quotient: u32,
    place: u32,
}

// Each bucket is a list of fixed-size chunks drawn from a shared free list, so
// memory follows the number of live big primes, not each bucket's peak size.
const CHUNK: usize = 1024;

struct Buckets {
    ring: Vec<Vec<Vec<BigPrime>>>,
    free: Vec<Vec<BigPrime>>,
    mask: usize,
}

impl Buckets {
    fn new(root: usize) -> Self {
        // One hit advances at most 6 * (p / 30) + 6 bytes, so a ring covering
        // that distance plus one window never wraps onto a live bucket.
        let windows = (root / 5 + 6) / SEGMENT_SIZE + 2;
        let len = windows.next_power_of_two();
        Self {
            ring: (0..len).map(|_| Vec::new()).collect(),
            free: Vec::new(),
            mask: len - 1,
        }
    }

    // `offset` is relative to the first window.
    #[inline]
    fn push(&mut self, offset: usize, quotient: u32, wheel: u8) {
        let bucket = &mut self.ring[(offset >> SEGMENT_SHIFT) & self.mask];
        if bucket.last().is_none_or(|chunk| chunk.len() == CHUNK) {
            let chunk = self.free.pop().unwrap_or_else(|| Vec::with_capacity(CHUNK));
            bucket.push(chunk);
        }
        bucket.last_mut().unwrap().push(BigPrime {
            quotient,
            place: (offset as u32 & OFFSET_MASK) | (wheel as u32) << SEGMENT_SHIFT,
        });
    }
}

// Streams sieving primes in (167, root], one decoded window at a time, so
// memory stays bounded by a window rather than the whole seed table.
// Non-generic, so the recursive sieve instantiates a finite set of closures.
fn stream_seeds(root: usize, visit: &mut dyn FnMut(&[u32])) {
    if root <= 65_536 {
        let primes = small_seed_primes(root);
        let begin = primes.partition_point(|&p| p <= 167);
        visit(&primes[begin..]);
        return;
    }
    let mut buffer = Vec::new();
    sieve_segments_with_seeds(0, root, None, |low, sieve| {
        buffer.clear();
        append_seeds(&mut buffer, low, sieve);
        visit(&buffer);
    });
}

fn for_each_seed(root: usize, seeds: Option<&[u32]>, visit: &mut dyn FnMut(&[u32])) {
    match seeds {
        Some(seeds) => {
            let begin = seeds.partition_point(|&p| p <= 167);
            visit(&seeds[begin..seeds.partition_point(|&p| p as usize <= root)]);
        }
        None => stream_seeds(root, visit),
    }
}

fn sieve_segments_with_seeds(
    start: usize,
    limit: usize,
    seeds: Option<&[u32]>,
    mut visit: impl FnMut(usize, &[u8]),
) {
    if start > limit || limit <= SMALL_PRIMES[SMALL_PRIMES.len() - 1] {
        return;
    }
    let root = limit.isqrt();
    let first_byte = start / 30;
    let total_bytes = limit / 30 + 1;
    let span = total_bytes - first_byte;
    let capacity = SEGMENT_SIZE.min(span);
    let mut segment = vec![0u8; capacity];

    if span <= SEGMENT_SIZE {
        let sieve = &mut segment[..];
        fill_segment(first_byte, sieve);
        match seeds {
            Some(seeds) => mark_single_segment(
                first_byte,
                &seeds[..seeds.partition_point(|&p| p as usize <= root)],
                sieve,
            ),
            None => stream_seeds(root, &mut |primes| {
                mark_single_segment(first_byte, primes, sieve)
            }),
        }
        finish_segment(start, limit, first_byte, first_byte, sieve);
        visit(first_byte, sieve);
        return;
    }

    let expanded = (span >= 8*SEGMENT_SIZE).then(|| PATTERNS.map(|p| repeat_pattern(p, capacity)));
    let dense_limit = DENSE_PRIME_LIMIT.min(span);
    let mut primes = Vec::new();
    let mut medium = Vec::new();
    let mut buckets = Buckets::new(root);
    // Big primes whose first multiple, p^2, lies beyond the ring. Ascending.
    let mut pending: Vec<u32> = Vec::new();
    for_each_seed(root, seeds, &mut |seeds| {
        for &p in seeds {
            let p = p as usize;
            if p <= dense_limit {
                primes.push(dense_prime(p, first_byte));
                continue;
            }
            let state = SparsePrime::new(p, first_byte);
            if state.next >= total_bytes {
                continue; // No multiple inside the range.
            }
            if p <= MEDIUM_LIMIT {
                medium.push(state);
                continue;
            }
            let offset = state.next - first_byte;
            if offset >> SEGMENT_SHIFT <= buckets.mask {
                buckets.push(offset, state.quotient, state.wheel);
            } else {
                pending.push(p as u32);
            }
        }
    });

    let mut low = first_byte;
    let mut window = 0;
    let mut active = 0;
    let mut medium_active = 0;
    let mut pending_active = 0;
    while low < total_bytes {
        let len = SEGMENT_SIZE.min(total_bytes - low);
        let sieve = &mut segment[..len];
        if let Some(ref patterns) = expanded { fill_expanded(low,sieve,patterns); } else { fill_segment(low, sieve); }

        // Activate each prime once, then keep all marking positions relative
        // to the current window. Inactive dense offsets refer to the first window;
        // inactive medium positions are global byte offsets.
        let next_active =
            primes.partition_point(|state| state.prime * state.prime / 30 < low + len);
        for state in &mut primes[active..next_active] {
            for next in &mut state.next {
                *next -= (low - first_byte) as u32;
            }
        }
        active = next_active;
        mark_dense_many(&mut primes[..active], sieve);

        let next_medium_active = medium.partition_point(|state| {
            let p = state.prime();
            p * p / 30 < low + len
        });
        for state in &mut medium[medium_active..next_medium_active] {
            state.next -= low;
        }
        medium_active = next_medium_active;
        for state in &mut medium[..medium_active] { mark_medium(state, sieve); }

        // Move pending primes into the ring once p^2 comes within its reach.
        // Earlier windows already took every prime below this bound, so each
        // new offset belongs to this window or a later one.
        while let Some(&p) = pending.get(pending_active) {
            let offset = p as usize * p as usize / 30 - first_byte;
            if offset >> SEGMENT_SHIFT > window + buckets.mask {
                break;
            }
            let state = SparsePrime::from_factor(p as usize, p as usize);
            buckets.push(offset, state.quotient, state.wheel);
            pending_active += 1;
        }

        let slot = window & buckets.mask;
        let mut chunks = std::mem::take(&mut buckets.ring[slot]);
        let base = window << SEGMENT_SHIFT;
        for mut chunk in chunks.drain(..) {
            for big in &chunk {
                let mut i = (big.place & OFFSET_MASK) as usize;
                let mut wheel = (big.place >> SEGMENT_SHIFT) as u8;
                while i < len {
                    let step = WHEEL_STEPS[wheel as usize];
                    sieve[i] &= step.mask;
                    i += (big.quotient * step.gap + step.carry) as usize;
                    wheel = step.next;
                }
                // The next hit is in a later window, never this slot again.
                let offset = base + i;
                if offset < span {
                    buckets.push(offset, big.quotient, wheel);
                }
            }
            chunk.clear();
            buckets.free.push(chunk);
        }
        buckets.ring[slot] = chunks;

        finish_segment(start, limit, first_byte, low, sieve);
        visit(low, sieve);
        low += len;
        window += 1;
    }
}

fn finish_segment(start: usize, limit: usize, first_byte: usize, low: usize, sieve: &mut [u8]) {
    let len = sieve.len();
    if low == 0 {
        sieve[0] &= !1; // 1 is not prime.
    }
    if low == first_byte {
        let bits = RESIDUES.iter().take_while(|&&r| r < start % 30).count();
        sieve[0] &= (u16::from(u8::MAX) << bits) as u8;
    }
    if low + len == limit / 30 + 1 {
        let remainder = limit % 30;
        let bits = RESIDUES.iter().take_while(|&&r| r <= remainder).count();
        sieve[len - 1] &= ((1u16 << bits) - 1) as u8;
    }
}

/// Visits every prime <= limit in ascending order, without storing the results.
pub fn for_each_prime(limit: usize, visit: impl FnMut(usize)) {
    for_each_prime_in_range(0, limit, visit);
}

/// Visits every prime in the inclusive interval [start, limit], in order.
/// An inverted interval is empty. Numbers below start are not sieved.
pub fn for_each_prime_in_range(start: usize, limit: usize, mut visit: impl FnMut(usize)) {
    if start > limit {
        return;
    }
    for p in SMALL_PRIMES {
        if p >= start && p <= limit {
            visit(p);
        }
    }
    sieve_segments(start, limit, |low, sieve| {
        for (i, &byte) in sieve.iter().enumerate() {
            let mut bits = byte;
            while bits != 0 {
                let bit = bits.trailing_zeros() as usize;
                visit(30 * (low + i) + RESIDUES[bit]);
                bits &= bits - 1;
            }
        }
    });
}

/// Counts primes <= limit using population counts over the packed sieve.
pub fn count_primes(limit: usize) -> usize {
    count_primes_in_range(0, limit)
}

/// Counts primes in the inclusive interval [start, limit].
/// An inverted interval is empty. Numbers below start are not sieved.
pub fn count_primes_in_range(start: usize, limit: usize) -> usize {
    if start > limit {
        return 0;
    }
    let mut count = SMALL_PRIMES
        .iter()
        .filter(|&&p| p >= start && p <= limit)
        .count();
    sieve_segments(start, limit, |_, sieve| {
        count += count_bits(sieve);
    });
    count
}

/// Generates an ordered vector of all primes in the inclusive interval.
/// An inverted interval is empty.
pub fn primes_in_range(start: usize, limit: usize) -> Vec<usize> {
    if start > limit {
        return Vec::new();
    }
    let width = limit.saturating_sub(start).saturating_add(1) as f64;
    let capacity = (width / (limit.max(3) as f64).ln() * 1.2 + 64.0) as usize;
    let mut primes = Vec::with_capacity(capacity);
    for p in SMALL_PRIMES {
        if p >= start && p <= limit {
            primes.push(p);
        }
    }
    sieve_segments(start, limit, |low, sieve| {
        append_primes(&mut primes, low, sieve)
    });
    primes
}

fn append_seeds(primes: &mut Vec<u32>, low: usize, sieve: &[u8]) {
    #[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
    if std::arch::is_aarch64_feature_detected!("neon") {
        // SAFETY: seed primes are bounded by sqrt(usize::MAX) <= u32::MAX;
        // the generic decoder reserves sixteen slots per lookup.
        unsafe { append_primes_neon(primes, low, sieve) };
        return;
    }
    for (i, &byte) in sieve.iter().enumerate() {
        let mut bits = byte;
        while bits != 0 {
            primes.push((30 * (low + i) + RESIDUES[bits.trailing_zeros() as usize]) as u32);
            bits &= bits - 1;
        }
    }
}

fn append_primes(primes: &mut Vec<usize>, low: usize, sieve: &[u8]) {
    #[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
    if std::arch::is_aarch64_feature_detected!("neon") {
        // SAFETY: NEON was checked; the function reserves space for its stores.
        unsafe { append_primes_neon(primes, low, sieve) };
        return;
    }
    for (i, &byte) in sieve.iter().enumerate() {
        let mut bits = byte;
        while bits != 0 {
            primes.push(30 * (low + i) + RESIDUES[bits.trailing_zeros() as usize]);
            bits &= bits - 1;
        }
    }
}

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct DecodedWord {
    offsets: u64,
    remaining: u16,
    len: u8,
}

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
#[allow(long_running_const_eval)]
static DECODED_WORDS: [DecodedWord; 65_536] = {
    let mut table = [DecodedWord {
        offsets: 0,
        remaining: 0,
        len: 0,
    }; 65_536];
    let mut word = 1usize;
    while word < 65_536 {
        let previous = table[word & (word - 1)];
        let bit = word.trailing_zeros() as usize;
        let offset = 30 * (bit / 8) + RESIDUES[bit % 8];
        let mut remaining = previous.remaining;
        if previous.len >= 8 {
            let last = (previous.offsets >> 56) as usize;
            remaining |= 1 << (8 * (last / 30) + RESIDUE_INDEX[last % 30]);
        }
        table[word] = DecodedWord {
            offsets: (previous.offsets << 8) | offset as u64,
            remaining,
            len: previous.len + 1,
        };
        word += 1;
    }
    table
};

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
#[target_feature(enable = "neon")]
#[inline]
// Caller provides sixteen writable u64 slots; only valid entries are exposed.
unsafe fn store_decoded_word(dst: *mut u64, number: usize, decoded: DecodedWord) {
    use std::arch::aarch64::*;
    let base = vdupq_n_u64(number as u64);
    let bytes = vcreate_u8(decoded.offsets);
    let halves = vmovl_u8(bytes);
    let lower = vmovl_u16(vget_low_u16(halves));
    // SAFETY: reserve guarantees sixteen spare 64-bit slots. The lookup
    // contains the first eight offsets; widening initializes four or eight
    // slots. Any remaining set bits are written individually before set_len.
    // Padding stores are not exposed through the vector's logical length.
    unsafe {
        vst1q_u64(dst, vaddq_u64(base, vmovl_u32(vget_low_u32(lower))));
        vst1q_u64(dst.add(2), vaddq_u64(base, vmovl_high_u32(lower)));
        if decoded.len > 4 {
            let upper = vmovl_high_u16(halves);
            vst1q_u64(dst.add(4), vaddq_u64(base, vmovl_u32(vget_low_u32(upper))));
            vst1q_u64(dst.add(6), vaddq_u64(base, vmovl_high_u32(upper)));
        }
        let mut remaining = decoded.remaining;
        let mut written = 8;
        while remaining != 0 {
            let bit = remaining.trailing_zeros() as usize;
            *dst.add(written) = (number + 30 * (bit / 8) + RESIDUES[bit % 8]) as u64;
            remaining &= remaining - 1;
            written += 1;
        }
    }
}

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
trait WheelOutput: Copy {
    fn number(n: usize) -> Self;
    // Caller provides sixteen writable slots; only decoded.len are exposed.
    unsafe fn store(dst: *mut Self, number: usize, decoded: DecodedWord);
}

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
impl WheelOutput for usize {
    fn number(n: usize) -> Self {
        n
    }

    #[target_feature(enable = "neon")]
    #[inline]
    unsafe fn store(dst: *mut Self, number: usize, decoded: DecodedWord) {
        // SAFETY: the caller supplies sixteen slots and NEON is enabled.
        unsafe { store_decoded_word(dst.cast(), number, decoded) };
    }
}

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
impl WheelOutput for u32 {
    fn number(n: usize) -> Self {
        n as u32
    }

    #[target_feature(enable = "neon")]
    #[inline]
    unsafe fn store(dst: *mut Self, number: usize, decoded: DecodedWord) {
        use std::arch::aarch64::*;
        let base = vdupq_n_u32(number as u32);
        let halves = vmovl_u8(vcreate_u8(decoded.offsets));
        // SAFETY: sixteen spare slots cover padded vector stores plus any
        // scalar tail. All valid seed values fit u32; padding is never exposed.
        unsafe {
            vst1q_u32(dst, vaddq_u32(base, vmovl_u16(vget_low_u16(halves))));
            if decoded.len > 4 {
                vst1q_u32(dst.add(4), vaddq_u32(base, vmovl_high_u16(halves)));
            }
            let mut remaining = decoded.remaining;
            let mut written = 8;
            while remaining != 0 {
                let bit = remaining.trailing_zeros() as usize;
                *dst.add(written) = (number + 30 * (bit / 8) + RESIDUES[bit % 8]) as u32;
                remaining &= remaining - 1;
                written += 1;
            }
        }
    }
}

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
#[target_feature(enable = "neon")]
// Keep the decoder separate from the large marking loop to reduce register pressure.
#[inline(never)]
unsafe fn append_primes_neon<T: WheelOutput>(primes: &mut Vec<T>, low: usize, sieve: &[u8]) {
    let mut blocks = sieve.chunks_exact(8);
    for (i, block) in blocks.by_ref().enumerate() {
        let bits = u64::from_le_bytes(block.try_into().unwrap());
        if bits == 0 {
            continue;
        }
        // Fetch four independent lookup records before writing any output.
        let a = DECODED_WORDS[(bits & 0xffff) as usize];
        let b = DECODED_WORDS[((bits >> 16) & 0xffff) as usize];
        let c = DECODED_WORDS[((bits >> 32) & 0xffff) as usize];
        let d = DECODED_WORDS[(bits >> 48) as usize];
        let len = primes.len();
        if primes.capacity() - len < 64 {
            primes.reserve(64);
        }
        let number = 30 * (low + 8 * i);
        let ab = a.len as usize + b.len as usize;
        let abc = ab + c.len as usize;
        // SAFETY: each record writes at most sixteen initialized values/pads.
        // Previous records contribute at most sixteen valid entries each, so
        // reserving 64 slots bounds every store. Only valid entries are exposed.
        unsafe {
            let dst = primes.as_mut_ptr().add(len);
            T::store(dst, number, a);
            T::store(dst.add(a.len as usize), number + 60, b);
            T::store(dst.add(ab), number + 120, c);
            T::store(dst.add(abc), number + 180, d);
            primes.set_len(len + abc + d.len as usize);
        }
    }
    let remainder = blocks.remainder();
    let tail_low = low + sieve.len() - remainder.len();
    let mut pairs = remainder.chunks_exact(2);
    for (i, pair) in pairs.by_ref().enumerate() {
        let word = u16::from_le_bytes([pair[0], pair[1]]);
        if word == 0 {
            continue;
        }
        let decoded = DECODED_WORDS[word as usize];
        let len = primes.len();
        if primes.capacity() - len < 16 {
            primes.reserve(16);
        }
        // SAFETY: sixteen spare slots bound the helper's stores.
        unsafe {
            let dst = primes.as_mut_ptr().add(len);
            T::store(dst, 30 * (tail_low + 2 * i), decoded);
            primes.set_len(len + decoded.len as usize);
        }
    }
    if let Some(&byte) = pairs.remainder().first() {
        let number = 30 * (low + sieve.len() - 1);
        let mut bits = byte;
        while bits != 0 {
            primes.push(T::number(number + RESIDUES[bits.trailing_zeros() as usize]));
            bits &= bits - 1;
        }
    }
}

fn count_bits(bytes: &[u8]) -> usize {
    #[cfg(target_arch = "aarch64")]
    if std::arch::is_aarch64_feature_detected!("neon") {
        // SAFETY: NEON support was checked; the function bounds its loads.
        return unsafe { count_bits_neon(bytes) };
    }
    bytes.iter().map(|byte| byte.count_ones() as usize).sum()
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn count_bits_neon(bytes: &[u8]) -> usize {
    use std::arch::aarch64::*;

    let mut count = 0;
    // Bound each accumulator's lanes well below u16::MAX even for all-set bits.
    for block in bytes.chunks(16 * 1024) {
        let mut a = vdupq_n_u16(0);
        let mut b = vdupq_n_u16(0);
        let mut chunks = block.chunks_exact(32);
        for chunk in &mut chunks {
            // SAFETY: each chunk contains exactly 32 readable bytes. NEON loads
            // accept unaligned addresses; neither load crosses the chunk.
            let (x, y) = unsafe { (vld1q_u8(chunk.as_ptr()), vld1q_u8(chunk.as_ptr().add(16))) };
            a = vpadalq_u8(a, vcntq_u8(x));
            b = vpadalq_u8(b, vcntq_u8(y));
        }
        count += vaddlvq_u16(a) as usize + vaddlvq_u16(b) as usize;
        count += chunks
            .remainder()
            .iter()
            .map(|x| x.count_ones() as usize)
            .sum::<usize>();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_wheel_matches_integer_multiples() {
        for p in (173..1_000).filter(|n: &usize| {
            !n.is_multiple_of(2) && !n.is_multiple_of(3) && !n.is_multiple_of(5)
        }) {
            for first_byte in [0, 1_000, 1_000_000] {
                let mut state = SparsePrime::new(p, first_byte);
                let mut factor = p.max((first_byte * 30).div_ceil(p));
                for _ in 0..64 {
                    while factor.is_multiple_of(2)
                        || factor.is_multiple_of(3)
                        || factor.is_multiple_of(5)
                    {
                        factor += 1;
                    }
                    let step = WHEEL_STEPS[state.wheel as usize];
                    let residue = RESIDUES[(!step.mask).trailing_zeros() as usize];
                    assert_eq!(state.next * 30 + residue, p * factor);
                    assert_eq!(state.prime(), p);
                    state.next += (state.quotient * step.gap + step.carry) as usize;
                    state.wheel = step.next;
                    factor += 1;
                }
            }
        }
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn bucketed_windows_match_single_window_chunks() {
        // Near 1.2e12 a few bucketed primes start at p^2 beyond the ring and wait
        // in the pending list; near 1e15 most big primes skip whole windows.
        for (start, width) in [
            (1_200_000_000_000, 60_000_000),
            (1_000_000_000_000_000, 20_000_000),
        ] {
            let end = start + width;
            let mut expected = Vec::new();
            let chunk = 30 * SEGMENT_SIZE - 1;
            let mut low = start;
            while low <= end {
                let high = (low + chunk - 1).min(end);
                expected.extend(primes_in_range(low, high));
                low = high + 1;
            }
            assert_eq!(primes_in_range(start, end), expected, "start={start}");
            assert_eq!(count_primes_in_range(start, end), expected.len());
        }
    }

    #[test]
    fn compact_dense_offsets_match_wide_arithmetic() {
        for p in [173usize, 4_093, DENSE_PRIME_LIMIT - 1] {
            if p > usize::MAX.isqrt() {
                continue;
            }
            let square = p * p / 30;
            for low in [
                0,
                square.saturating_sub(1),
                square,
                square + 1,
                usize::MAX / 30 - DENSE_PRIME_LIMIT,
            ] {
                let state = dense_prime(p, low);
                for (i, &r) in RESIDUES.iter().enumerate() {
                    let factor = p + (r + 30 - p % 30) % 30;
                    let mut byte = p as u128 * factor as u128 / 30;
                    if byte < low as u128 {
                        byte += (low as u128 - byte).div_ceil(p as u128) * p as u128;
                    }
                    assert_eq!(state.next[i] as u128, byte - low as u128);
                }
            }
        }
    }

    #[test]
    fn seed_decoder_handles_u32_endpoint_padding() {
        let low = u32::MAX as usize / 30 - 2;
        let sieve = [255, 255, 15];
        let mut seeds = Vec::new();
        append_seeds(&mut seeds, low, &sieve);
        let mut wide = Vec::new();
        append_primes(&mut wide, low, &sieve);
        assert!(wide.iter().all(|&n| n as u64 <= u32::MAX as u64));
        assert_eq!(
            seeds,
            wide.into_iter().map(|n| n as u32).collect::<Vec<_>>()
        );
    }

    #[test]
    fn decodes_every_word_including_dense_masks_and_growth() {
        for word in 0..=u16::MAX {
            for copies in [1, 4] {
                let pair = word.to_le_bytes();
                let sieve: Vec<_> = (0..copies).flat_map(|_| pair).collect();
                let mut actual = vec![42];
                append_primes(&mut actual, 1_000, &sieve);
                let mut expected = vec![42];
                for (i, byte) in sieve.iter().enumerate() {
                    for (bit, &r) in RESIDUES.iter().enumerate() {
                        if byte & (1 << bit) != 0 {
                            expected.push(30_000 + 30 * i + r);
                        }
                    }
                }
                assert_eq!(actual, expected, "word = {word}, copies = {copies}");
                let mut seeds = vec![42u32];
                append_seeds(&mut seeds, 1_000, &sieve);
                assert_eq!(
                    seeds,
                    expected.iter().map(|&n| n as u32).collect::<Vec<_>>()
                );
            }
        }
        for len in 0..=33 {
            let sieve: Vec<_> = (0..len).map(|i| (i * 137) as u8).collect();
            let mut actual = vec![42];
            append_primes(&mut actual, 1_000, &sieve);
            let mut expected = vec![42];
            for (i, byte) in sieve.iter().enumerate() {
                for (bit, &r) in RESIDUES.iter().enumerate() {
                    if byte & (1 << bit) != 0 {
                        expected.push(30_000 + 30 * i + r);
                    }
                }
            }
            assert_eq!(actual, expected, "tail length = {len}");
            let mut seeds = vec![42u32];
            append_seeds(&mut seeds, 1_000, &sieve);
            assert_eq!(
                seeds,
                expected.iter().map(|&n| n as u32).collect::<Vec<_>>()
            );
        }
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn high_range_matches_trial_division() {
        let start = 10_000_000_000usize;
        let end = start + 2_000;
        let expected: Vec<_> = (start..=end)
            .filter(|&n| {
                !n.is_multiple_of(2) && (3..=n.isqrt()).step_by(2).all(|p| !n.is_multiple_of(p))
            })
            .collect();
        assert_eq!(primes_in_range(start, end), expected);
        assert_eq!(count_primes_in_range(start, end), expected.len());
    }

    #[test]
    fn decodes_every_byte_and_grows_the_output_safely() {
        for byte in 0..=u8::MAX {
            let mut actual = vec![42];
            append_primes(&mut actual, 1_000, &[byte]);
            let mut expected = vec![42];
            for (bit, &residue) in RESIDUES.iter().enumerate() {
                if byte & (1 << bit) != 0 {
                    expected.push(30_000 + residue);
                }
            }
            assert_eq!(actual, expected, "byte = {byte}");
        }
    }

    #[test]
    fn population_counts_match_scalar_for_tails_and_full_accumulators() {
        for len in [
            0,
            1,
            15,
            16,
            17,
            31,
            32,
            33,
            16_383,
            16_384,
            16_385,
            SEGMENT_SIZE,
        ] {
            let mut bytes = vec![u8::MAX; len];
            assert_eq!(count_bits(&bytes), 8 * len);
            for (i, byte) in bytes.iter_mut().enumerate() {
                *byte = i.wrapping_mul(137).wrapping_add(i / 7) as u8;
            }
            let expected: usize = bytes.iter().map(|byte| byte.count_ones() as usize).sum();
            assert_eq!(count_bits(&bytes), expected, "length = {len}");
        }
    }

    #[test]
    fn lists_primes_in_order() {
        let mut primes = Vec::new();
        for_each_prime(30, |p| primes.push(p));
        assert_eq!(primes, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn known_counts() {
        for (limit, expected) in [
            (0, 0),
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 2),
            (9, 4),
            (10, 4),
            (100, 25),
            (1_000, 168),
            (1_000_000, 78_498),
            (100_000_000, 5_761_455),
        ] {
            assert_eq!(count_primes(limit), expected, "limit = {limit}");
        }
    }

    #[test]
    fn matches_independent_sieve_at_every_small_limit() {
        let limit = 2_000usize;
        let mut reference = vec![true; limit + 1];
        reference[0] = false;
        reference[1] = false;
        for p in 2..=limit.isqrt() {
            if reference[p] {
                for n in (p * p..=limit).step_by(p) {
                    reference[n] = false;
                }
            }
        }
        let mut expected = Vec::new();
        for (n, &is_prime) in reference.iter().enumerate() {
            if is_prime {
                expected.push(n);
            }
            let mut actual = Vec::new();
            for_each_prime(n, |p| actual.push(p));
            assert_eq!(actual, expected, "limit = {n}");
            assert_eq!(count_primes(n), expected.len(), "limit = {n}");
        }
    }

    #[test]
    fn enumerates_across_segment_boundaries() {
        let limit = 30 * SEGMENT_SIZE + 61;
        let mut reference = vec![true; limit + 1];
        reference[0] = false;
        reference[1] = false;
        for p in 2..=limit.isqrt() {
            if reference[p] {
                for n in (p * p..=limit).step_by(p) {
                    reference[n] = false;
                }
            }
        }
        let expected: Vec<_> = (2..=limit).filter(|&n| reference[n]).collect();
        let mut actual = Vec::new();
        for_each_prime(limit, |p| actual.push(p));
        assert_eq!(actual, expected);
        for end in 30 * SEGMENT_SIZE - 31..=limit {
            assert_eq!(count_primes(end), expected.partition_point(|&p| p <= end));
        }
    }

    #[test]
    fn ranges_match_an_independent_sieve() {
        let limit = 30 * SEGMENT_SIZE + 100;
        let mut reference = vec![true; limit + 1];
        reference[0] = false;
        reference[1] = false;
        for p in 2..=limit.isqrt() {
            if reference[p] {
                for n in (p * p..=limit).step_by(p) {
                    reference[n] = false;
                }
            }
        }
        let primes: Vec<_> = (2..=limit).filter(|&n| reference[n]).collect();
        let mut ranges = vec![
            (0, 0),
            (0, 2),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (7, 7),
            (167, 167),
            (168, 172),
            (173, 173),
            (10, 9),
            (173 * 173 - 1, 173 * 173 + 1),
            (29, limit),
            (30 * SEGMENT_SIZE - 31, limit),
        ];
        for n in 150..=210 {
            ranges.push((n, n));
            ranges.push((100_000 + n, 100_000 + n + 29));
        }
        let mut random = 42u64;
        for _ in 0..100 {
            random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
            let start = (random >> 32) as usize % limit;
            let end = (start + random as usize % 10_001).min(limit);
            ranges.push((start, end));
        }
        for (start, end) in ranges {
            let expected = if start > end {
                &primes[0..0]
            } else {
                &primes
                    [primes.partition_point(|&p| p < start)..primes.partition_point(|&p| p <= end)]
            };
            let mut actual = Vec::new();
            for_each_prime_in_range(start, end, |p| actual.push(p));
            assert_eq!(actual, expected, "range [{start}, {end}]");
            assert_eq!(
                primes_in_range(start, end),
                expected,
                "generated range [{start}, {end}]"
            );
            assert_eq!(
                count_primes_in_range(start, end),
                expected.len(),
                "range [{start}, {end}]"
            );
        }
    }
}
