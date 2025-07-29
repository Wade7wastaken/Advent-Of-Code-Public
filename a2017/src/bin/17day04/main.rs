use lib::{CountWhere, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .count_where(|l| l.split_ascii_whitespace().all_unique()) as u32
}

fn part2(input: &str) -> u32 {
    input.lines().count_where(|l| {
        l.split_ascii_whitespace()
            .map(|word| word.bytes().counts())
            .tuple_combinations()
            .all(|(a, b)| a != b)
    }) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 386);
        assert_eq!(part2(input), 208);
    }
}
