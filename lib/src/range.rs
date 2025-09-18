use std::ops::{Add, Range as RustRange, RangeInclusive as RustInclusiveRange, Sub};

// Represents [a, b)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Range<T: Copy> {
    start: T,
    end: T,
}

impl<T: Copy> Range<T> {
    /// # Safety
    /// Caller is responsible for ensuring start <= end.
    pub const unsafe fn new_unchecked(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub const fn start(self) -> T {
        self.start
    }

    pub const fn end(&self) -> T {
        self.end
    }

    pub fn is_empty(self) -> bool
    where
        T: PartialEq,
    {
        self.start == self.end
    }
}

impl<T: Copy + PartialOrd> Range<T> {
    pub fn try_new(start: T, end: T) -> Option<Self> {
        (start <= end).then_some(Self { start, end })
    }

    pub fn new(start: T, end: T) -> Self {
        Self::try_new(start, end).expect("start can't be larger than end")
    }

    pub fn contains(self, x: T) -> bool {
        self.start <= x && self.end > x
    }

    pub fn covers(&self, r: Range<T>) -> bool {
        self.start <= r.start && self.end >= r.end
    }

    pub fn overlaps(&self, r: Range<T>) -> bool {
        if r.is_empty() {
            return self.contains(r.start);
        }
        self.start < r.end && r.start < self.end
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
        self.start.partial_cmp(&other.end)
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

// Represents [a, b]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InclusiveRange<T: Copy> {
    start: T,
    end: T,
}

impl<T: Copy> InclusiveRange<T> {
    /// # Safety
    /// Caller is responsible for ensuring start <= end.
    pub const unsafe fn new_unchecked(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub const fn start(self) -> T {
        self.start
    }

    pub const fn end(self) -> T {
        self.end
    }

    pub fn is_empty(self) -> bool
    where
        T: PartialEq,
    {
        self.start == self.end
    }
}

impl<T: Copy + PartialOrd> InclusiveRange<T> {
    pub fn try_new(start: T, end: T) -> Option<Self> {
        (start <= end).then_some(Self { start, end })
    }

    pub fn new(start: T, end: T) -> Self {
        Self::try_new(start, end).expect("start can't be larger than end")
    }

    pub fn contains(&self, x: T) -> bool {
        self.start <= x && self.end >= x
    }

    pub fn covers(&self, r: InclusiveRange<T>) -> bool {
        self.start <= r.start && self.end >= r.end
    }

    pub fn overlaps(&self, r: InclusiveRange<T>) -> bool {
        if r.is_empty() {
            return self.contains(r.start);
        }
        self.start <= r.end && r.start <= self.end
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
        self.start.partial_cmp(&other.end)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rust_range() {
        let r: Range<i32> = (0..100).into();
        assert_eq!(r, Range::new(0, 100));

        let r: InclusiveRange<i32> = (0..=100).into();
        assert_eq!(r, InclusiveRange::new(0, 100));
    }

    #[test]
    fn contains() {
        let range = Range::new(0, 100);
        assert!(range.contains(0));
        assert!(range.contains(50));
        assert!(range.contains(99));
        assert!(!range.contains(100));
        assert!(!range.contains(-1));
        assert!(!range.contains(101));

        let range = InclusiveRange::new(0, 100);
        assert!(range.contains(0));
        assert!(range.contains(50));
        assert!(range.contains(99));
        assert!(range.contains(100));
        assert!(!range.contains(-1));
        assert!(!range.contains(101));
    }

    #[test]
    fn contains_range() {
        let range = Range::new(0, 100);
        assert!(range.covers(Range::new(0, 100)));
        assert!(range.covers(Range::new(0, 25)));
        assert!(range.covers(Range::new(50, 75)));
        assert!(range.covers(Range::new(100, 100)));
        assert!(!range.covers(Range::new(101, 101)));
        assert!(!range.covers(Range::new(-1, 101)));
        assert!(!range.covers(Range::new(-5, -1)));
        assert!(!range.covers(Range::new(101, 105)));
        assert!(!range.covers(Range::new(-5, 95)));
        assert!(!range.covers(Range::new(5, 105)));

        let range = InclusiveRange::new(0, 100);
        assert!(range.covers(InclusiveRange::new(0, 100)));
        assert!(range.covers(InclusiveRange::new(0, 25)));
        assert!(range.covers(InclusiveRange::new(50, 75)));
        assert!(range.covers(InclusiveRange::new(100, 100)));
        assert!(!range.covers(InclusiveRange::new(101, 101)));
        assert!(!range.covers(InclusiveRange::new(-1, 101)));
        assert!(!range.covers(InclusiveRange::new(-5, -1)));
        assert!(!range.covers(InclusiveRange::new(101, 105)));
        assert!(!range.covers(InclusiveRange::new(-5, 95)));
        assert!(!range.covers(InclusiveRange::new(5, 105)));
    }

    #[test]
    fn overlaps() {
        let range = Range::new(0, 100);
        assert!(range.overlaps(Range::new(0, 100)));

        assert!(range.overlaps(Range::new(50, 75)));

        assert!(range.overlaps(Range::new(0, 0)));
        assert!(range.overlaps(Range::new(50, 50)));
        assert!(!range.overlaps(Range::new(100, 100)));

        assert!(!range.overlaps(Range::new(101, 105)));
        assert!(!range.overlaps(Range::new(-5, -1)));

        assert!(!range.overlaps(Range::new(-5, 0)));
        assert!(!range.overlaps(Range::new(100, 105)));

        assert!(range.overlaps(Range::new(-1, 101)));
        assert!(range.overlaps(Range::new(-5, 95)));
        assert!(range.overlaps(Range::new(5, 105)));

        let range = InclusiveRange::new(0, 100);
        assert!(range.overlaps(InclusiveRange::new(0, 100)));

        assert!(range.overlaps(InclusiveRange::new(50, 75)));

        assert!(range.overlaps(InclusiveRange::new(0, 0)));
        assert!(range.overlaps(InclusiveRange::new(50, 50)));
        assert!(range.overlaps(InclusiveRange::new(100, 100)));

        assert!(!range.overlaps(InclusiveRange::new(101, 105)));
        assert!(!range.overlaps(InclusiveRange::new(-5, -1)));

        assert!(range.overlaps(InclusiveRange::new(-5, 0)));
        assert!(range.overlaps(InclusiveRange::new(100, 105)));

        assert!(range.overlaps(InclusiveRange::new(-1, 101)));
        assert!(range.overlaps(InclusiveRange::new(-5, 95)));
        assert!(range.overlaps(InclusiveRange::new(5, 105)));
    }

    #[test]
    fn ops() {
        let range = Range::new(0, 100);
        assert_eq!(range + 5, Range::new(5, 105));
        assert_eq!(range - 5, Range::new(-5, 95));
    }
}
