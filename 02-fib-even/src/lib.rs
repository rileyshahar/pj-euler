#![feature(destructuring_assignment)]

#[must_use]
pub fn solve() -> u32 {
    let (mut prev, mut curr) = (1, 2);
    let mut out = 0;

    while curr < 4_000_000 {
        out += curr;
        for _ in 0..3 {
            (prev, curr) = (curr, prev + curr);
        }
    }

    out
}
