use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .minmax()
                .into_option()
                .unwrap()
        })
        .map(|x| x.1 - x.0)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .tuple_combinations()
                .filter_map(|(a, b)| {
                    (a % b == 0)
                        .then_some(a / b)
                        .or_else(|| (b % a == 0).then_some(b / a))
                })
                .exactly_one()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 41919);
        assert_eq!(part2(input), 303);
    }
}
