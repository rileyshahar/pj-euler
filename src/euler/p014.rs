//! Find the collatz number under 1,000,000 with the longest sequence.
use std::collections::HashMap;

use crate::utils::seqs::collatz_length;

fn solve_for(bound: u64) -> u64 {
    ((bound / 2)..bound)
        .scan(HashMap::with_capacity((bound / 2) as usize), |hm, n| {
            Some((n, collatz_length(n, hm)))
        })
        .max_by_key(|&(_, l)| l)
        .expect("the iterator is non-empty")
        .0
}

super::example!(10 => 9);
super::problem!(u64: 1_000_000 => 837_799);
