use std::collections::{HashMap, HashSet};

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_dist_line(line: &str) -> (&str, &str, u32) {
    let (src, _, end, _, dist) = line.split_ascii_whitespace().collect_tuple().unwrap();
    (src, end, dist.parse().unwrap())
}

fn score_permutation(dist_map: &HashMap<(&str, &str), u32>, p: Vec<&str>) -> u32 {
    p.into_iter()
        .tuple_windows()
        .map(|edge| dist_map.get(&edge).unwrap())
        .sum()
}

fn build_graph(input: &str) -> (HashMap<(&str, &str), u32>, HashSet<&str>) {
    let mut dist_map = HashMap::new();
    let mut cities = HashSet::new();
    for (src, end, dist) in input.lines().map(parse_dist_line) {
        cities.insert(src);
        cities.insert(end);
        dist_map.insert((src, end), dist);
        dist_map.insert((end, src), dist);
    }
    (dist_map, cities)
}

fn all_path_lens(input: &str) -> impl Iterator<Item = u32> {
    let (dist_map, cities) = build_graph(input);
    let n = cities.len();
    cities
        .into_iter()
        .permutations(n)
        .map(move |p| score_permutation(&dist_map, p))
}

fn part1(input: &str) -> u32 {
    all_path_lens(input).min().unwrap()
}

fn part2(input: &str) -> u32 {
    all_path_lens(input).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 141);
        assert_eq!(part2(input), 736);
    }
}
