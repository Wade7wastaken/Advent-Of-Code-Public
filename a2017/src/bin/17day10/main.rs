use a2017::{knot_hash, vec_reverse};
use lib::{Inline, itertools::Itertools};

use std::fmt::Write;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .enumerate()
        .fold(((0..=255).collect_vec(), 0), |(list, i), (skip, length)| {
            (vec_reverse(list, i, length), i + length + skip)
        })
        .0
        .into_iter()
        .take(2)
        .map(u32::from)
        .product()
}

fn part2(input: &str) -> String {
    knot_hash(input)
        .into_iter()
        .fold(String::new(), |output, b| {
            output.inline(|o| write!(o, "{b:x}"))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1980);
        assert_eq!(part2(input), "899124dac21012ebc32e2f4d11eaec55");
    }
}
