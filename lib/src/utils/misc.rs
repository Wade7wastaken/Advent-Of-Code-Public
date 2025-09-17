use std::{hint::unreachable_unchecked, ops::Sub};

use md5::{
    digest::{Output, OutputSizeUser},
    Digest,
};

use crate::{defer, tern};

/// Equivalent to `(a-b).abs()` but underflow safe.
pub fn abs_diff<T: PartialOrd + Sub<Output = T>>(a: T, b: T) -> T {
    tern!(a > b, a - b, b - a)
}

/// If a and b are equal, return Some(a), else None.
pub fn equal_combine<T: PartialEq>(a: T, b: T) -> Option<T> {
    defer!((a == b).then_some(a); drop(b))
}

/// Runs a function on some data cyclically i times.
#[inline]
pub fn cycle<T>(mut state: T, i: usize, mut f: impl FnMut(T) -> T) -> T {
    for _ in 0..i {
        state = f(state);
    }
    state
}

pub trait Inline<R> {
    #[must_use]
    fn inline(self, f: impl FnOnce(&mut Self) -> R) -> Self;
}

impl<T, R> Inline<R> for T {
    /// Runs a closure that takes a mutable reference to self, then returns an
    /// owned mutated self.
    fn inline(mut self, f: impl FnOnce(&mut Self) -> R) -> Self {
        f(&mut self);
        self
    }
}

/// Converts a hex digit into it's character representation.
#[must_use]
pub fn to_char(x: u8) -> u8 {
    debug_assert!(matches!(x, 0x0..=0xf));
    match x {
        0x0..=0x9 => x + b'0',
        0xa..=0xf => x - 0xa + b'a',
        _ => unsafe { unreachable_unchecked() },
    }
}

// pub trait DigestHex<T> {
//     fn to_hex(&self) -> [u8; 32];
//     fn hex_digit(&self, i: usize) -> u8;
// }

// impl<T: OutputSizeUser> DigestHex<T> for Output<T> {
/// Transforms a Digest into 32 hex digits.
pub fn to_hex(data: [u8; 16]) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..16 {
        let byte = data[i];
        res[2 * i] = to_char(byte >> 4);
        res[2 * i + 1] = to_char(byte & 0x0f);
    }
    res
}

/// Retrieves a specific hex digit from a Digest.
pub fn hex_digit(data: &[u8], i: usize) -> u8 {
    let a = data[i / 2];
    let b = tern!(i % 2 == 0, a >> 4, a & 0xf);
    to_char(b)
}
// }
