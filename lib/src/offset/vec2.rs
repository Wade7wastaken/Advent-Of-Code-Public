use std::fmt::{self, Display};

use derive_more::derive::{Add, AddAssign, Mul, MulAssign};
use num::Num;

use crate::{Dir, Offset, Point2};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, Add, AddAssign, Mul, MulAssign)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub const EAST: Vec2 = Vec2::new(1, 0);
    pub const WEST: Vec2 = Vec2::new(-1, 0);
    pub const NORTH: Vec2 = Vec2::new(0, -1);
    pub const SOUTH: Vec2 = Vec2::new(0, 1);

    // groups of directions
    pub const ORTHO: [Vec2; 4] = [Vec2::EAST, Vec2::WEST, Vec2::NORTH, Vec2::SOUTH];
    pub const ORTHO_SNAKE: [Vec2; 4] = [Vec2::NORTH, Vec2::WEST, Vec2::EAST, Vec2::SOUTH];
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
    pub const CORNERS: [Vec2; 4] = [
        Vec2::new(-1, -1),
        Vec2::new(1, -1),
        Vec2::new(-1, 1),
        Vec2::new(1, 1),
    ];

    /// Creates a vec2 with x and y components. Const for direction groups
    #[must_use]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn try_into_dir(self) -> Option<Dir> {
        match self {
            Vec2::EAST => Some(Dir::East),
            Vec2::WEST => Some(Dir::West),
            Vec2::NORTH => Some(Dir::North),
            Vec2::SOUTH => Some(Dir::South),
            _ => None,
        }
    }

    /// Calculates the dot product between two dirs
    #[must_use]
    pub const fn dot(self, other: Self) -> isize {
        self.x * other.x + self.y * other.y
    }

    pub fn between<T: Num + Copy + TryInto<isize>>(a: Point2<T>, b: Point2<T>) -> Option<Self> {
        let x = b.x.try_into().ok()? - a.x.try_into().ok()?;
        let y = b.y.try_into().ok()? - a.y.try_into().ok()?;
        Some(Self { x, y })
    }
}

impl Offset for Vec2 {
    /// Reverses a vec2
    fn reverse(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    /// Turns the vec2 left (ccw) by 90 degrees
    fn turn_left(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    /// Turns the vec2 right (cw) by 90 degrees
    fn turn_right(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Determines if two dirs are perpendicular/orthogonal
    fn is_ortho(self, other: Self) -> bool {
        self.dot(other) == 0
    }

    /// Determines if other is the reverse of self
    fn is_reverse(self, other: Self) -> bool {
        self == other.reverse()
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
        assert!(Vec2::new(4, -5).is_reverse(Vec2::new(-4, 5)));
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
