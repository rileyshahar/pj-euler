//! Utilities relating to prime numbers.

use super::num_fn::{_0, _1, _2, _3};
use super::num_seq::NumTraits;

/// Check whether n is prime.
///
/// # Examples
/// ```
/// use pj_euler::utils::primes::is_prime;
///
/// assert!(is_prime(2));
/// assert!(is_prime(5));
/// assert!(!is_prime(6));
/// assert!(!is_prime(9));
/// assert!(is_prime(17));
/// ```
#[must_use]
pub fn is_prime<N>(n: N) -> bool
where
    N: NumTraits,
{
    if n == _0() || n == _1() {
        return false;
    }
    if n == _2() {
        return true;
    }
    if n % _2() == _0() {
        return false;
    }

    // only have to check odd factors
    for i in (_3()..).step_by(2).take_while(|&x| x * x <= n) {
        if n % i == _0() {
            return false;
        }
    }
    true
}

/// An iterator of primes.
///
/// # Examples
/// ```
/// # use pj_euler::utils::primes::Primes;
/// let mut p = Primes::new();
/// assert_eq!(p.next(), Some(2));
/// assert_eq!(p.next(), Some(3));
/// assert_eq!(p.next(), Some(5));
/// assert_eq!(p.next(), Some(7));
/// assert_eq!(p.next(), Some(11));
/// ```
///
/// ```
/// # use pj_euler::utils::primes::Primes;
/// let mut p = Primes::<u32>::new();
/// assert_eq!(p.nth(999).expect("there are infinitely many primes"), 7919);
/// ```
pub struct Primes<T> {
    computed: Vec<T>,
}

impl<T> Primes<T> {
    /// Make a new primes iterator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            computed: Vec::default(),
        }
    }
}

impl<T> Default for Primes<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: NumTraits> Iterator for Primes<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.computed.last() {
            None => _2(),                  // 2 is the first prime
            Some(&n) if n == _2() => _3(), // need a special case so we can step by 2
            Some(&n) => ((n + _2())..)
                .step_by(2)
                .find(|&p| {
                    self.computed
                        .iter()
                        .copied()
                        .take_while(|&x| x * x <= p)
                        .all(|x| p % x != _0())
                })
                .expect("there are infinitely many primes"),
        };

        self.computed.push(out);
        Some(out)
    }
}

#[cfg(test)]
mod benches {
    use super::{is_prime, Primes};

    extern crate test;
    use test::Bencher;

    #[bench]
    fn is_prime_bench(b: &mut Bencher) {
        b.iter(|| {
            for i in 6_000_000..6_000_100 {
                let _ = is_prime(i);
            }
        });
    }

    #[bench]
    fn first_thousand_primes(b: &mut Bencher) {
        b.iter(|| {
            let _ = Primes::<usize>::new()
                .take(10000)
                .collect::<Vec<_>>()
                .last();
        });
    }
}
