use lib::{InclusiveRange, Range, RangeSet, Ranged, StringTools, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    // println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let (fresh, available) = input.split_paragraphs_once().unwrap();
    let fresh_ranges = fresh
        .lines()
        .map(|x| {
            x.split('-')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b)| InclusiveRange::new(a, b)).collect_vec();

        let mut ans = 0;

    for a in available.lines().map(|l| l.parse().unwrap()) {
        if fresh_ranges.iter().find(|r| r.contains(a)).is_some() {
            ans += 1;
        }
    }
    ans
}

fn normalize(mut ranges: Vec<InclusiveRange<u64>>) -> Vec<InclusiveRange<u64>> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_unstable();
    let mut new_ranges = vec![];
    let cur_range = ranges.into_iter().fold(None, |cur_range, r| {
        Some(cur_range.map_or(r, |prev_range: InclusiveRange<u64>| {
            prev_range.union(r).unwrap_or_else(|| {
                new_ranges.push(prev_range);
                r
            })
        }))
    });
    if let Some(prev_range) = cur_range {
        new_ranges.push(prev_range);
    }

    new_ranges
}

fn part2(input: &str) -> u64 {
    let (fresh, available) = input.split_paragraphs_once().unwrap();
    let fresh_ranges = fresh
        .lines()
        .map(|x| {
            x.split('-')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b)| InclusiveRange::new(a, b)).collect_vec();

    let set = normalize(fresh_ranges);
    println!("{set:?}");
    set.into_iter().map(|r| r.end() - r.start() + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        // assert_eq!(part1(input), todo!());
        // assert_eq!(part2(input), todo!());
    }
}
