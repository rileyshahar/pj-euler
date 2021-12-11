//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
const fn solve_for(bound: u32) -> u32 {
    // pretty sure the compiler does all this at const time anyway, but it's simplified to be sure
    // uses the facts that sum_{i=0}^n i = n(n+1)/2 and sum_{i=0}^n i^2 = (2n+1)(n+1)n/6
    (bound * (bound + 1) / 2).pow(2) - (2 * bound + 1) * (bound + 1) * bound / 6
}

super::example!(10 => 2640);
super::problem!(u32: 100 => 25_164_150);
