use lib::{itertools::Itertools, CountWhere, Range};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

fn parse_range(r: &str) -> Range<u32> {
    let (a, b) = r
        .split('-')
        .map(|d| d.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Range::new_inclusive(a, b)
}

fn part1(input: &str) -> u32 {
    input.lines().count_where(|l| {
        let (r1, r2) = l.split(',').map(parse_range).collect_tuple().unwrap();
        r1.contains_range(&r2) || r2.contains_range(&r1)
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
