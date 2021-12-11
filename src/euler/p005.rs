//! Find the sum of all the multiples of 3 or 5 below 1000.
use std::collections::HashMap;

use crate::utils::primes::PrimeFactorization;

fn solve_for(bound: u32) -> u32 {
    // The idea is to get the prime factorizations of each number less than the bound, find the
    // greatest exponent associated with each prime in that range, and multiply those numbers
    // together.
    (1..=bound)
        .flat_map(PrimeFactorization::of)
        .fold(HashMap::<u32, u32>::default(), |mut hm, p| {
            if let Err(mut e) = hm.try_insert(p.factor, p.exponent) {
                let v = e.entry.get_mut();
                if p.exponent > *v {
                    *v = p.exponent;
                }
            }
            hm
        })
        .into_iter()
        .fold(1, |prod, (p, e)| prod * p.pow(e))
}

super::example!(10 => 2520);
super::problem!(u32: 20 => 232_792_560);
