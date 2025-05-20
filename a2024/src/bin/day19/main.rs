use std::collections::HashMap;

use lib::{CountWhere, StringTools, itertools::Itertools, regex::Regex, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let (patterns_str, towels_str) = input.split_paragraphs_once().unwrap();
    let patterns = patterns_str.split(", ").join("|");
    let regex = Regex::new(&format!("^({patterns})*$")).unwrap();
    towels_str
        .lines()
        .count_where(|towel| regex.is_match(towel)) as u32
}

fn num_ways<'a>(patterns: &[&str], towel: &'a str, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(prev) = cache.get(towel) {
        return *prev;
    }
    let res = patterns
        .iter()
        .map(|pattern| {
            towel.strip_prefix(pattern).map_or(0, |left| {
                tern!(left.is_empty(), 1, num_ways(patterns, left, cache))
            })
        })
        .sum();
    cache.insert(towel, res);
    res
}

fn part2(input: &str) -> u64 {
    let (patterns_str, towels_str) = input.split_paragraphs_once().unwrap();
    let patterns = patterns_str.split(", ").collect_vec();

    let mut cache = HashMap::new();

    towels_str
        .lines()
        .map(|towel| num_ways(&patterns, towel, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 317);
        assert_eq!(part2(input), 883443544805484);
    }
}
