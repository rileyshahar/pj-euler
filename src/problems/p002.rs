//! Find the sum of the even-valued Fibonacci numbers leq 4,000,000.
use crate::utils;

fn solve_for(bound: u32) -> u32 {
    let (mut prev, mut curr) = (1, 2);
    let mut out = 0;

    while curr < bound {
        out += curr;
        for _ in 0..3 {
            (prev, curr) = (curr, prev + curr);
        }
    }

    out
}

utils::test::example!(100 => 44);
utils::test::problem!(u32: 4_000_000 => 4_613_732);
