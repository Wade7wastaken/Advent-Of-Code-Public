use lib::{InclusiveRange, IteratorExt, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

fn parse_range(r: &str) -> InclusiveRange<u32> {
    let (a, b) = r
        .split('-')
        .map(|d| d.parse().unwrap())
        .collect_tuple()
        .unwrap();
    InclusiveRange::new(a, b)
}

fn part1(input: &str) -> u32 {
    input.lines().count_where(|l| {
        let (r1, r2) = l.split(',').map(parse_range).collect_tuple().unwrap();
        r1.covers(r2) || r2.covers(r1)
    }) as u32
}

fn part2(input: &str) -> u32 {
    todo!();
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
