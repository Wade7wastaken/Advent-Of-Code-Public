use std::{
    collections::HashMap,
    hash::Hash,
    iter::{Rev, Scan},
};

use either::Either;

use crate::tern;

pub trait IteratorExt: Iterator + Sized {
    fn collect_string(self) -> String
    where
        Self: Iterator<Item = u8>;

    fn collect_hashmap<K: Hash + Eq, Value, Bucket, R>(
        self,
        f_free: impl FnMut(Value) -> Bucket,
        f_taken: impl FnMut(&mut Bucket, Value) -> R,
    ) -> HashMap<K, Bucket>
    where
        Self: Iterator<Item = (K, Value)>;

    fn rev_if(self, cond: bool) -> Either<Self, Rev<Self>>
    where
        Self: DoubleEndedIterator;

    fn count_where<F: FnMut(Self::Item) -> bool>(self, f: F) -> usize;

    fn apply<State: Clone, F: FnMut(State, Self::Item) -> State>(
        self,
        init: State,
        f: F,
    ) -> Scan<Self, State, impl FnMut(&mut State, Self::Item) -> Option<State>>;

    fn detect_cycle(self) -> Option<(usize, usize, Vec<Self::Item>)>
    where
        Self::Item: Clone + Eq + Hash;

    fn nth_cyclic(self, n: usize) -> Option<Self::Item>
    where
        Self::Item: Clone + Eq + Hash;
}

impl<I: Iterator> IteratorExt for I {
    /// Collects an iterator of u8s into a String.
    fn collect_string(self) -> String
    where
        Self: Iterator<Item = u8>,
    {
        String::from_utf8(self.collect()).unwrap()
    }

    /// Collects an iterator of key value pairs into a hashmap using a closure
    /// to create a hashmap value, and a closure to insert a new item into an
    /// existing value.
    fn collect_hashmap<K: Hash + Eq, Value, Bucket, R>(
        self,
        mut f_free: impl FnMut(Value) -> Bucket,
        mut f_taken: impl FnMut(&mut Bucket, Value) -> R,
    ) -> HashMap<K, Bucket>
    where
        Self: Iterator<Item = (K, Value)>,
    {
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

    // Reverses self if a condition is true.
    fn rev_if(self, cond: bool) -> Either<Self, Rev<Self>>
    where
        Self: DoubleEndedIterator,
    {
        tern!(cond, Either::Right(self.rev()), Either::Left(self))
    }

    // Similar to self.filter(f).count() but f takes ownership of each item.
    fn count_where<F: FnMut(Self::Item) -> bool>(self, mut f: F) -> usize {
        self.fold(0, |count, x| tern!(f(x), count + 1, count))
    }

    /// Applies a mapping function to an iterator, but the previously returned
    /// item is available in the mapping function.
    fn apply<State: Clone, F: FnMut(State, Self::Item) -> State>(
        self,
        init: State,
        mut f: F,
    ) -> Scan<Self, State, impl FnMut(&mut State, Self::Item) -> Option<State>> {
        self.scan(init, move |st, x| {
            let res = f(st.clone(), x);
            *st = res.clone();
            Some(res)
        })
    }

    /// Detects a cycle in the iterator, returning the index of the start of the
    /// cycle, the length of the cycle, and the items collected. The items
    /// include all yielded values up to and including the first one seen twice.
    fn detect_cycle(self) -> Option<(usize, usize, Vec<Self::Item>)>
    where
        Self::Item: Clone + Eq + Hash,
    {
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

    /// Gets the nth element of a cyclic iterator, but skips redundant cycles.
    fn nth_cyclic(self, n: usize) -> Option<Self::Item>
    where
        Self::Item: Clone + Eq + Hash,
    {
        let (first, cycle_size, mut items) = self.detect_cycle()?;

        let times = (n - first) % cycle_size + first;
        Some(items.remove(times))
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn collect_string() {
        assert_eq!(
            [b'a', b'b', b'c', b'd'].into_iter().collect_string(),
            "abcd".to_string()
        );
    }

    #[test]
    fn collect_hashmap() {
        // Take first
        assert_eq!(
            [(1, 'a'), (2, 'b'), (3, 'd'), (3, 'c')]
                .into_iter()
                .collect_hashmap(|x| x, |_, _| ()),
            HashMap::from([(1, 'a'), (2, 'b'), (3, 'd')])
        );

        // Take last
        assert_eq!(
            [(1, 'a'), (2, 'b'), (3, 'd'), (3, 'c')]
                .into_iter()
                .collect_hashmap(|x| x, |bucket, new| *bucket = new),
            HashMap::from([(1, 'a'), (2, 'b'), (3, 'c')])
        );

        // Take all
        assert_eq!(
            [(1, 'a'), (2, 'b'), (3, 'd'), (3, 'c')]
                .into_iter()
                .collect_hashmap(|x| vec![x], Vec::push),
            HashMap::from([(1, vec!['a']), (2, vec!['b']), (3, vec!['d', 'c'])])
        );
    }

    #[test]
    fn rev_if() {
        let iter = [1, 2, 3, 4].into_iter();
        assert_eq!(iter.clone().rev_if(false).collect_vec(), vec![1, 2, 3, 4]);
        assert_eq!(iter.rev_if(true).collect_vec(), vec![4, 3, 2, 1]);
    }

    #[test]
    fn count_where() {
        assert_eq!(
            [1, 2, 3, 1, 5, 3, 6, 5, 4]
                .into_iter()
                .count_where(|x| x % 2 == 0),
            3
        );
    }

    #[test]
    fn apply() {
        assert_eq!(
            [1, 5, 3, -2]
                .into_iter()
                .apply(5, |acc, x| acc + x)
                .collect_vec(),
            vec![6, 11, 14, 12]
        );
    }

    #[test]
    fn detect_cycle() {
        assert_eq!(
            (0..).map(|i| i % 12).detect_cycle().unwrap(),
            (0, 12, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0])
        );
    }

    #[test]
    fn nth_cyclic() {
        // 1_000_000 % 12 == 4
        assert_eq!((0..).map(|i| i % 12).nth_cyclic(1_000_000).unwrap(), 4);
    }
}
