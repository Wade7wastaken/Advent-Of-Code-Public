use std::collections::HashSet;

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let nums = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect_vec();

    let map = nums.iter().copied().collect::<HashSet<_>>();
    for n in nums {
        let target = 2020 - n;
        if let Some(&other) = map.get(&target) {
            return n * other;
        }
    }

    panic!()
}

fn part2(input: &str) -> u32 {
    let nums = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect_vec();

    let map = nums.iter().copied().collect::<HashSet<_>>();
    for n1 in nums.iter().copied() {
        for n2 in nums.iter().copied() {
            if n1 + n2 <= 2020 {
                let target = 2020 - n1 - n2;
                if let Some(&other) = map.get(&target) {
                    return n1 * n2 * other;
                }
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 982464);
        assert_eq!(part2(input), 162292410);
    }
}
