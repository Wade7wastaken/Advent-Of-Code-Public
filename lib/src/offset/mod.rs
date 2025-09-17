mod dir;
mod vec2;

use std::{fmt::Display, hash::Hash};

pub use dir::*;
pub use vec2::*;

pub trait Offset: Display + Copy + PartialEq + Eq + Hash + Into<Vec2> {
    #[must_use]
    fn reverse(self) -> Self;

    #[must_use]
    fn turn_left(self) -> Self;

    #[must_use]
    fn turn_right(self) -> Self;

    #[must_use]
    fn is_ortho(self, other: Self) -> bool;

    #[must_use]
    fn is_reverse(self, other: Self) -> bool;
}
