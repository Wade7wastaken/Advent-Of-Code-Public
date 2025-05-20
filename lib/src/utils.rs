use std::{collections::HashMap, hash::Hash, iter::Rev, ops::Sub, panic, str::Split};

use either::Either;
use itertools::Itertools;

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

// util for swapping a 2-tuple
pub trait Swap<A, B> {
    fn swap(self) -> (B, A);
}

impl<A, B> Swap<A, B> for (A, B) {
    fn swap(self) -> (B, A) {
        (self.1, self.0)
    }
}

pub trait SwapIf<T> {
    fn swap_if(self, swap: bool) -> (T, T);
}

impl<T> SwapIf<T> for (T, T) {
    fn swap_if(self, swap: bool) -> (T, T) {
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

// more fleshed-out HashMap collect
pub trait CollectHashmap<K: Hash + Eq, VMap, VIt> {
    fn collect_hashmap(
        self,
        f_free: impl Fn(VIt) -> VMap,
        f_taken: impl FnMut(&mut VMap, VIt),
    ) -> HashMap<K, VMap>;
}

impl<I: Iterator<Item = (K, Vit)>, K: Hash + Eq, VMap, Vit> CollectHashmap<K, VMap, Vit> for I {
    fn collect_hashmap(
        self,
        f_free: impl Fn(Vit) -> VMap,
        mut f_taken: impl FnMut(&mut VMap, Vit),
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

pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(
    f: F,
) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
