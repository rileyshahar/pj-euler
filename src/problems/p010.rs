//! Find the sum of all the primes below two million.
use crate::utils;

use utils::primes::Primes;

fn solve_for(bound: u64) -> u64 {
    Primes::<u64>::new().take_while(|&p| p < bound).sum()
}

utils::test::example!(10 => 17);
utils::test::problem!(u64: 2_000_000 => 142_913_828_922);
