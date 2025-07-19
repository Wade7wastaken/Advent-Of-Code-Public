use std::hash::Hash;

#[derive(Debug, Default, Clone, Eq)]
pub struct DigitIter {
    n: u32,
    length: usize,
}

#[must_use]
pub fn integer_length(n: u32) -> usize {
    if n == 0 {
        return 1;
    }
    n.ilog10() as usize + 1
}

impl DigitIter {
    #[must_use]
    pub fn new(n: u32) -> Self {
        Self {
            n,
            length: integer_length(n),
        }
    }

    #[must_use]
    pub fn value(&self) -> u32 {
        self.n
    }

    pub fn add_left<N: Into<u32>>(&mut self, d: N) {
        let d = d.into();
        assert!((d < 9), "{d} is not a digit");
        self.n += d * 10u32.pow(self.length as u32);
        self.length += 1;
    }

    #[must_use]
    pub fn added_left<N: Into<u32>>(mut self, d: N) -> Self {
        self.add_left(d);
        self
    }

    pub fn add_right<N: Into<u32>>(&mut self, d: N) {
        let d = d.into();
        assert!(d < 10, "{d} is not a digit");
        self.n *= 10;
        self.n += d;
        self.length += 1;
    }

    #[must_use]
    pub fn added_right<N: Into<u32>>(mut self, d: N) -> Self {
        self.add_right(d);
        self
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl Iterator for DigitIter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None;
        }
        if self.length == 1 {
            let digit = self.n;
            self.n = 0;
            self.length = 0;
            return Some(digit);
        }
        let mask = 10u32.pow(self.length as u32 - 1);
        let digit = self.n / mask;
        self.n %= mask;
        self.length -= 1;
        Some(digit)
    }
}

impl DoubleEndedIterator for DigitIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None;
        }
        let digit = self.n % 10;
        self.n /= 10;
        self.length -= 1;
        Some(digit)
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
