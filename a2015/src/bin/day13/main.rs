use std::collections::{HashMap, HashSet};

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn score_permutation(p: Vec<&str>, happiness: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    p.into_iter()
        .circular_tuple_windows()
        .map(|(a, b)| {
            happiness.get(a).unwrap().get(b).unwrap() + happiness.get(b).unwrap().get(a).unwrap()
        })
        .sum()
}

fn part1(input: &str) -> u32 {
    let mut people: HashSet<&str> = HashSet::new();
    let mut happiness: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    for (name, _, sign, n, _, _, _, _, _, _, other) in input
        .lines()
        .map(|l| l.split_whitespace().collect_tuple().unwrap())
    {
        let n: i32 = n.parse().unwrap();
        let n = match sign {
            "gain" => n,
            "lose" => -n,
            _ => panic!(),
        };
        people.insert(name);
        happiness
            .entry(name)
            .or_default()
            .insert(other.strip_suffix('.').unwrap(), n);
    }

    let num_people = people.len();

    people
        .into_iter()
        .permutations(num_people)
        .map(|perm| score_permutation(perm, &happiness))
        .max()
        .unwrap() as u32
}

fn score_permutation_non_circular(p: Vec<&str>, happiness: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    p.into_iter()
        .tuple_windows()
        .map(|(a, b)| {
            happiness.get(a).unwrap().get(b).unwrap() + happiness.get(b).unwrap().get(a).unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut people: HashSet<&str> = HashSet::new();
    let mut happiness: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    for (name, _, sign, n, _, _, _, _, _, _, other) in input
        .lines()
        .map(|l| l.split_whitespace().collect_tuple().unwrap())
    {
        let n: i32 = n.parse().unwrap();
        let n = match sign {
            "gain" => n,
            "lose" => -n,
            _ => panic!(),
        };
        people.insert(name);
        happiness
            .entry(name)
            .or_default()
            .insert(other.strip_suffix('.').unwrap(), n);
    }

    let num_people = people.len();

    people
        .into_iter()
        .permutations(num_people)
        .map(|perm| score_permutation_non_circular(perm, &happiness))
        .max()
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 709);
        assert_eq!(part2(input), 668);
    }
}
