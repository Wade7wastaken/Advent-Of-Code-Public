use crate::{Range, Ranged, range};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct RangeSet<T: Copy>(Vec<Range<T>>);

impl<T: Copy + Ord> RangeSet<T> {
    #[must_use]
    pub fn new(ranges: Vec<Range<T>>) -> Self {
        Self(ranges).normalize()
    }

    fn normalize(mut self) -> Self {
        if self.0.is_empty() {
            return self;
        }
        self.0.sort_unstable();
        let mut new_ranges = vec![];
        let cur_range = self.0.into_iter().fold(None, |cur_range, r| {
            Some(cur_range.map_or(r, |prev_range: Range<T>| {
                prev_range.union(r).unwrap_or_else(|| {
                    new_ranges.push(prev_range);
                    r
                })
            }))
        });
        if let Some(prev_range) = cur_range {
            new_ranges.push(prev_range);
        }

        Self(new_ranges)
    }

    #[must_use]
    pub fn union(mut self, mut other: Self) -> Self {
        self.0.append(&mut other.0);
        self.normalize()
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut i = 0;
        let mut j = 0;

        let mut result = vec![];

        while let Some(a) = self.0.get(i)
            && let Some(b) = other.0.get(j)
        {
            if let Some(intersection) = a.intersection(*b) {
                result.push(intersection);
            }

            if a.end() < b.end() {
                i += 1;
            } else {
                j += 1;
            }
        }

        Self(result)
    }

    #[must_use]
    pub fn remove(self, other: &Self) -> Self {
        let mut output = vec![];

        let mut j = 0;

        for a in self.0 {
            let mut cur = a.start();

            while j < other.0.len() && other.0[j].end() <= cur {
                j += 1;
            }

            let mut k = j;
            while k < other.0.len() && other.0[k].start() < a.end() {
                let b = other.0[k];
                if b.start() > cur {
                    output.push(range(cur, b.start().min(a.end())));
                }

                cur = cur.max(b.end());
                if cur >= a.end() {
                    break;
                }
                k += 1;
            }

            if cur < a.end() {
                output.push(range(cur, a.end()));
            }

            j = k;
        }

        Self(output)
    }
}

#[must_use]
pub fn range_set<T: Copy + Ord>(ranges: Vec<Range<T>>) -> RangeSet<T> {
    RangeSet::new(ranges)
}

#[cfg(test)]
mod tests {
    use crate::range;

    use super::*;

    #[test]
    fn normalize() {
        assert_eq!(RangeSet::<i32>::default().0, vec![]);
        assert_eq!(range_set(vec![range(100, 200)]).0, vec![range(100, 200)]);
        assert_eq!(
            range_set(vec![range(100, 200), range(100, 200)]).0,
            vec![range(100, 200)]
        );
        assert_eq!(
            range_set(vec![range(100, 200), range(100, 300)]).0,
            vec![range(100, 300)]
        );
        assert_eq!(
            range_set(vec![range(100, 300), range(100, 200)]).0,
            vec![range(100, 300)]
        );
        assert_eq!(
            range_set(vec![range(100, 200), range(150, 200)]).0,
            vec![range(100, 200)]
        );
        assert_eq!(
            range_set(vec![range(100, 200), range(50, 300)]).0,
            vec![range(50, 300)]
        );
    }

    #[test]
    fn union() {
        let base = range_set(vec![range(100, 200), range(300, 400)]);

        let other = range_set(vec![]);
        let expected = range_set(vec![range(100, 200), range(300, 400)]);
        assert_eq!(base.clone().union(other), expected);

        let other = range_set(vec![]);
        let expected = range_set(vec![range(100, 200), range(300, 400)]);
        assert_eq!(other.union(base.clone()), expected);

        let other = range_set(vec![range(0, 100)]);
        let expected = range_set(vec![range(0, 200), range(300, 400)]);
        assert_eq!(base.clone().union(other), expected);

        let other = range_set(vec![range(0, 150)]);
        let expected = range_set(vec![range(0, 200), range(300, 400)]);
        assert_eq!(base.clone().union(other), expected);

        let other = range_set(vec![range(50, 250)]);
        let expected = range_set(vec![range(50, 250), range(300, 400)]);
        assert_eq!(base.clone().union(other), expected);

        let other = range_set(vec![range(200, 300)]);
        let expected = range_set(vec![range(100, 400)]);
        assert_eq!(base.clone().union(other), expected);

        let other = range_set(vec![range(0, 500)]);
        let expected = range_set(vec![range(0, 500)]);
        assert_eq!(base.clone().union(other), expected);

        let other = range_set(vec![range(225, 275), range(0, 75)]);
        let expected = range_set(vec![
            range(0, 75),
            range(100, 200),
            range(225, 275),
            range(300, 400),
        ]);
        assert_eq!(base.clone().union(other), expected);
    }

    #[test]
    fn intersection() {
        let base = range_set(vec![range(100, 200), range(300, 400)]);

        let other = range_set(vec![range(0, 100)]);
        let expected = range_set(vec![]);
        assert_eq!(base.clone().intersection(&other), expected);

        let other = range_set(vec![range(0, 150)]);
        let expected = range_set(vec![range(100, 150)]);
        assert_eq!(base.clone().intersection(&other), expected);

        let other = range_set(vec![range(50, 250)]);
        let expected = range_set(vec![range(100, 200)]);
        assert_eq!(base.clone().intersection(&other), expected);

        let other = range_set(vec![range(200, 300)]);
        let expected = range_set(vec![]);
        assert_eq!(base.clone().intersection(&other), expected);

        let other = range_set(vec![range(0, 500)]);
        let expected = range_set(vec![range(100, 200), range(300, 400)]);
        assert_eq!(base.clone().intersection(&other), expected);

        let other = range_set(vec![range(225, 275), range(0, 75)]);
        let expected = range_set(vec![]);
        assert_eq!(base.clone().intersection(&other), expected);
    }
}
