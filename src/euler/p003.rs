//! Find the largest prime factor of 600,851,475,143.
use crate::utils::primes::PrimeFactorization;

fn solve_for(n: u64) -> u64 {
    PrimeFactorization::of(n)
        .last()
        .expect("every number has at least one prime factor")
        .factor
}

super::example!(13195 => 29);
super::problem!(u64: 600_851_475_143 => 6857);
