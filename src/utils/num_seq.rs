//! A generic trait for iterators of numbers.

use std::iter::Step;

use num::Num;

/// Traits needed for a `NumIter`.
pub trait NumTraits = Num + PartialOrd + Copy + Step;

/// An iterator over numbers.
pub trait NumIter: Iterator<Item = Self::NumItem> + Sized {
    type NumItem: NumTraits;
}

impl<N: NumTraits, T: Iterator<Item = N>> NumIter for T {
    type NumItem = T::Item;
}
