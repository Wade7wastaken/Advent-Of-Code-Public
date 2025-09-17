use std::collections::{HashMap, HashSet};

use lib::{IteratorExt, StringTools, Swap, defer, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_rule_map(rules_str: &str) -> HashMap<u32, Vec<u32>> {
    // key: number to insert, value: numbers that must be before
    rules_str
        .lines()
        .map(|l| {
            l.split('|')
                .map(|s| s.parse().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
                .swap()
        })
        .collect_hashmap(|v| vec![v], Vec::push)
}

fn follows_rules(map: &HashMap<u32, Vec<u32>>, nums: &[u32]) -> bool {
    let mut seen_nums = HashSet::new();
    let nums_set = nums.iter().collect::<HashSet<_>>();
    nums.iter().all(|n| {
        defer!(
            map.get(n).is_none_or(|required| {
                required
                    .iter()
                    .all(|x| !nums_set.contains(x) || seen_nums.contains(x))
            });
            seen_nums.insert(*n)
        )
    })
}

fn part1(input: &str) -> u32 {
    let (rules_str, numbers) = input.split_paragraphs_once().unwrap();
    let map = parse_rule_map(rules_str);

    numbers
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect_vec())
        .filter(|nums| follows_rules(&map, nums))
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

// returns whether to continue correcting nums
fn correct_num(nums: &mut Vec<u32>, map: &HashMap<u32, Vec<u32>>, nums_set: &HashSet<u32>) -> bool {
    let mut seen_nums = HashSet::new();
    for (i, n) in nums.iter().copied().enumerate() {
        if let Some(required) = map.get(&n) {
            let found = required
                .iter()
                .find(|x| !seen_nums.contains(*x) && nums_set.contains(x));

            if let Some(x) = found {
                let idx = nums.iter().position(|a| a == x).unwrap();
                // swap is slower for some reason
                let removed = nums.remove(idx);
                nums.insert(i, removed);
                return true;
            }
        }
        seen_nums.insert(n);
    }
    false
}

fn correct_nums(nums: &mut Vec<u32>, map: &HashMap<u32, Vec<u32>>) {
    let nums_set = nums.iter().copied().collect::<HashSet<_>>();
    while correct_num(nums, map, &nums_set) {}
}

fn part2(input: &str) -> u32 {
    let (rules_str, numbers) = input.split_paragraphs_once().unwrap();
    let map = parse_rule_map(rules_str);

    numbers
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect_vec())
        .filter(|nums| !follows_rules(&map, nums))
        .update(|nums| correct_nums(nums, &map))
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 7365);
        assert_eq!(part2(input), 5770);
    }
}
