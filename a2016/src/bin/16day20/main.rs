use lib::{Range, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u64 {
    let ranges = input
        .lines()
        .map(|l| {
            l.split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b)| Range::new_inclusive(a, b))
        .collect_vec();

    (0..u64::MAX)
        .find(|i| ranges.iter().all(|r| !r.contains(i)))
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let ranges = input
        .lines()
        .map(|l| {
            l.split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b)| Range::new_inclusive(a, b))
        .collect_vec();

    let mut i: u64 = 0;
    let mut count = 0;
    'outer: while u32::try_from(i).is_ok() {
        for range in &ranges {
            if range.contains(&i) {
                i = *range.end();
                continue 'outer;
            }
        }
        count += 1;
        i += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 4793564);
        assert_eq!(part2(input), 146);
    }
}
