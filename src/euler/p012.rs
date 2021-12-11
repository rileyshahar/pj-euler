//! What is the value of the first triangle number to have over five hundred divisors?
use crate::utils::primes::number_of_divisors;
use crate::utils::seqs::Triangle;

fn solve_for(divisors: u32) -> u32 {
    Triangle::new()
        .find(|&n| number_of_divisors(n) > divisors)
        .expect("infinitely many triangle numbers")
}

super::example!(5 => 28);
super::problem!(u32: 500 => 76_576_500);
