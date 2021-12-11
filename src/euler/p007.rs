//! What is the 10,001st prime number?
use crate::utils::primes::Primes;

fn solve_for(n: usize) -> u32 {
    Primes::new()
        .nth(n - 1) // nth indexes from 0
        .expect("there are infinitely many primes")
}

super::example!(6 => 13);
super::problem!(u32: 10001 => 104_743);
