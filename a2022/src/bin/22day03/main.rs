use std::collections::HashSet;

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
            let (a, b) = l.split_at(l.len() / 2);
            let shared = *a
                .bytes()
                .collect::<HashSet<_>>()
                .intersection(&b.bytes().collect())
                .exactly_one()
                .unwrap();
            u32::from(match shared {
                b'a'..=b'z' => shared - b'a' + 1,
                b'A'..=b'Z' => shared - b'A' + 27,
                _ => unreachable!(),
            })
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let shared = *a
                .bytes()
                .collect::<HashSet<_>>()
                .intersection(&b.bytes().collect())
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&c.bytes().collect())
                .exactly_one()
                .unwrap();
            u32::from(match shared {
                b'a'..=b'z' => shared - b'a' + 1,
                b'A'..=b'Z' => shared - b'A' + 27,
                _ => unreachable!(),
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 7428);
        assert_eq!(part2(input), 2650);
    }
}
