//! Find the sum of all the multiples of 3 or 5 below 1000.
use crate::utils;

fn solve_for(bound: u32) -> u32 {
    (0..bound).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

utils::test::example!(10 => 23);
utils::test::problem!(u32: 1000 => 233_168);
