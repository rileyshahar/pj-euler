//! Iterators of various numeric sequences.

use num::{Num, NumCast};

use super::num_fn::{_0, _1, _2};

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
