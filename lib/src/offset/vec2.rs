use std::fmt::{self, Debug, Display};

use derive_more::derive::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num::Num;

use crate::{Dir, Offset, Point2};

/// An arbitrary 2d vector that has an x and y component. The positive x axis
/// faces east and the positive y axis faces south.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    PartialEq,
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    /// The unit vector facing East, or positive x.
    pub const EAST: Vec2 = Vec2::new(1, 0);

    /// The unit vector facing west, or negative x.
    pub const WEST: Vec2 = Vec2::new(-1, 0);

    /// The unit vector facing north, or negative y.
    pub const NORTH: Vec2 = Vec2::new(0, -1);

    /// The unit vector facing south, or positive y.
    pub const SOUTH: Vec2 = Vec2::new(0, 1);

    /// Each of the cardinal directions in the order they are defined.
    pub const ORTHO: [Vec2; 4] = [Vec2::EAST, Vec2::WEST, Vec2::NORTH, Vec2::SOUTH];

    /// Each of the cardinal directions in the order they are encountered in a
    /// linear search.
    pub const ORTHO_SNAKE: [Vec2; 4] = [Vec2::NORTH, Vec2::WEST, Vec2::EAST, Vec2::SOUTH];

    /// All adjacent unit directions, including corners.
    pub const SURROUNDING: [Vec2; 8] = [
        Vec2::new(-1, -1),
        Vec2::new(0, -1),
        Vec2::new(1, -1),
        Vec2::new(-1, 0),
        Vec2::new(1, 0),
        Vec2::new(-1, 1),
        Vec2::new(0, 1),
        Vec2::new(1, 1),
    ];

    /// All adjacent corners.
    pub const CORNERS: [Vec2; 4] = [
        Vec2::new(-1, -1),
        Vec2::new(1, -1),
        Vec2::new(-1, 1),
        Vec2::new(1, 1),
    ];

    /// Creates a Vec2 with the given x and y components.
    #[must_use]
    #[inline]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Calculates the dot product between two Vec2s.
    #[must_use]
    #[inline]
    pub const fn dot(self, other: Self) -> isize {
        self.x * other.x + self.y * other.y
    }

    /// Calculates b - a, or the vector staring at a and pointing to b, as a
    /// Vec2.
    #[must_use]
    pub fn between<T: Num + Copy + TryInto<isize>>(a: Point2<T>, b: Point2<T>) -> Option<Self> {
        let x = b.x.try_into().ok()? - a.x.try_into().ok()?;
        let y = b.y.try_into().ok()? - a.y.try_into().ok()?;
        Some(Self { x, y })
    }
}

impl Offset for Vec2 {
    #[inline]
    fn reverse(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    #[inline]
    fn turn_left(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    #[inline]
    fn turn_right(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    #[inline]
    fn is_ortho(self, other: Self) -> bool {
        self.dot(other) == 0
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<Dir> for Vec2 {
    fn from(val: Dir) -> Self {
        match val {
            Dir::East => Vec2::new(1, 0),
            Dir::West => Vec2::new(-1, 0),
            Dir::North => Vec2::new(0, -1),
            Dir::South => Vec2::new(0, 1),
        }
    }
}

impl<T: Num + Copy + TryInto<isize>> TryFrom<Point2<T>> for Vec2 {
    type Error = <T as TryInto<isize>>::Error;

    fn try_from(value: Point2<T>) -> Result<Self, Self::Error> {
        let x: isize = value.x.try_into()?;
        let y: isize = value.y.try_into()?;

        Ok(Self { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        assert_eq!(Vec2::new(2, 5) + Vec2::new(8, -3), Vec2::new(10, 2));
        assert_eq!(Vec2::new(1, -2) * 3, Vec2::new(3, -6));
    }

    #[test]
    fn reverse() {
        assert_eq!(Vec2::new(4, -5).reverse(), Vec2::new(-4, 5));
        assert!(Vec2::new(4, -5).is_reverse_of(Vec2::new(-4, 5)));
    }

    #[test]
    fn turn_left() {
        assert_eq!(Vec2::new(4, -5).turn_left(), Vec2::new(-5, -4));
    }

    #[test]
    fn turn_right() {
        assert_eq!(Vec2::new(4, -5).turn_right(), Vec2::new(5, 4));
    }

    #[test]
    fn dot() {
        assert_eq!(Vec2::new(4, -5).dot(Vec2::new(1, 2)), -6);
    }

    #[test]
    fn is_ortho() {
        assert!(Vec2::new(1, 5).is_ortho(Vec2::new(-5, 1)));
    }

    #[test]
    fn between() {
        assert_eq!(
            Vec2::between(Point2::new(1, 1), Point2::new(4, 6)),
            Some(Vec2::new(3, 5))
        );
    }
}
