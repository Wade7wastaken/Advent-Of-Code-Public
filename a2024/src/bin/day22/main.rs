use std::collections::HashMap;

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

fn part2(input: &str) -> u64 {
    let maps = input.lines().map(|l| l.parse().unwrap()).map(|n| {
        let mut secret = n;
        // idea: cyclic iterator and difference iterator
        let windows = (0..)
            .map(|_| {
                let next = rng(secret);
                let diff = (next % 10) as i32 - (secret % 10) as i32;
                secret = next;
                (diff, next % 10)
            })
            .take(2000)
            .tuple_windows();

        let mut map = HashMap::new();
        for (a, b, c, d) in windows {
            let seq = (a.0, b.0, c.0, d.0);
            map.entry(seq).or_insert(d.1);
        }
        map
    });

    let mut a: HashMap<_, u64> = HashMap::new();

    for map in maps {
        for (k, v) in map {
            *a.entry(k).or_default() += v;
        }
    }

    a.into_values().max().unwrap()
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
