use std::ops::{Add, Range as RustRange, RangeInclusive as RustInclusiveRange, Sub};

/// Trait for any ranged data structure.
pub trait Ranged<T: Copy>: Copy {
    /// Returns whether the range is empty.
    fn is_empty(self) -> bool;

    /// Returns whether the range contains a value.
    fn contains(self, x: T) -> bool;

    /// Returns whether or not this range completely covers another range.
    fn covers(self, r: Self) -> bool;

    /// Returns whether this range overlaps with another range. Two ranges
    /// overlap if they have a value in common.
    fn overlaps(self, r: Self) -> bool;

    /// Returns whether this range touches another range. Touching ranges don't
    /// necessarily have common values, but there are no values between them
    fn touches(self, r: Self) -> bool;

    /// Returns the union of two ranges. If the ranges don't touch, None is
    /// returned.
    fn union(self, r: Self) -> Option<Self>
    where
        T: Ord;

    /// Returns the intersection of two ranges. If the ranges don't overlap,
    /// None is returned.
    fn intersection(self, r: Self) -> Option<Self>
    where
        T: Ord;

    type RemoveResult;

    /// Removes a range from this range.
    fn remove(self, other: Self) -> Self::RemoveResult;
}

// Represents [a, b). If a and b are the same, the range is empty, but it can
// still overlap and touch with other ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Range<T: Copy> {
    start: T,
    end: T,
}

impl<T: Copy> Range<T> {
    /// Creates a new `Range` without ensuring start <= end.
    ///
    /// # Safety
    /// Caller is responsible for ensuring start <= end.
    pub const unsafe fn new_unchecked(start: T, end: T) -> Self {
        Self { start, end }
    }

    /// Creates a new `Range` and returns None if start is greater than end.
    pub fn try_new(start: T, end: T) -> Option<Self>
    where
        T: PartialOrd,
    {
        (start <= end).then_some(unsafe { Self::new_unchecked(start, end) })
    }

    /// Creates a new `Range` and panics if start is greater than end.
    pub fn new(start: T, end: T) -> Self
    where
        T: PartialOrd,
    {
        Self::try_new(start, end).expect("start can't be larger than end")
    }

    /// Creates a new `Range` from a start and a length.
    pub fn new_by_len(start: T, len: T) -> Self
    where
        T: PartialOrd + Add<Output = T>,
    {
        Self::new(start, start + len)
    }

    /// Returns the start of the `Range`.
    pub const fn start(self) -> T {
        self.start
    }

    /// Returns the end of the `Range`.
    pub const fn end(&self) -> T {
        self.end
    }
}

impl<T: Copy + PartialOrd> Ranged<T> for Range<T> {
    fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Range is non-inclusive, so start is included, but end is not.
    fn contains(self, x: T) -> bool {
        self.start <= x && self.end > x
    }

    fn covers(self, r: Self) -> bool {
        if r.is_empty() {
            return self.contains(r.start);
        }
        self.start <= r.start && self.end >= r.end
    }

    /// If self is empty, contains is used to check for an overlap.
    fn overlaps(self, r: Self) -> bool {
        if r.is_empty() {
            return self.contains(r.start);
        }
        self.start < r.end && r.start < self.end
    }

    fn touches(self, r: Self) -> bool {
        self.start <= r.end && r.start <= self.end
    }

    fn union(self, r: Self) -> Option<Self>
    where
        T: Ord,
    {
        if r.is_empty() {
            return Some(self);
        }

        self.touches(r)
            .then_some(unsafe { Self::new_unchecked(self.start.min(r.start), self.end.max(r.end)) })
    }

    fn intersection(self, r: Self) -> Option<Self>
    where
        T: Ord,
    {
        self.overlaps(r)
            .then_some(unsafe { Self::new_unchecked(self.start.max(r.start), self.end.min(r.end)) })
    }

    type RemoveResult = Vec<Self>;

    fn remove(self, other: Self) -> Self::RemoveResult {
        if !self.overlaps(other) {
            return vec![self];
        }

        let mut ans = vec![];

        if other.covers(self) {
            return ans;
        }

        if self.start < other.start {
            ans.push(unsafe { Range::new_unchecked(self.start, other.start) });
        }

        if self.end > other.end {
            ans.push(unsafe { Range::new_unchecked(other.end, self.end) });
        }

        ans
    }
}

impl<T: Copy + PartialOrd> From<RustRange<T>> for Range<T> {
    fn from(val: RustRange<T>) -> Self {
        Self::new(val.start, val.end)
    }
}

impl<T: Copy> From<InclusiveRange<T>> for Range<T> {
    fn from(val: InclusiveRange<T>) -> Self {
        unsafe { Range::new_unchecked(val.start, val.end) }
    }
}

impl<T: Copy + PartialOrd> PartialOrd for Range<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl<T: Copy + Ord> Ord for Range<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Range<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        unsafe { Self::new_unchecked(self.start + rhs, self.end + rhs) }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Range<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        unsafe { Self::new_unchecked(self.start - rhs, self.end - rhs) }
    }
}

#[must_use]
pub fn range<T: Copy + Ord>(start: T, end: T) -> Range<T> {
    Range::new(start, end)
}

// Represents [a, b]. This range can never be empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InclusiveRange<T: Copy> {
    start: T,
    end: T,
}

impl<T: Copy> InclusiveRange<T> {
    /// Creates a new `InclusiveRange` without ensuring start <= end.
    ///
    /// # Safety
    /// Caller is responsible for ensuring start <= end.
    pub const unsafe fn new_unchecked(start: T, end: T) -> Self {
        Self { start, end }
    }

    /// Creates a new `InclusiveRange` and returns None if start is greater than
    /// end.
    pub fn try_new(start: T, end: T) -> Option<Self>
    where
        T: PartialOrd,
    {
        (start <= end).then_some(unsafe { Self::new_unchecked(start, end) })
    }

    /// Creates a new `InclusiveRange` and panics if start is greater than
    /// end.
    pub fn new(start: T, end: T) -> Self
    where
        T: PartialOrd,
    {
        Self::try_new(start, end).expect("start can't be larger than end")
    }

    /// Returns the start of the `InclusiveRange`.
    pub const fn start(self) -> T {
        self.start
    }

    /// Returns the end of the `InclusiveRange`.
    pub const fn end(self) -> T {
        self.end
    }
}

impl<T: Copy + PartialOrd> Ranged<T> for InclusiveRange<T> {
    fn is_empty(self) -> bool {
        false
    }

    fn contains(self, x: T) -> bool {
        self.start <= x && self.end >= x
    }

    fn covers(self, r: Self) -> bool {
        self.start <= r.start && self.end >= r.end
    }

    fn overlaps(self, r: Self) -> bool {
        self.start <= r.end && r.start <= self.end
    }

    fn touches(self, r: Self) -> bool {
        if r.is_empty() {
            return self.contains(r.start);
        }
        self.start <= r.end && r.start <= self.end
    }

    fn union(self, r: Self) -> Option<Self>
    where
        T: Ord,
    {
        (self.touches(r))
            .then_some(unsafe { Self::new_unchecked(self.start.min(r.start), self.end.max(r.end)) })
    }

    fn intersection(self, r: Self) -> Option<Self>
    where
        T: Ord,
    {
        self.overlaps(r)
            .then_some(unsafe { Self::new_unchecked(self.start.max(r.start), self.end.min(r.end)) })
    }

    type RemoveResult = Vec<Self>;

    fn remove(self, other: Self) -> Self::RemoveResult {
        if !self.overlaps(other) {
            return vec![self];
        }

        let mut ans = vec![];

        if other.covers(self) {
            return ans;
        }

        if self.start < other.start {
            ans.push(unsafe { InclusiveRange::new_unchecked(self.start, other.start) });
        }

        if self.end > other.end {
            ans.push(unsafe { InclusiveRange::new_unchecked(other.end, self.end) });
        }

        ans
    }
}

impl<T: Copy + PartialOrd> From<RustInclusiveRange<T>> for InclusiveRange<T> {
    fn from(val: RustInclusiveRange<T>) -> Self {
        let (start, end) = val.into_inner();
        Self::new(start, end)
    }
}

impl<T: Copy> From<Range<T>> for InclusiveRange<T> {
    fn from(val: Range<T>) -> Self {
        unsafe { InclusiveRange::new_unchecked(val.start, val.end) }
    }
}

impl<T: Copy + PartialOrd> PartialOrd for InclusiveRange<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl<T: Copy + Ord> Ord for InclusiveRange<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for InclusiveRange<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        unsafe { Self::new_unchecked(self.start + rhs, self.end + rhs) }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for InclusiveRange<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        unsafe { Self::new_unchecked(self.start - rhs, self.end - rhs) }
    }
}

#[must_use]
pub fn inclusive_range<T: Copy + Ord>(start: T, end: T) -> InclusiveRange<T> {
    InclusiveRange::new(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rust_range() {
        let r: Range<i32> = (0..100).into();
        assert_eq!(r, range(0, 100));

        let r: InclusiveRange<i32> = (0..=100).into();
        assert_eq!(r, inclusive_range(0, 100));
    }

    #[test]
    fn contains() {
        let r = range(0, 100);
        assert!(r.contains(0));
        assert!(r.contains(50));
        assert!(r.contains(99));
        assert!(!r.contains(100));
        assert!(!r.contains(-1));
        assert!(!r.contains(101));

        let r = inclusive_range(0, 100);
        assert!(r.contains(0));
        assert!(r.contains(50));
        assert!(r.contains(99));
        assert!(r.contains(100));
        assert!(!r.contains(-1));
        assert!(!r.contains(101));
    }

    #[test]
    fn covers() {
        let r = range(0, 100);
        assert!(r.covers(range(0, 100))); // same

        assert!(r.covers(range(0, 25))); // inner low
        assert!(r.covers(range(50, 75))); // inner middle
        assert!(r.covers(range(75, 100))); // inner high

        assert!(!r.covers(range(-1, -1))); // unit outside low
        assert!(r.covers(range(0, 0))); // unit low
        assert!(r.covers(range(50, 50))); // unit middle
        assert!(!r.covers(range(100, 100))); // unit high
        assert!(!r.covers(range(101, 101))); // unit outside high

        assert!(!r.covers(range(-1, 101))); // outside both
        assert!(!r.covers(range(-5, -1))); // outside low
        assert!(!r.covers(range(101, 105))); // outside high

        assert!(!r.covers(range(-5, 0))); // touching low
        assert!(!r.covers(range(100, 105))); // touching high

        assert!(!r.covers(range(-1, 99))); // leaking low
        assert!(!r.covers(range(1, 101))); // leaking high

        let r = inclusive_range(0, 100);
        assert!(r.covers(inclusive_range(0, 100))); // same

        assert!(r.covers(inclusive_range(0, 25))); // inner low
        assert!(r.covers(inclusive_range(50, 75))); // inner middle
        assert!(r.covers(inclusive_range(75, 100))); // inner high

        assert!(!r.covers(inclusive_range(-1, -1))); // unit outside low
        assert!(r.covers(inclusive_range(0, 0))); // unit low
        assert!(r.covers(inclusive_range(50, 50))); // unit middle
        assert!(r.covers(inclusive_range(100, 100))); // unit high
        assert!(!r.covers(inclusive_range(101, 101))); // unit outside high

        assert!(!r.covers(inclusive_range(-1, 101))); // outside both
        assert!(!r.covers(inclusive_range(-5, -1))); // outside low
        assert!(!r.covers(inclusive_range(101, 105))); // outside high

        assert!(!r.covers(inclusive_range(-5, 0))); // touching low
        assert!(!r.covers(inclusive_range(100, 105))); // touching high

        assert!(!r.covers(inclusive_range(-1, 99))); // leaking low
        assert!(!r.covers(inclusive_range(1, 101))); // leaking high
    }

    #[test]
    fn overlaps() {
        let r = range(0, 100);
        assert!(r.overlaps(range(0, 100))); // same

        assert!(r.overlaps(range(0, 25))); // inner low
        assert!(r.overlaps(range(50, 75))); // inner middle
        assert!(r.overlaps(range(75, 100))); // inner high

        assert!(!r.overlaps(range(-1, -1))); // unit outside low
        assert!(r.overlaps(range(0, 0))); // unit low
        assert!(r.overlaps(range(50, 50))); // unit middle
        assert!(!r.overlaps(range(100, 100))); // unit high
        assert!(!r.overlaps(range(101, 101))); // unit outside high

        assert!(r.overlaps(range(-1, 101))); // outside both
        assert!(!r.overlaps(range(-5, -1))); // outside low
        assert!(!r.overlaps(range(101, 105))); // outside high

        assert!(!r.overlaps(range(-5, 0))); // touching low
        assert!(!r.overlaps(range(100, 105))); // touching high

        assert!(r.overlaps(range(-1, 99))); // leaking low
        assert!(r.overlaps(range(1, 101))); // leaking high

        let r = inclusive_range(0, 100);
        assert!(r.overlaps(inclusive_range(0, 100))); // same

        assert!(r.overlaps(inclusive_range(0, 25))); // inner low
        assert!(r.overlaps(inclusive_range(50, 75))); // inner middle
        assert!(r.overlaps(inclusive_range(75, 100))); // inner high

        assert!(!r.overlaps(inclusive_range(-1, -1))); // unit outside low
        assert!(r.overlaps(inclusive_range(0, 0))); // unit low
        assert!(r.overlaps(inclusive_range(50, 50))); // unit middle
        assert!(r.overlaps(inclusive_range(100, 100))); // unit high
        assert!(!r.overlaps(inclusive_range(101, 101))); // unit outside high

        assert!(r.overlaps(inclusive_range(-1, 101))); // outside both
        assert!(!r.overlaps(inclusive_range(-5, -1))); // outside low
        assert!(!r.overlaps(inclusive_range(101, 105))); // outside high

        assert!(r.overlaps(inclusive_range(-5, 0))); // touching low
        assert!(r.overlaps(inclusive_range(100, 105))); // touching high

        assert!(r.overlaps(inclusive_range(-1, 99))); // leaking low
        assert!(r.overlaps(inclusive_range(1, 101))); // leaking high
    }

    #[test]
    fn touches() {
        let r = range(0, 100);
        assert!(r.touches(range(0, 100))); // same

        assert!(r.touches(range(0, 25))); // inner low
        assert!(r.touches(range(50, 75))); // inner middle
        assert!(r.touches(range(75, 100))); // inner high

        assert!(!r.touches(range(-1, -1))); // unit outside low
        assert!(r.touches(range(0, 0))); // unit low
        assert!(r.touches(range(50, 50))); // unit middle
        assert!(r.touches(range(100, 100))); // unit high
        assert!(!r.touches(range(101, 101))); // unit outside high

        assert!(r.touches(range(-1, 101))); // outside both
        assert!(!r.touches(range(-5, -1))); // outside low
        assert!(!r.touches(range(101, 105))); // outside high

        assert!(r.touches(range(-5, 0))); // touching low
        assert!(r.touches(range(100, 105))); // touching high

        assert!(r.touches(range(-1, 99))); // leaking low
        assert!(r.touches(range(1, 101))); // leaking high

        let r = inclusive_range(0, 100);
        assert!(r.touches(inclusive_range(0, 100))); // same

        assert!(r.touches(inclusive_range(0, 25))); // inner low
        assert!(r.touches(inclusive_range(50, 75))); // inner middle
        assert!(r.touches(inclusive_range(75, 100))); // inner high

        assert!(!r.touches(inclusive_range(-1, -1))); // unit outside low
        assert!(r.touches(inclusive_range(0, 0))); // unit low
        assert!(r.touches(inclusive_range(50, 50))); // unit middle
        assert!(r.touches(inclusive_range(100, 100))); // unit high
        assert!(!r.touches(inclusive_range(101, 101))); // unit outside high

        assert!(r.touches(inclusive_range(-1, 101))); // outside both
        assert!(!r.touches(inclusive_range(-5, -1))); // outside low
        assert!(!r.touches(inclusive_range(101, 105))); // outside high

        assert!(r.touches(inclusive_range(-5, 0))); // touching low
        assert!(r.touches(inclusive_range(100, 105))); // touching high

        assert!(r.touches(inclusive_range(-1, 99))); // leaking low
        assert!(r.touches(inclusive_range(1, 101))); // leaking high
    }

    #[test]
    fn union() {
        let r = range(0, 100);
        assert_eq!(r.union(range(0, 100)), Some(r)); // same

        assert_eq!(r.union(range(0, 25)), Some(r)); // lower middle
        assert_eq!(r.union(range(50, 75)), Some(r)); // inner middle
        assert_eq!(r.union(range(75, 100)), Some(r)); // upper middle

        assert_eq!(r.union(range(-25, -25)), Some(r)); // unit outside low
        assert_eq!(r.union(range(0, 0)), Some(r)); // unit low
        assert_eq!(r.union(range(50, 50)), Some(r)); // unit middle
        assert_eq!(r.union(range(100, 100)), Some(r)); // unit high
        assert_eq!(r.union(range(125, 125)), Some(r)); // unit outside high

        assert_eq!(r.union(range(-25, 125)), Some(range(-25, 125))); // outside both
        assert_eq!(r.union(range(-50, -25)), None); // outside low
        assert_eq!(r.union(range(125, 150)), None); // outside high

        assert_eq!(r.union(range(100, 150)), Some(range(0, 150))); // touching high
        assert_eq!(r.union(range(-50, 0)), Some(range(-50, 100))); // touching low

        assert_eq!(r.union(range(50, 150)), Some(range(0, 150))); // leaking high
        assert_eq!(r.union(range(-50, 50)), Some(range(-50, 100))); // leaking low

        let r = inclusive_range(0, 100);
        assert_eq!(r.union(inclusive_range(0, 100)), Some(r)); // same

        assert_eq!(r.union(inclusive_range(0, 25)), Some(r)); // lower middle
        assert_eq!(r.union(inclusive_range(50, 75)), Some(r)); // inner middle
        assert_eq!(r.union(inclusive_range(75, 100)), Some(r)); // upper middle

        assert_eq!(r.union(inclusive_range(-25, -25)), None); // unit outside low
        assert_eq!(r.union(inclusive_range(0, 0)), Some(r)); // unit low
        assert_eq!(r.union(inclusive_range(50, 50)), Some(r)); // unit middle
        assert_eq!(r.union(inclusive_range(100, 100)), Some(r)); // unit high
        assert_eq!(r.union(inclusive_range(125, 125)), None); // unit outside high

        assert_eq!(
            r.union(inclusive_range(-25, 125)),
            Some(inclusive_range(-25, 125))
        ); // outside both
        assert_eq!(r.union(inclusive_range(-50, -25)), None); // outside low
        assert_eq!(r.union(inclusive_range(125, 150)), None); // outside high

        assert_eq!(
            r.union(inclusive_range(100, 150)),
            Some(inclusive_range(0, 150))
        ); // touching high
        assert_eq!(
            r.union(inclusive_range(-50, 0)),
            Some(inclusive_range(-50, 100))
        ); // touching low

        assert_eq!(
            r.union(inclusive_range(50, 150)),
            Some(inclusive_range(0, 150))
        ); // leaking high
        assert_eq!(
            r.union(inclusive_range(-50, 50)),
            Some(inclusive_range(-50, 100))
        ); // leaking low
    }

    #[test]
    fn remove() {
        let r = range(0, 100);
        assert_eq!(r.remove(range(0, 100)), vec![]); // same

        assert_eq!(r.remove(range(0, 25)), vec![range(25, 100)]); // lower middle
        assert_eq!(r.remove(range(50, 75)), vec![range(0, 50), range(75, 100)]); // inner middle
        assert_eq!(r.remove(range(75, 100)), vec![range(0, 75)]); // upper middle

        assert_eq!(r.remove(range(-25, -25)), vec![r]); // unit outside low
        assert_eq!(r.remove(range(0, 0)), vec![r]); // unit low
        assert_eq!(r.remove(range(50, 50)), vec![range(0, 50), range(50, 100)]); // unit middle
        assert_eq!(r.remove(range(100, 100)), vec![r]); // unit high
        assert_eq!(r.remove(range(125, 125)), vec![r]); // unit outside high

        assert_eq!(r.remove(range(-25, 125)), vec![]); // outside both
        assert_eq!(r.remove(range(-50, -25)), vec![r]); // outside low
        assert_eq!(r.remove(range(125, 150)), vec![r]); // outside high

        assert_eq!(r.remove(range(100, 150)), vec![r]); // touching high
        assert_eq!(r.remove(range(-50, 0)), vec![r]); // touching low

        assert_eq!(r.remove(range(50, 150)), vec![range(0, 50)]); // leaking high
        assert_eq!(r.remove(range(-50, 50)), vec![range(50, 100)]); // leaking low

        let r = inclusive_range(0, 100);
        assert_eq!(r.remove(inclusive_range(0, 100)), vec![]); // same

        assert_eq!(
            r.remove(inclusive_range(0, 25)),
            vec![inclusive_range(25, 100)]
        ); // lower middle
        assert_eq!(
            r.remove(inclusive_range(50, 75)),
            vec![inclusive_range(0, 50), inclusive_range(75, 100)]
        ); // inner middle
        assert_eq!(
            r.remove(inclusive_range(75, 100)),
            vec![inclusive_range(0, 75)]
        ); // upper middle

        assert_eq!(r.remove(inclusive_range(-25, -25)), vec![r]); // unit outside low
        assert_eq!(r.remove(inclusive_range(0, 0)), vec![r]); // unit low
        assert_eq!(
            r.remove(inclusive_range(50, 50)),
            vec![inclusive_range(0, 50), inclusive_range(50, 100)]
        ); // unit middle
        assert_eq!(r.remove(inclusive_range(100, 100)), vec![r]); // unit high
        assert_eq!(r.remove(inclusive_range(125, 125)), vec![r]); // unit outside high

        assert_eq!(r.remove(inclusive_range(-25, 125)), vec![]); // outside both
        assert_eq!(r.remove(inclusive_range(-50, -25)), vec![r]); // outside low
        assert_eq!(r.remove(inclusive_range(125, 150)), vec![r]); // outside high

        assert_eq!(r.remove(inclusive_range(100, 150)), vec![r]); // touching high
        assert_eq!(r.remove(inclusive_range(-50, 0)), vec![r]); // touching low

        assert_eq!(
            r.remove(inclusive_range(50, 150)),
            vec![inclusive_range(0, 50)]
        ); // leaking high
        assert_eq!(
            r.remove(inclusive_range(-50, 50)),
            vec![inclusive_range(50, 100)]
        ); // leaking low
    }

    #[test]
    fn ops() {
        let r = range(0, 100);
        assert_eq!(r + 5, range(5, 105));
        assert_eq!(r - 5, range(-5, 95));

        let r = inclusive_range(0, 100);
        assert_eq!(r + 5, inclusive_range(5, 105));
        assert_eq!(r - 5, inclusive_range(-5, 95));
    }
}
