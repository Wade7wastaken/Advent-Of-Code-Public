use std::collections::{HashMap, HashSet};

use lib::{itertools::Itertools, select, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

type Happiness<'a> = HashMap<(&'a str, &'a str), i32>;

fn score_permutation(p: Vec<&str>, happiness: &Happiness) -> i32 {
    p.into_iter()
        .circular_tuple_windows()
        .map(|(a, b)| happiness.get(&(a, b)).unwrap())
        .sum()
}

fn find_max_score(input: &str, scoring_fn: fn(p: Vec<&str>, happiness: &Happiness) -> i32) -> u32 {
    let mut people = HashSet::new();
    let mut happiness = HashMap::new();

    for l in input.lines() {
        let (name, sign, n, other) = select!(l.split_ascii_whitespace(); 0, 2, 3, 10);
        let n = n.parse::<i32>().unwrap() * tern!(sign == "gain", 1, -1);
        people.insert(name);

        let other = other.strip_suffix('.').unwrap();

        *happiness.entry((name, other)).or_default() += n;
        *happiness.entry((other, name)).or_default() += n;
    }

    let num_people = people.len();

    people
        .into_iter()
        .permutations(num_people)
        .map(|perm| scoring_fn(perm, &happiness))
        .max()
        .unwrap() as u32
}

fn part1(input: &str) -> u32 {
    find_max_score(input, score_permutation)
}

fn score_permutation_non_circular(p: Vec<&str>, happiness: &Happiness) -> i32 {
    p.into_iter()
        .tuple_windows()
        .map(|(a, b)| happiness.get(&(a, b)).unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    find_max_score(input, score_permutation_non_circular)
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
