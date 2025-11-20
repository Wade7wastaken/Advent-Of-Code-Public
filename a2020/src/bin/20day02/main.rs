use lib::{IteratorExt, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.lines().count_where(|l| {
        let (rule, password) = l.split_once(": ").unwrap();
        let (range, letter_str) = rule.split_once(' ').unwrap();
        let (lower, upper) = range
            .split('-')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let letter = letter_str.as_bytes()[0];
        (lower..=upper).contains(&password.bytes().count_where(|b| b == letter))
    }) as u32
}

fn part2(input: &str) -> u32 {
    input.lines().count_where(|l| {
        let (rule, password) = l.split_once(": ").unwrap();
        let (range, letter_str) = rule.split_once(' ').unwrap();
        let (i1, i2) = range
            .split('-')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let letter = letter_str.as_bytes()[0];
        let password = password.as_bytes();
        (password[i1 - 1] == letter) ^ (password[i2 - 1] == letter)
    }) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 560);
        assert_eq!(part2(input), 303);
    }
}
