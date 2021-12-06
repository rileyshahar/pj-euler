//! Find the largest prime factor of 600,851,475,143.
use crate::utils;

fn solve_for(n: u64) -> u64 {
    (2..=n)
        .rev()
        .find(|x| n % x == 0)
        .expect("n has a prime factor")
}

// utils::test::example!(13195 => 29);
utils::test::problem!(u64: 600_851_475_143);
