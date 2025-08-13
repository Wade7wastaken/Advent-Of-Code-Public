use std::fmt::Display;
use std::ops::Add;
use std::ops::Range as RustRange;
use std::ops::RangeInclusive as RustRangeInclusive;
use std::ops::Sub;

use num::One;

// Represents [a, b)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range<T> {
    a: T,
    b: T,
}

impl<T> Range<T> {
    /// # Safety
    /// Caller is responsible for ensuring start <= end
    pub unsafe fn new_unchecked(start: T, end: T) -> Self {
        Self { a: start, b: end }
    }

    pub fn start(&self) -> &T {
        &self.a
    }

    pub fn end(&self) -> &T {
        &self.b
    }
}

impl<T: PartialOrd + Display> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {start} can't be larger than end {end}");
        Self { a: start, b: end }
    }
}

impl<T: Add<Output = T> + One + PartialOrd + Display> Range<T> {
    pub fn new_inclusive(start: T, end: T) -> Self {
        Self::new(start, end + T::one())
    }
}

impl<T: PartialOrd + Display> From<RustRange<T>> for Range<T> {
    fn from(val: RustRange<T>) -> Self {
        Self::new(val.start, val.end)
    }
}

impl<T: Add<Output = T> + One + PartialOrd + Display> From<RustRangeInclusive<T>> for Range<T> {
    fn from(val: RustRangeInclusive<T>) -> Self {
        let (start, end) = val.into_inner();
        Self::new_inclusive(start, end)
    }
}

impl<T: PartialOrd> Range<T> {
    pub fn is_empty(&self) -> bool {
        self.a == self.b
    }

    pub fn contains(&self, x: &T) -> bool {
        self.a <= *x && self.b > *x
    }

    pub fn contains_range(&self, r: &Range<T>) -> bool {
        self.a <= r.a && self.b >= r.b
    }

    pub fn overlaps(&self, r: &Range<T>) -> bool {
        if r.is_empty() {
            return self.contains(&r.a);
        }
        self.a < r.b && r.a < self.b
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Range<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        unsafe { Self::new_unchecked(self.a + rhs, self.b + rhs) }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Range<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        unsafe { Self::new_unchecked(self.a - rhs, self.b - rhs) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rust_range() {
        let r: Range<i32> = (0..100).into();
        assert_eq!(r, Range::new(0, 100));
        let r: Range<i32> = (0..=100).into();
        assert_eq!(r, Range::new_inclusive(0, 100));
    }

    #[test]
    fn contains() {
        let range = Range::new(0, 100);
        assert!(range.contains(&0));
        assert!(range.contains(&50));
        assert!(range.contains(&99));
        assert!(!range.contains(&100));
        assert!(!range.contains(&-1));
        assert!(!range.contains(&101));
        let range = Range::new_inclusive(0, 100);
        assert!(range.contains(&0));
        assert!(range.contains(&50));
        assert!(range.contains(&99));
        assert!(range.contains(&100));
        assert!(!range.contains(&-1));
        assert!(!range.contains(&101));
    }

    #[test]
    fn contains_range() {
        let range = Range::new(0, 100);
        assert!(range.contains_range(&Range::new(0, 100)));
        assert!(range.contains_range(&Range::new(0, 25)));
        assert!(range.contains_range(&Range::new(50, 75)));
        assert!(range.contains_range(&Range::new(100, 100)));
        assert!(!range.contains_range(&Range::new(101, 101)));
        assert!(!range.contains_range(&Range::new(-1, 101)));
        assert!(!range.contains_range(&Range::new(-5, -1)));
        assert!(!range.contains_range(&Range::new(101, 105)));
        assert!(!range.contains_range(&Range::new(-5, 95)));
    }

    #[test]
    fn overlaps() {
        let range = Range::new(0, 100);
        assert!(range.overlaps(&Range::new(0, 100)));

        assert!(range.overlaps(&Range::new(50, 75)));

        assert!(range.overlaps(&Range::new(0, 0)));
        assert!(range.overlaps(&Range::new(50, 50)));
        assert!(!range.overlaps(&Range::new(100, 100)));

        assert!(!range.overlaps(&Range::new(101, 105)));
        assert!(!range.overlaps(&Range::new(-5, -1)));

        assert!(!range.overlaps(&Range::new(-5, 0)));
        assert!(!range.overlaps(&Range::new(100, 105)));

        assert!(range.overlaps(&Range::new(-1, 101)));
        assert!(range.overlaps(&Range::new(-5, 95)));
        assert!(range.overlaps(&Range::new(5, 105)));
    }

    #[test]
    fn ops() {
        let range = Range::new(0, 100);
        assert_eq!(range.clone() + 5, Range::new(5, 105));
        assert_eq!(range - 5, Range::new(-5, 95));
    }
}
