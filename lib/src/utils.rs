use std::{
    collections::HashMap, hash::Hash, hint::unreachable_unchecked, iter::Rev, ops::Sub, str::Split,
};

use either::Either;
use itertools::Itertools;
use md5::Digest;

#[macro_export]
macro_rules! tern {
    ($cond:expr, $a:expr, $b:expr) => {
        if $cond {
            $a
        } else {
            $b
        }
    };
}

#[macro_export]
macro_rules! select {
    ($iter:expr; $first:expr $(, $rest:expr)*) => {
        {
            let mut __iter = &mut $iter;
            let __first_item = __iter.nth($first).unwrap();
            select!(@acc __iter, (__first_item), $first; $($rest),*)
        }
    };

    (@acc $iter:expr, ($($picked:expr),*), $prev:expr; $next:expr $(, $rest:expr)*) => {
        {
            let __skip = $next - $prev - 1;
            let __item = $iter.nth(__skip).unwrap();
            select!(@acc $iter, ($($picked,)* __item), $next; $($rest),*)
        }
    };

    (@acc $_iter:expr, ($($picked:expr),*), $_last:expr;) => {
        ($($picked),*)
    };
}

// (a-b).abs() but underflow safe
pub fn abs_diff<T: PartialOrd + Sub<Output = T>>(a: T, b: T) -> T {
    tern!(a > b, a - b, b - a)
}

// conditionally reverses an iterator
pub trait ConditionalRev<T: DoubleEndedIterator> {
    fn rev_if(self, cond: bool) -> Either<T, Rev<T>>;
}

impl<T: DoubleEndedIterator> ConditionalRev<T> for T {
    fn rev_if(self, cond: bool) -> Either<T, Rev<T>> {
        tern!(cond, Either::Right(self.rev()), Either::Left(self))
    }
}

// util for iter.filter(f).count() but f takes ownership
pub trait CountWhere<Item, F: Fn(Item) -> bool> {
    fn count_where(self, f: F) -> usize;
}

impl<Item, F: Fn(Item) -> bool, I: Iterator<Item = Item>> CountWhere<Item, F> for I {
    fn count_where(self, f: F) -> usize {
        self.fold(0, |count, x| tern!(f(x), count + 1, count))
    }
}

pub trait DetectCycle<Item> {
    // start, len, items
    fn detect_cycle(self) -> Option<(usize, usize, Vec<Item>)>;

    fn nth_cyclic(self, n: usize) -> Option<Item>
    where
        Self: Sized,
    {
        let (first, cycle_size, mut items) = self.detect_cycle()?;

        let times = (n - first) % cycle_size + first;

        Some(items.remove(times))
    }
}

impl<Item: Clone + Eq + Hash, I: Iterator<Item = Item>> DetectCycle<Item> for I {
    fn detect_cycle(self) -> Option<(usize, usize, Vec<Item>)> {
        let mut vec = Vec::new();
        let mut map = HashMap::new();
        for (i, item) in self.enumerate() {
            vec.push(item.clone());
            if let Some(prev) = map.insert(item, i) {
                return Some((prev, i - prev, vec));
            }
        }

        None
    }
}

// util for swapping a 2-tuple
pub trait Swap<A, B> {
    fn swap(self) -> (B, A);
}

impl<A, B> Swap<A, B> for (A, B) {
    fn swap(self) -> (B, A) {
        (self.1, self.0)
    }
}

pub trait SwapIf {
    #[must_use]
    fn swap_if(self, swap: bool) -> Self;
}

impl<T> SwapIf for (T, T) {
    fn swap_if(self, swap: bool) -> Self {
        tern!(swap, (self.1, self.0), (self.0, self.1))
    }
}

// runs a function on some data cyclically i times
#[inline]
pub fn cycle<T>(mut initial: T, i: usize, f: impl Fn(T) -> T) -> T {
    for _ in 0..i {
        initial = f(initial);
    }
    initial
}

// string utilities
pub trait StringTools {
    fn split_lines_once(&self) -> Option<(&str, &str)>;

    fn paragraphs(&self) -> Split<'_, &str>;

    fn split_paragraphs_once(&self) -> Option<(&str, &str)>;
}

impl StringTools for &str {
    fn split_lines_once(&self) -> Option<(&str, &str)> {
        self.lines().collect_tuple()
    }

    fn paragraphs(&self) -> Split<'_, &str> {
        tern!(
            self.contains("\r\n\r\n"),
            self.split("\r\n\r\n"),
            self.split("\n\n")
        )
    }

    fn split_paragraphs_once(&self) -> Option<(&str, &str)> {
        self.paragraphs().collect_tuple()
    }
}

pub trait SliceTools<T> {
    fn at(&self, i: usize) -> &T;

    fn at_mut(&mut self, i: usize) -> &mut T;
}

impl<T> SliceTools<T> for [T] {
    fn at(&self, i: usize) -> &T {
        self.get(i % self.len()).unwrap()
    }

    fn at_mut(&mut self, i: usize) -> &mut T {
        let len = self.len();
        self.get_mut(i % len).unwrap()
    }
}

// more fleshed-out HashMap collect
pub trait CollectHashmap<K: Hash + Eq, VMap, VIt, R> {
    fn collect_hashmap(
        self,
        f_free: impl Fn(VIt) -> VMap,
        f_taken: impl FnMut(&mut VMap, VIt) -> R,
    ) -> HashMap<K, VMap>;
}

impl<I: Iterator<Item = (K, Vit)>, K: Hash + Eq, VMap, Vit, R> CollectHashmap<K, VMap, Vit, R>
    for I
{
    fn collect_hashmap(
        self,
        f_free: impl Fn(Vit) -> VMap,
        mut f_taken: impl FnMut(&mut VMap, Vit) -> R,
    ) -> HashMap<K, VMap> {
        let mut map = HashMap::new();
        for (k, v) in self {
            if let Some(v2) = map.get_mut(&k) {
                f_taken(v2, v);
            } else {
                map.insert(k, f_free(v));
            }
        }
        map
    }
}

pub trait CollectString {
    fn collect_string(self) -> String;
}

impl<I: Iterator<Item = u8>> CollectString for I {
    fn collect_string(self) -> String {
        String::from_utf8_lossy(&self.collect_vec()).into_owned()
    }
}

fn to_char(x: u8) -> u8 {
    debug_assert!(matches!(x, 0x0..=0xf));
    match x {
        0x0..=0x9 => x + b'0',
        0xa..=0xf => x - 0xa + b'a',
        _ => unsafe { unreachable_unchecked() },
    }
}

pub trait DigestHex {
    fn to_hex(&self) -> [u8; 32];
    fn hex_digit(&self, i: usize) -> u8;
}

impl DigestHex for Digest {
    fn to_hex(&self) -> [u8; 32] {
        let mut res = [0; 32];
        for i in 0..16 {
            let byte = self.0[i];
            res[2 * i] = to_char(byte >> 4);
            res[2 * i + 1] = to_char(byte & 0x0f);
        }
        res
    }
    fn hex_digit(&self, i: usize) -> u8 {
        let a = self.0[i / 2];
        let b = tern!(i % 2 == 0, a >> 4, a & 0xf);
        to_char(b)
    }
}

pub fn equal_combine<T: PartialEq>(a: T, b: T) -> Option<T> {
    let res = (a == b).then_some(a);
    drop(b);
    res
}

pub trait Inline<R> {
    #[must_use]
    fn inline(self, f: impl Fn(&mut Self) -> R) -> Self;
}

impl<E, R> Inline<R> for E {
    fn inline(mut self, f: impl Fn(&mut Self) -> R) -> Self {
        f(&mut self);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest() {
        let digest = md5::compute("abc");
        let hex = digest.to_hex();
        for (i, h) in hex.into_iter().enumerate() {
            assert_eq!(h, digest.hex_digit(i));
        }
    }

    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(5, 3), 2);
        assert_eq!(abs_diff(3, 5), 2);
        assert_eq!(abs_diff(5, 5), 0);
        assert_eq!(abs_diff(1u8, 5u8), 4u8);
    }

    #[test]
    fn test_rev_if() {
        let arr = [1, 2, 3, 4, 5];
        let rev: Vec<_> = arr.into_iter().rev_if(true).collect();
        assert_eq!(rev, vec![5, 4, 3, 2, 1]);

        let not_rev: Vec<_> = arr.into_iter().rev_if(false).collect();
        assert_eq!(not_rev, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_count_where() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let count = vec.into_iter().count_where(|x| x % 2 == 0);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_swap() {
        assert_eq!((1, "a").swap(), ("a", 1));
    }

    #[test]
    fn test_tern() {
        assert_eq!(tern!(true, 1, 2), 1);
        assert_eq!(tern!(false, 1, 2), 2);
    }

    #[test]
    fn test_cycle() {
        let result = cycle(1, 3, |x| x + 1);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_split_lines_once() {
        let s = "line1\r\nline2";
        assert_eq!(s.split_lines_once(), Some(("line1", "line2")));
        let s = "line1\r\nline2\r\nline3";
        assert_eq!(s.split_lines_once(), None);
        let s = "line1line2line3";
        assert_eq!(s.split_lines_once(), None);
    }

    #[test]
    fn test_paragraphs() {
        let s = "para1\n\npara2\n\npara3";
        let paragraphs = s.paragraphs().collect_vec();
        assert_eq!(paragraphs, vec!["para1", "para2", "para3"]);
    }

    #[test]
    fn test_split_paragraphs_once() {
        let s = "para1\r\n\r\npara2";
        assert_eq!(s.split_paragraphs_once(), Some(("para1", "para2")));
        let s = "para1\r\n\r\npara2\r\n\r\npara3";
        assert_eq!(s.split_paragraphs_once(), None);
        let s = "para1para2\r\npara3";
        assert_eq!(s.split_paragraphs_once(), None);
    }

    #[test]
    fn test_collect_hashmap() {
        let vec = vec![(1, "a"), (1, "b"), (2, "c")];
        let map = vec.into_iter().collect_hashmap(|v| vec![v], Vec::push);
        assert_eq!(map.get(&1), Some(&vec!["a", "b"]));
        assert_eq!(map.get(&2), Some(&vec!["c"]));
    }

    #[test]
    fn test_collect_hashmap_empty() {
        let vec: Vec<(i32, &str)> = vec![];
        let map = vec.into_iter().collect_hashmap(|v| vec![v], Vec::push);
        assert!(map.is_empty());
    }

    #[test]
    fn test_collect_hashmap_overwrite() {
        let vec = vec![(1, "a"), (1, "b"), (2, "c")];
        let map: HashMap<_, _> = vec
            .into_iter()
            .collect_hashmap(|v| v, |vmap, vit| *vmap = vit);
        assert_eq!(map.get(&1), Some(&"b"));
        assert_eq!(map.get(&2), Some(&"c"));
    }

    #[test]
    fn test_collect_hashmap_keep() {
        let vec = vec![(1, "a"), (1, "b"), (2, "c")];
        let map: HashMap<_, _> = vec.into_iter().collect_hashmap(|v| v, |_, _| {});
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), Some(&"c"));
    }
}
