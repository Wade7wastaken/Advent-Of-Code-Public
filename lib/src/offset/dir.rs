use std::{error::Error, fmt::Debug, hash::Hash, str::FromStr};

use derive_more::derive::Display;

use crate::{Offset, Vec2};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Display)]
pub enum Dir {
    #[display("East")]
    East,
    #[display("West")]
    West,
    #[display("North")]
    North,
    #[display("South")]
    South,
}

impl Dir {
    // groups of directions
    pub const ORTHO: [Dir; 4] = [Dir::East, Dir::West, Dir::North, Dir::South];
    pub const ORTHO_SNAKE: [Dir; 4] = [Dir::North, Dir::West, Dir::East, Dir::South];

    /// Returns an arbitrary index for each of the dirs, so that dirs can be
    /// used as the keys for an array. Don't depend on the specific value of the
    /// index.
    #[must_use]
    pub const fn idx(self) -> usize {
        match self {
            Dir::East => 0,
            Dir::West => 1,
            Dir::North => 2,
            Dir::South => 3,
        }
    }
}

impl Offset for Dir {
    /// Reverses a Dir.
    fn reverse(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::West => Self::East,
            Self::North => Self::South,
            Self::South => Self::North,
        }
    }

    /// Turns the Dir left (ccw) by 90 degrees.
    fn turn_left(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }

    /// Turns the Dir right (cw) by 90 degrees.
    fn turn_right(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
        }
    }

    /// Determines if two Dirs are perpendicular/orthogonal.
    fn is_ortho(self, other: Self) -> bool {
        !self.is_reverse(other) && self != other
    }

    /// Determines if other is the reverse of self.
    fn is_reverse(self, other: Self) -> bool {
        self == other.reverse()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display("Error converting to Dir: \"{}\" is not an orthogonal unit vector", _0)]
pub struct DirConvertError(Vec2);
impl Error for DirConvertError {}

impl TryFrom<Vec2> for Dir {
    type Error = DirConvertError;
    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        match value {
            Vec2 { x: 1, y: 0 } => Ok(Dir::East),
            Vec2 { x: -1, y: 0 } => Ok(Dir::West),
            Vec2 { x: 0, y: -1 } => Ok(Dir::North),
            Vec2 { x: 0, y: 1 } => Ok(Dir::South),
            _ => Err(DirConvertError(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display("Error parsing Dir: \"{}\" is not a dir", _0)]
pub struct DirParseError(String);
impl Error for DirParseError {}

impl TryFrom<u8> for Dir {
    type Error = DirParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            b'^' | b'n' | b'u' => Ok(Dir::North),
            b'<' | b'w' | b'l' => Ok(Dir::West),
            b'>' | b'e' | b'r' => Ok(Dir::East),
            b'v' | b's' | b'd' => Ok(Dir::South),
            _ => Err(DirParseError(char::from(value).to_string())),
        }
    }
}

impl TryFrom<char> for Dir {
    type Error = DirParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            '^' | 'n' | 'u' => Ok(Dir::North),
            '<' | 'w' | 'l' => Ok(Dir::West),
            '>' | 'e' | 'r' => Ok(Dir::East),
            'v' | 's' | 'd' => Ok(Dir::South),
            _ => Err(DirParseError(value.to_string())),
        }
    }
}

impl FromStr for Dir {
    type Err = DirParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "^" | "n" | "u" | "north" | "up" => Ok(Dir::North),
            "<" | "w" | "l" | "west" | "left" => Ok(Dir::West),
            ">" | "e" | "r" | "east" | "right" => Ok(Dir::East),
            "v" | "s" | "d" | "south" | "down" => Ok(Dir::South),
            _ => Err(DirParseError(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::itertools::Itertools;

    #[test]
    fn display() {
        assert_eq!(Dir::East.to_string(), "East");
        assert_eq!(Dir::West.to_string(), "West");
        assert_eq!(Dir::North.to_string(), "North");
        assert_eq!(Dir::South.to_string(), "South");
    }

    #[test]
    fn index() {
        assert!(Dir::ORTHO.into_iter().map(Dir::idx).all_unique());
    }

    #[test]
    fn reverse() {
        assert_eq!(Dir::East.reverse(), Dir::West);
        assert_eq!(Dir::North.reverse(), Dir::South);
    }

    #[test]
    fn turn_left() {
        assert_eq!(Dir::East.turn_left(), Dir::North);
        assert_eq!(Dir::North.turn_left(), Dir::West);
    }

    #[test]
    fn turn_right() {
        assert_eq!(Dir::East.turn_right(), Dir::South);
        assert_eq!(Dir::North.turn_right(), Dir::East);
    }

    #[test]
    fn is_ortho() {
        assert!(Dir::North.is_ortho(Dir::East));
        assert!(Dir::West.is_ortho(Dir::South));
    }

    #[test]
    fn from_arrow() {
        assert_eq!(Dir::try_from(b'^'), Ok(Dir::North));
        assert_eq!(Dir::try_from(b'W'), Ok(Dir::West));
        assert_eq!(Dir::try_from(b'L'), Ok(Dir::West));
        assert_eq!(Dir::try_from(b'e'), Ok(Dir::East));
        assert_eq!(Dir::try_from(b'd'), Ok(Dir::South));
        assert_eq!(Dir::try_from(b'A'), Err(DirParseError("A".to_string())));

        assert_eq!(Dir::try_from('^'), Ok(Dir::North));
        assert_eq!(Dir::try_from('W'), Ok(Dir::West));
        assert_eq!(Dir::try_from('L'), Ok(Dir::West));
        assert_eq!(Dir::try_from('e'), Ok(Dir::East));
        assert_eq!(Dir::try_from('d'), Ok(Dir::South));
        assert_eq!(Dir::try_from('A'), Err(DirParseError("A".to_string())));

        assert_eq!("^".parse(), Ok(Dir::North));
        assert_eq!("N".parse(), Ok(Dir::North));
        assert_eq!("L".parse(), Ok(Dir::West));
        assert_eq!("West".parse(), Ok(Dir::West));
        assert_eq!("Right".parse(), Ok(Dir::East));
        assert_eq!("e".parse(), Ok(Dir::East));
        assert_eq!("d".parse(), Ok(Dir::South));
        assert_eq!("south".parse(), Ok(Dir::South));
        assert_eq!("down".parse(), Ok(Dir::South));
        assert_eq!("A".parse::<Dir>(), Err(DirParseError("A".to_string())));
    }
}
