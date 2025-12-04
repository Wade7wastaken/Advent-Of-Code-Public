mod dir;
mod vec2;

use std::{fmt::Display, hash::Hash};

pub use dir::*;
pub use vec2::*;

/// Represents any object that gives an offset from a position without depending
/// on the position.
pub trait Offset: Display + Copy + PartialEq + Eq + Hash + Into<Vec2> {
    /// Reverses an Offset.
    #[must_use]
    fn reverse(self) -> Self;

    /// Determines if other is the reverse of self.
    #[must_use]
    #[inline]
    fn is_reverse_of(self, other: Self) -> bool {
        self == other.reverse()
    }

    /// Turns an Offset left (ccw) by 90 degrees.
    #[must_use]
    fn turn_left(self) -> Self;

    /// Turns an Offset right (cw) by 90 degrees.
    #[must_use]
    fn turn_right(self) -> Self;

    /// Determines if two Offsets are perpendicular/orthogonal.
    #[must_use]
    fn is_ortho(self, other: Self) -> bool;
}
