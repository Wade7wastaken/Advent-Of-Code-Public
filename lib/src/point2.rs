use std::{fmt::Display, ops::Neg};

use derive_more::derive::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use num::{CheckedAdd, CheckedSub, Float, Num};

use crate::{abs_diff, tern, Dir};

/// Represents a point in 2d space
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
)]
pub struct Point2<T: Num + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Point2<T> {
    /// Creates a point from an x and y coordinate
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Creates a point from a tuple
    pub fn from_tuple((x, y): (T, T)) -> Self {
        Self { x, y }
    }

    /// Returns the x and y coordinates as a tuple
    pub fn tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    /// Swaps the x and y coordinates
    #[must_use]
    pub fn swap(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    /// Swaps the x and y coordinates if cond is true
    #[must_use]
    pub fn swap_if(self, cond: bool) -> Self {
        tern!(cond, self.swap(), self)
    }

    /// Applies a dir to the point
    pub fn apply(mut self, dir: Dir) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + TryFrom<isize>,
    {
        type F<T> = fn(&T, &T) -> Option<T>;

        let x_op: F<T> = tern!(dir.x < 0, T::checked_sub, T::checked_add);
        let new = T::one() * dir.x.abs().try_into().ok()?;
        self.x = x_op(&self.x, &new)?;

        let y_op: F<T> = tern!(dir.y < 0, T::checked_sub, T::checked_add);
        let new = T::one() * dir.y.abs().try_into().ok()?;
        self.y = y_op(&self.y, &new)?;

        Some(self)
    }

    /// Applies a dir scaled by a factor to the point
    pub fn apply_n(mut self, dir: Dir, n: T) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + TryFrom<isize>,
    {
        type F<T> = fn(&T, &T) -> Option<T>;

        let x_op: F<T> = tern!(dir.x < 0, T::checked_sub, T::checked_add);
        let new = T::one() * dir.x.abs().try_into().ok()? * n;
        self.x = x_op(&self.x, &new)?;

        let y_op: F<T> = tern!(dir.y < 0, T::checked_sub, T::checked_add);
        let new = T::one() * dir.y.abs().try_into().ok()? * n;
        self.y = y_op(&self.y, &new)?;

        Some(self)
    }

    // Checks if a point is in [a, b) for both coordinates
    pub fn within(&self, a: impl Into<Point2<T>>, b: impl Into<Point2<T>>) -> bool
    where
        T: PartialOrd,
    {
        let a: Point2<T> = a.into();
        let b: Point2<T> = b.into();
        self.x >= a.x && self.y >= a.y && self.x < b.x && self.y < b.y
    }

    /// Applies a mapping function to each coordinate independently
    pub fn map<F: Num + Copy>(self, f: impl Fn(T) -> F) -> Point2<F> {
        Point2 {
            x: f(self.x),
            y: f(self.y),
        }
    }

    /// Calculates the distance between the point and another point
    pub fn dist(&self, other: impl Into<Point2<T>>) -> T
    where
        T: Float + PartialOrd,
    {
        let other: Point2<T> = other.into();
        let dx = abs_diff(other.x, self.x);
        let dy = abs_diff(other.y, self.y);
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculates the distance squared between the point and another point
    pub fn dist_squared(&self, other: impl Into<Point2<T>>) -> T
    where
        T: PartialOrd,
    {
        let other: Point2<T> = other.into();
        let dx = abs_diff(other.x, self.x);
        let dy = abs_diff(other.y, self.y);
        dx * dx + dy * dy
    }

    /// Calculates the manhattan distance between the point and another point
    pub fn manhattan_dist(&self, other: impl Into<Point2<T>>) -> T
    where
        T: PartialOrd,
    {
        let other: Point2<T> = other.into();
        let dx = abs_diff(self.x, other.x);
        let dy = abs_diff(self.y, other.y);
        dx + dy
    }
}

impl<T: Num + Copy + Neg<Output = T>> Point2<T> {
    /// Reflects the point over the x axis
    #[must_use]
    pub fn reflect_x(self) -> Self {
        Self { y: -self.y, ..self }
    }

    /// Reflects the point over the y axis
    #[must_use]
    pub fn reflect_y(self) -> Self {
        Self { x: -self.x, ..self }
    }
}

impl<T: Num + Copy> From<(T, T)> for Point2<T> {
    fn from(value: (T, T)) -> Self {
        Self::from_tuple(value)
    }
}

impl<T: Num + Copy> From<Point2<T>> for (T, T) {
    fn from(value: Point2<T>) -> Self {
        value.tuple()
    }
}

impl<T: Num + Copy + Display> Display for Point2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algebra() {
        let mut p = Point2::new(5, 7);
        assert_eq!(p + (4, 5).into(), (9, 12).into());
        assert_eq!(p - (4, 5).into(), (1, 2).into());
        assert_eq!(p * 3, (15, 21).into());
        assert_eq!(p / 2, (2, 3).into());
        assert_eq!(-p, (-5, -7).into());
        assert_eq!(p % 4, (1, 3).into());

        p += (4, 5).into();
        assert_eq!(p, (9, 12).into());
        p -= (4, 5).into();
        assert_eq!(p, (5, 7).into());
        p *= 3;
        assert_eq!(p, (15, 21).into());
        p /= 3;
        assert_eq!(p, (5, 7).into());
        p %= 4;
        assert_eq!(p, (1, 3).into());
    }

    #[test]
    fn tuples() {
        let point = Point2::from_tuple((1, 2));
        assert_eq!(point.tuple(), (1, 2));
        assert_eq!(point, Point2::from((1, 2)));
        assert_eq!(point, (1, 2).into());
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn swap() {
        let point = Point2::new(1, 2);
        assert_eq!(point.swap(), (2, 1).into());
        assert_eq!(point.swap_if(true), (2, 1).into());
        assert_eq!(point.swap_if(false), (1, 2).into());
    }

    #[test]
    fn apply() {
        let point = Point2::new(1, 2);
        assert_eq!(point.apply(Dir::NORTH).unwrap(), (1, 1).into());
        assert_eq!(point.apply(Dir::SOUTH).unwrap(), (1, 3).into());
        assert_eq!(point.apply(Dir::EAST).unwrap(), (2, 2).into());
        assert_eq!(point.apply(Dir::WEST).unwrap(), (0, 2).into());

        assert_eq!(point.apply_n(Dir::NORTH, 3).unwrap(), (1, -1).into());
        assert_eq!(point.apply_n(Dir::SOUTH, 3).unwrap(), (1, 5).into());
        assert_eq!(point.apply_n(Dir::EAST, 3).unwrap(), (4, 2).into());
        assert_eq!(point.apply_n(Dir::WEST, 3).unwrap(), (-2, 2).into());

        let point = Point2::new(0u32, 0u32);
        assert_eq!(point.apply(Dir::NORTH), None);
        assert_eq!(point.apply(Dir::WEST), None);
        let point = Point2::new(5u32, 5u32);
        assert_eq!(point.apply_n(Dir::NORTH, 6), None);
        assert_eq!(point.apply_n(Dir::WEST, 6), None);
    }

    #[test]
    fn within() {
        let a = (1, 1);
        let b = (5, 5);
        assert!(Point2::new(2, 2).within(a, b));
        assert!(!Point2::new(-2, 2).within(a, b));
    }

    #[test]
    fn map() {
        assert_eq!(Point2::new(2, 2).map(f64::from), (2.0, 2.0).into());
        assert_eq!(Point2::new(2, 2).map(|x| x + 1), (3, 3).into());
    }

    #[test]
    fn dist() {
        assert_eq!(Point2::new(1.0, 2.0).dist((3.0, 4.0)), 8.0.sqrt());
        assert_eq!(Point2::new(1, 2).dist_squared((3, 4)), 8);
        assert_eq!(Point2::new(1, 2).manhattan_dist((3, 4)), 4);
    }

    #[test]
    fn reflect() {
        let point = Point2::new(1, 2);
        assert_eq!(point.reflect_x(), (1, -2).into());
        assert_eq!(point.reflect_y(), (-1, 2).into());
    }
}
