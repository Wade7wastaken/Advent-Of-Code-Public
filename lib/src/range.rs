use std::ops::Add;
use std::ops::Range as RustRange;
use std::ops::RangeInclusive as RustRangeInclusive;
use std::ops::Sub;

use num::Num;

use crate::tern;

// Represents [a, b) or [a, b] if inclusive is true
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Range<T> {
    a: T,
    b: T,
    inclusive: bool,
}

impl<T> From<RustRange<T>> for Range<T> {
    fn from(val: RustRange<T>) -> Self {
        Range {
            a: val.start,
            b: val.end,
            inclusive: false,
        }
    }
}

impl<T> From<RustRangeInclusive<T>> for Range<T> {
    fn from(val: RustRangeInclusive<T>) -> Self {
        let (start, end) = val.into_inner();
        Range {
            a: start,
            b: end,
            inclusive: true,
        }
    }
}

impl<T> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Self {
            a: start,
            b: end,
            inclusive: false,
        }
    }

    pub fn new_inclusive(start: T, end: T) -> Self {
        Self {
            a: start,
            b: end,
            inclusive: true,
        }
    }
}

impl<T: Num + PartialOrd> Range<T> {
    pub fn contains(&self, x: &T) -> bool {
        tern!(
            self.inclusive,
            self.a <= *x && self.b >= *x,
            self.a <= *x && self.b > *x
        )
    }

    pub fn contains_range(&self, r: &Range<T>) -> bool {
        if self.inclusive && !r.inclusive && self.b == r.b {
            return false;
        }
        // inclusiveness doesn't matter
        self.a <= r.a && self.b >= r.b
    }

    pub fn overlaps(&self, r: &Range<T>) -> bool {
        // tern!(self.inclusive,
        //     self.a.max(r.a) - self.b.min(r.b) <= 0,
        //     self.a.max(r.a) - self.b.min(r.b) <= 0,
        // )
        self.a <= r.b && r.a < self.b
    }
}

impl<T: Num + Copy> Add<T> for Range<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.a + rhs, self.b + rhs)
    }
}

impl<T: Num + Copy> Sub<T> for Range<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self::new(self.a - rhs, self.b - rhs)
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
        assert!(range.contains_range(&Range::new(50, 75)));
        assert!(range.contains_range(&Range::new(100, 100)));
        assert!(!range.contains_range(&Range::new(101, 101)));
        assert!(!range.contains_range(&Range::new(-1, 101)));
        assert!(!range.contains_range(&Range::new(-5, -1)));
        assert!(!range.contains_range(&Range::new(101, 105)));
        assert!(!range.contains_range(&Range::new(-5, 95)));
    }

    // #[test]
    // fn overlaps() {
    //     let range = Range::new(0, 100);
    //     assert!(range.overlaps(&Range::new(0, 100)));
    //     assert!(range.overlaps(&Range::new(50, 75)));
    //     assert!(range.overlaps(&Range::new(100, 100)));
    //     assert!(!range.overlaps(&Range::new(101, 101)));
    //     assert!(range.overlaps(&Range::new(-1, 101)));
    //     assert!(!range.overlaps(&Range::new(-5, 0)));
    //     assert!(!range.overlaps(&Range::new(100, 105)));
    //     assert!(range.overlaps(&Range::new(-5, 95)));
    // }

    #[test]
    fn ops() {
        let range = Range::new(0, 100);
        assert_eq!(range.clone() + 5, Range::new(5, 105));
        assert_eq!(range - 5, Range::new(-5, 95));
    }
}
