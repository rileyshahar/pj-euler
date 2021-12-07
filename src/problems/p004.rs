//! Find the largest palindrome made from the product of two 3-digit numbers.
use crate::utils;
use itertools::Itertools;

const fn is_palindrome(n: u32) -> bool {
    let mut m = n;
    let mut rev = 0;
    while m != 0 {
        rev = m % 10 + rev * 10;
        m /= 10;
    }
    rev == n
}

fn solve_for(bound: u32) -> u32 {
    (11..bound)
        // easy to show a palindrome has to be divisible by 11 (consider n as a sum of scaled
        // powers of 10)
        .step_by(11)
        .cartesian_product(1..bound)
        .map(|(n, m)| n * m)
        .filter(|&n| is_palindrome(n))
        .max()
        .expect("there are palindromes in the range")
}

utils::test::example!(100 => 9009);
utils::test::problem!(u32: 1000 => 906_609);

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    const fn is_palindrime_twelve() {
        assert!(!is_palindrome(12));
    }

    #[test]
    const fn is_palindrime_large() {
        assert!(is_palindrome(100_343_001));
    }
}
