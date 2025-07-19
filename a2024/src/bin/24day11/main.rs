use std::collections::HashMap;

use lib::{cycle, itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn process_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let log = stone.ilog10() + 1;
    tern!(
        log % 2 == 0,
        vec![stone / 10_u64.pow(log / 2), stone % 10_u64.pow(log / 2)],
        vec![stone * 2024]
    )
}

fn blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut map = HashMap::new();
    for (stone, freq) in stones {
        for processed in process_stone(stone) {
            *map.entry(processed).or_default() += freq;
        }
    }
    map
}

fn num_stones(input: &str, blinks: usize) -> u64 {
    let stones = input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .counts();

    cycle(stones, blinks, blink).into_values().sum::<usize>() as u64
}

fn part1(input: &str) -> u64 {
    num_stones(input, 25)
}

fn part2(input: &str) -> u64 {
    num_stones(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 218079);
        assert_eq!(part2(input), 259755538429618);
    }
}
