use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use derive_more::derive::{Add, AddAssign, Mul, MulAssign};
use num::Num;

use crate::Point2;

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    AddAssign,
    Mul,
    MulAssign,
)]
pub struct Dir {
    pub x: isize,
    pub y: isize,
}

impl Dir {
    // primitive directions
    pub const EAST: Dir = Dir::new(1, 0);
    pub const WEST: Dir = Dir::new(-1, 0);
    pub const NORTH: Dir = Dir::new(0, -1);
    pub const SOUTH: Dir = Dir::new(0, 1);

    // groups of directions
    pub const ORTHO: [Dir; 4] = [Dir::EAST, Dir::WEST, Dir::NORTH, Dir::SOUTH];
    pub const ORTHO_SNAKE: [Dir; 4] = [Dir::NORTH, Dir::WEST, Dir::EAST, Dir::SOUTH];
    pub const SURROUNDING: [Dir; 8] = [
        Dir::new(-1, -1),
        Dir::new(0, -1),
        Dir::new(1, -1),
        Dir::new(-1, 0),
        Dir::new(1, 0),
        Dir::new(-1, 1),
        Dir::new(0, 1),
        Dir::new(1, 1),
    ];
    pub const CORNERS: [Dir; 4] = [
        Dir::new(-1, -1),
        Dir::new(1, -1),
        Dir::new(-1, 1),
        Dir::new(1, 1),
    ];

    /// Creates a dir with x and y components. Const for direction groups
    #[must_use]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Returns an arbitrary index for each of the primitive dirs, so that
    /// primitive dirs can be used as the keys for an array. Don't depend on the
    /// specific value of the index.
    #[must_use]
    pub fn idx(self) -> usize {
        match self {
            Dir::EAST => 0,
            Dir::WEST => 1,
            Dir::NORTH => 2,
            Dir::SOUTH => 3,
            _ => panic!("Dir::idx only supports primitive dirs"),
        }
    }

    #[must_use]
    pub fn is_primitive(self) -> bool {
        matches!(self, Dir::EAST | Dir::WEST | Dir::NORTH | Dir::SOUTH)
    }

    /// Reverses a dir
    #[must_use]
    pub fn reverse(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    /// Turns the dir left (ccw) by 90 degrees
    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    /// Turns the dir right (cw) by 90 degrees
    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Calculates the dot product between two dirs
    #[must_use]
    pub fn dot(self, other: Dir) -> isize {
        self.x * other.x + self.y * other.y
    }

    /// Determines if two dirs are perpendicular/orthogonal
    #[must_use]
    pub fn is_ortho(self, other: Dir) -> bool {
        self.dot(other) == 0
    }

    /// Determines if other is the reverse of self
    #[must_use]
    pub fn is_reverse(self, other: Dir) -> bool {
        self == other.reverse()
    }

    pub fn between<T: Num + Copy + TryInto<isize>>(a: Point2<T>, b: Point2<T>) -> Option<Dir> {
        let x = b.x.try_into().ok()? - a.x.try_into().ok()?;
        let y = b.y.try_into().ok()? - a.y.try_into().ok()?;
        Some(Self { x, y })
    }
}

impl TryFrom<char> for Dir {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' | 'N' | 'U' | 'n' | 'u' => Ok(Dir::NORTH),
            '<' | 'W' | 'L' | 'w' | 'l' => Ok(Dir::WEST),
            '>' | 'E' | 'R' | 'e' | 'r' => Ok(Dir::EAST),
            'v' | 'S' | 'D' | 's' | 'd' => Ok(Dir::SOUTH),
            _ => Err(()),
        }
    }
}

impl FromStr for Dir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" | "N" | "U" | "North" | "Up" | "n" | "u" | "north" | "up" => Ok(Dir::NORTH),
            "<" | "W" | "L" | "West" | "Left" | "w" | "l" | "west" | "left" => Ok(Dir::WEST),
            ">" | "E" | "R" | "East" | "Right" | "e" | "r" | "east" | "right" => Ok(Dir::EAST),
            "v" | "S" | "D" | "South" | "Down" | "s" | "d" | "south" | "down" => Ok(Dir::SOUTH),
            _ => Err(()),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_primitive() {
            write!(
                f,
                "{}",
                match *self {
                    Dir::EAST => "East",
                    Dir::WEST => "West",
                    Dir::NORTH => "North",
                    Dir::SOUTH => "South",
                    _ => unreachable!(),
                }
            )
        } else {
            write!(f, "⟨{},{}⟩", self.x, self.y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::itertools::Itertools;

    #[test]
    fn arithmetic() {
        assert_eq!(Dir::new(2, 5) + Dir::new(8, -3), Dir::new(10, 2));
        assert_eq!(Dir::new(1, -2) * 3, Dir::new(3, -6));
    }

    #[test]
    fn primitives() {
        assert!(Dir::ORTHO.into_iter().all(Dir::is_primitive));
        assert!(Dir::ORTHO.into_iter().map(Dir::idx).all_unique());
    }

    #[test]
    fn reverse() {
        assert_eq!(Dir::EAST.reverse(), Dir::WEST);
        assert_eq!(Dir::NORTH.reverse(), Dir::SOUTH);
        assert_eq!(Dir::new(4, -5).reverse(), Dir::new(-4, 5));
        assert!(Dir::new(4, -5).is_reverse(Dir::new(-4, 5)));
    }

    #[test]
    fn turn_left() {
        assert_eq!(Dir::EAST.turn_left(), Dir::NORTH);
        assert_eq!(Dir::NORTH.turn_left(), Dir::WEST);
        assert_eq!(Dir::new(4, -5).turn_left(), Dir::new(-5, -4));
    }

    #[test]
    fn turn_right() {
        assert_eq!(Dir::EAST.turn_right(), Dir::SOUTH);
        assert_eq!(Dir::NORTH.turn_right(), Dir::EAST);
        assert_eq!(Dir::new(4, -5).turn_right(), Dir::new(5, 4));
    }

    #[test]
    fn dot() {
        assert_eq!(Dir::EAST.dot(Dir::NORTH), 0);
        assert_eq!(Dir::EAST.dot(Dir::WEST), -1);
        assert_eq!(Dir::EAST.dot(Dir::EAST), 1);
        assert_eq!(Dir::new(4, -5).dot(Dir::new(1, 2)), -6);
    }

    #[test]
    fn is_ortho() {
        assert!(Dir::NORTH.is_ortho(Dir::EAST));
        assert!(Dir::WEST.is_ortho(Dir::SOUTH));
        assert!(Dir::new(1, 5).is_ortho(Dir::new(-5, 1)));
    }

    #[test]
    fn between() {
        assert_eq!(
            Dir::between(Point2::new(1, 1), Point2::new(4, 6)),
            Some(Dir::new(3, 5))
        );
    }

    #[test]
    fn from_arrow() {
        assert_eq!(Dir::try_from('^'), Ok(Dir::NORTH));
        assert_eq!(Dir::try_from('W'), Ok(Dir::WEST));
        assert_eq!(Dir::try_from('L'), Ok(Dir::WEST));
        assert_eq!(Dir::try_from('e'), Ok(Dir::EAST));
        assert_eq!(Dir::try_from('d'), Ok(Dir::SOUTH));
        assert_eq!(Dir::try_from('A'), Err(()));

        assert_eq!("^".parse(), Ok(Dir::NORTH));
        assert_eq!("N".parse(), Ok(Dir::NORTH));
        assert_eq!("L".parse(), Ok(Dir::WEST));
        assert_eq!("West".parse(), Ok(Dir::WEST));
        assert_eq!("Right".parse(), Ok(Dir::EAST));
        assert_eq!("e".parse(), Ok(Dir::EAST));
        assert_eq!("d".parse(), Ok(Dir::SOUTH));
        assert_eq!("south".parse(), Ok(Dir::SOUTH));
        assert_eq!("down".parse(), Ok(Dir::SOUTH));
        assert_eq!("A".parse::<Dir>(), Err(()));
    }
}
