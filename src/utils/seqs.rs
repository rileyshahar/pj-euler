//! Iterators of various numeric sequences.

use std::collections::HashMap;

use num::{Num, NumCast};

use super::num_fn::{_0, _1, _2, _3};

/// An iterator over the Triangle numbers, defined by
///
/// ```text
/// T(0) = 0
/// T(n) = T(n-1) + n = n(n + 1)/2
/// ```
///
/// We also implement `Iterator::nth` manually using the explicit formula. It gets the nth of the
/// remaining sequence, not the nth from the beginning.
///
/// # Examples
/// ```
/// # use pj_euler::utils::seqs::Triangle;
/// let mut t = Triangle::new();
/// assert_eq!(t.next(), Some(0));
/// assert_eq!(t.next(), Some(1));
/// assert_eq!(t.next(), Some(3));
/// assert_eq!(t.next(), Some(6));
/// ```
/// ```
/// # use pj_euler::utils::seqs::Triangle;
/// assert_eq!(Triangle::new().nth(0), Some(0));
/// assert_eq!(Triangle::new().nth(1000), Some(500_500));
/// ```
pub struct Triangle<T> {
    // The previously yielded triangle number.
    last: T,

    // The number of triangle numbers yielded so far.
    count: T,
}

impl<N: Num> Default for Triangle<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Num> Triangle<N> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            last: _0(),
            count: _0(),
        }
    }
}

impl<N: Num + NumCast + Copy> Iterator for Triangle<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.last = self.last + self.count;
        self.count = self.count + _1();
        Some(self.last)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        // we are computing T(n)
        let n = self.count + num::cast(n).unwrap();

        // uses the fact that T(n) = n(n+1)/2
        self.last = n * (n + _1()) / _2();
        self.count = n + _1();

        Some(self.last)
    }
}

/// An iterator over the Collatz sequence starting from n, defined by
///
/// ```text
/// n -> 3n + 1    n odd
/// n -> 2n        n even
/// ```
///
/// # Examples
/// ```
/// # use pj_euler::utils::seqs::Collatz;
/// let mut c = Collatz::<u32>::of(6);
/// assert_eq!(c.next(), Some(6));
/// assert_eq!(c.next(), Some(3));
/// assert_eq!(c.next(), Some(10));
/// assert_eq!(c.next(), Some(5));
/// assert_eq!(c.next(), Some(16));
/// assert_eq!(c.next(), Some(8));
/// assert_eq!(c.next(), Some(4));
/// assert_eq!(c.next(), Some(2));
/// assert_eq!(c.next(), Some(1));
/// ```
///
/// ```
/// # use pj_euler::utils::seqs::Collatz;
/// let mut c = Collatz::<u32>::of(4);
/// assert_eq!(c.next(), Some(4));
/// assert_eq!(c.next(), Some(2));
/// assert_eq!(c.next(), Some(1));
///
/// // loops infinitely
/// assert_eq!(c.next(), Some(4));
/// ```
pub struct Collatz<T> {
    n: T,
}

impl<T> Collatz<T> {
    /// Create the Collatz sequence of `n`.
    pub const fn of(n: T) -> Self {
        Self { n }
    }
}

impl<N: Num + Copy + PartialOrd> Collatz<N> {
    /// Stop at the first occurance of `1`, stopping the cycle `4->2->1->2`.
    ///
    /// # Example
    /// ```
    /// # use pj_euler::utils::seqs::Collatz;
    /// let mut c = Collatz::<u32>::of(4).cut_off();
    /// assert_eq!(c.next(), Some(4));
    /// assert_eq!(c.next(), Some(2));
    /// assert_eq!(c.next(), Some(1));
    /// assert_eq!(c.next(), None);
    /// ```
    pub fn cut_off(self) -> impl Iterator<Item = N> {
        self.take_while(|&x| x != _1()).chain(std::iter::once(_1()))
    }
}

impl<N: Num + Copy + PartialOrd> Iterator for Collatz<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n;
        if self.n % _2() == _0() {
            self.n = self.n / _2();
        } else {
            self.n = self.n * _3() + _1();
        }
        Some(out)
    }
}
/// Adds the collatz length of n (and maybe other numbers) to the provided map.
///
/// A lot of computation can easily be shared through this process.
///
/// # Example
/// ```
/// # use std::collections::HashMap;
/// # use pj_euler::utils::seqs::collatz_length;
/// assert_eq!(collatz_length(17, &mut HashMap::new()), 13);
/// ```
pub fn collatz_length<
    N: Num + Copy + PartialEq + Eq + std::hash::Hash,
    S: std::hash::BuildHasher,
>(
    n: N,
    lengths: &mut HashMap<N, usize, S>,
) -> usize {
    #[allow(clippy::option_if_let_else)] // required for borrow checker, I think
    if let Some(l) = lengths.get(&n) {
        *l
    } else {
        let l = if n == _1() {
            1
        } else if n % _2() == _0() {
            collatz_length(n / _2(), lengths) + 1
        } else {
            collatz_length((n * _3() + _1()) / _2(), lengths) + 2
        };
        lengths.insert(n, l);
        l
    }
}

#[cfg(test)]
mod benches {
    use super::Triangle;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn triangle_nth(b: &mut Bencher) {
        b.iter(|| {
            let mut t = Triangle::<u64>::new();
            assert_eq!(t.nth(100_000_000), Some(5_000_000_050_000_000));
            assert_eq!(t.next(), Some(5_000_000_050_000_000 + 100_000_001));
        });
    }
}
