use std::{
    collections::{HashMap, HashSet},
    iter,
};

use lib::{cycle, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn rng(input: u64) -> u64 {
    let s1 = ((input * 64) ^ input) % 16777216;
    let s2 = ((s1 / 32) ^ s1) % 16777216;
    ((s2 * 2048) ^ s2) % 16777216
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|n| cycle(n, 2000, rng))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut abc = HashMap::new();
    let mut seen = HashSet::new();

    for secret in input.lines().map(|l| l.parse().unwrap()) {
        let windows = iter::successors(Some(secret), |x| Some(rng(*x)))
            .map(|x| (x % 10) as u32)
            .tuple_windows()
            .map(|(a, b)| (a as i8 - b as i8, b))
            .take(2000)
            .tuple_windows();

        for (a, b, c, d) in windows {
            let seq = (a.0, b.0, c.0, d.0);
            if !seen.insert(seq) {
                continue;
            }
            *abc.entry(seq).or_default() += d.1;
        }
        seen.clear();
    }

    abc.into_values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 12664695565);
        assert_eq!(part2(input), 1444);
    }
}
