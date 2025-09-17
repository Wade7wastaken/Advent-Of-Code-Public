use lib::itertools::Itertools;
use lib::{IteratorExt, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_number_list(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn is_valid(nums: &[u32]) -> bool {
    let is_increasing = nums[1] > nums[0];
    nums.iter()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&b.abs_diff(*a)) && tern!(is_increasing, a < b, a > b))
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_number_list)
        .count_where(|nums| is_valid(&nums)) as u32
}

fn clone_and_remove(nums: &[u32], i: usize) -> Vec<u32> {
    let mut new_nums = nums.to_owned();
    new_nums.remove(i);
    new_nums
}

fn part2(input: &str) -> u32 {
    input.lines().map(parse_number_list).count_where(|nums| {
        is_valid(&nums.clone()) || (0..nums.len()).any(|i| is_valid(&clone_and_remove(&nums, i)))
    }) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 407);
        assert_eq!(part2(input), 459);
    }
}
