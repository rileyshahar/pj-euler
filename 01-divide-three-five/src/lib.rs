//! Find the sum of all the multiples of 3 or 5 below 1000.

#[must_use]
pub fn solve() -> u32 {
    (0..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

utils::euler_expect!(233_168);
