use lib::{itertools::Itertools, num::Integer};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let minmax = l
                .split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .minmax()
                .into_option()
                .unwrap();
            minmax.1 - minmax.0
        })
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
                    let (div, m) = a.div_rem(&b);
                    if m == 0 {
                        return Some(div);
                    }
                    let (div, m) = b.div_rem(&a);
                    if m == 0 {
                        return Some(div);
                    }
                    None
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
