use std::{
    hash::Hash,
    ops::{Add, Mul},
};

use crate::{tern, Inline};

#[derive(Debug, Default, Clone, Eq)]
pub struct DigitIter {
    n: u32,
    length: usize,
}

#[must_use]
pub const fn integer_length(n: u32) -> usize {
    tern!(n == 0, 1, n.ilog10() as usize + 1)
}

impl DigitIter {
    #[must_use]
    pub const fn new(n: u32) -> Self {
        Self {
            n,
            length: integer_length(n),
        }
    }

    #[must_use]
    pub const fn value(&self) -> u32 {
        self.n
    }

    pub fn add_left<N: Into<u32>>(&mut self, d: N) {
        let d = d.into();
        assert!((d < 9), "{d} is not a digit");
        self.n += d * 10u32.pow(self.length as u32);
        self.length += 1;
    }

    #[must_use]
    pub fn added_left<N: Into<u32>>(self, d: N) -> Self {
        self.inline(|s| s.add_left(d))
    }

    pub fn add_right<N: Into<u32>>(&mut self, d: N) {
        let d = d.into();
        assert!(d < 10, "{d} is not a digit");
        self.n *= 10;
        self.n += d;
        self.length += 1;
    }

    #[must_use]
    pub fn added_right<N: Into<u32>>(self, d: N) -> Self {
        self.inline(|s| s.add_right(d))
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl Iterator for DigitIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.length = self.length.checked_sub(1)?;
        let mask = 10u32.pow(self.length as u32);
        let digit = self.n / mask;
        self.n %= mask;
        Some(digit as u8)
    }
}

impl DoubleEndedIterator for DigitIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.length = self.length.checked_sub(1)?;
        let digit = self.n % 10;
        self.n /= 10;
        Some(digit as u8)
    }
}

impl PartialEq for DigitIter {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Hash for DigitIter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.n.hash(state);
    }
}

impl Ord for DigitIter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.n.cmp(&other.n)
    }
}

impl PartialOrd for DigitIter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub trait CollectDigits<T> {
    fn collect_digits(self) -> T;
}

impl<T: Mul<Output = T> + Add<Output = T> + From<u8>, I: Iterator<Item = T>> CollectDigits<T>
    for I
{
    fn collect_digits(self) -> T {
        self.fold(T::from(0), |acc, n| (acc * T::from(10u8)) + n)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn iter() {
        let digits = DigitIter::new(123456789);
        assert_eq!(
            digits.clone().collect_vec(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(digits.rev().collect_vec(), vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn zeros() {
        let digits = DigitIter::new(1000);
        assert_eq!(digits.clone().collect_vec(), vec![1, 0, 0, 0]);
        assert_eq!(digits.rev().collect_vec(), vec![0, 0, 0, 1]);
    }

    #[test]
    fn post_state() {
        let mut digits = DigitIter::new(1234);
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(3));
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), None);
        assert_eq!(digits.n, 0);
        assert_eq!(digits.length, 0);
    }

    #[test]
    fn zero() {
        let digits = DigitIter::new(0);
        assert_eq!(digits.clone().collect_vec(), vec![0]);
        assert_eq!(digits.rev().collect_vec(), vec![0]);
    }

    #[test]
    fn building() {
        let mut builder = DigitIter::default();
        assert_eq!(builder.next(), None);
        let builder = builder
            .added_left(1u32)
            .added_left(0u32)
            .added_left(3u32)
            .added_right(9u32)
            .added_right(2u32);
        assert_eq!(builder.value(), 30192);
    }
}
