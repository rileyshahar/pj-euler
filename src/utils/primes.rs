//! Utilities relating to prime numbers.

use std::iter::Step;

use num::Num;

use super::num_fn::{_0, _1, _2, _3};

/// Check whether n is prime.
///
/// # Examples
/// ```
/// # use pj_euler::utils::primes::is_prime;
/// assert!(is_prime(2));
/// assert!(is_prime(5));
/// assert!(!is_prime(6));
/// assert!(!is_prime(9));
/// assert!(is_prime(17));
/// ```
#[must_use]
pub fn is_prime<N>(n: N) -> bool
where
    N: Num + Step + PartialOrd + Copy,
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

impl<N: Num + Step + PartialOrd + Copy> Iterator for Primes<N> {
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

/// A single prime factor.
#[derive(Debug, PartialEq)]
pub struct PrimeFactor<T> {
    pub factor: T,
    pub exponent: T,
}

/// An iterator over the prime factorization of a number in ascending order.
///
/// # Examples
/// ```
/// # use pj_euler::utils::primes::{PrimeFactor, PrimeFactorization};
/// let mut pf = PrimeFactorization::<u32>::of(40);
/// assert_eq!(pf.next(), Some(PrimeFactor { factor: 2, exponent: 3 }));
/// assert_eq!(pf.next(), Some(PrimeFactor { factor: 5, exponent: 1 }));
/// assert_eq!(pf.next(), None);
/// ```
pub struct PrimeFactorization<T> {
    num: T,
    factor: T,
}

impl<N: Num> PrimeFactorization<N> {
    /// Create a prime factorization.
    pub fn of(num: N) -> Self {
        Self { num, factor: _2() }
    }
}

impl<N: Num + PartialOrd + Copy> Iterator for PrimeFactorization<N> {
    type Item = PrimeFactor<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num <= _1() {
            None
        } else {
            // find the next prime factor of self.num
            while self.num % self.factor != _0() {
                if self.factor == _2() {
                    self.factor = self.factor + _1();
                } else {
                    self.factor = self.factor + _2();
                }
            }

            // compute the corresponding exponent
            let mut exponent = _1();
            self.num = self.num / self.factor;
            while self.num % self.factor == _0() {
                self.num = self.num / self.factor;
                exponent = exponent + _1();
            }
            Some(PrimeFactor {
                factor: self.factor,
                exponent,
            })
        }
    }
}

/// Determine the total number of divisors the number.
///
/// # Examples
/// ```
/// # use pj_euler::utils::primes::number_of_divisors;
/// assert_eq!(number_of_divisors(28), 6);
/// ```
/// ```
/// # use pj_euler::utils::primes::number_of_divisors;
/// assert_eq!(number_of_divisors(2), 2);
/// ```
/// ``` # use pj_euler::utils::primes::number_of_divisors;
/// assert_eq!(number_of_divisors(1), 1);
/// ```
pub fn number_of_divisors<N: Num + PartialOrd + Copy>(of: N) -> N {
    PrimeFactorization::of(of)
        .map(|PrimeFactor { exponent, .. }| exponent + _1())
        .fold(_1(), |p, n| p * n)
}

#[cfg(test)]
mod benches {
    use super::{is_prime, PrimeFactorization, Primes};

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
    fn thousanth_prime(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(Primes::<usize>::new().nth(999), Some(7919));
        });
    }

    #[bench]
    fn factorize_large_n(b: &mut Bencher) {
        b.iter(|| for _factor in PrimeFactorization::of(6_002_462) {});
    }
}
