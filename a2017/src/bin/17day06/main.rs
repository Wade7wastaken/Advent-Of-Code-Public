use std::collections::{HashMap, HashSet};

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn redistribute(state: &mut [u32]) {
    let (i, &blocks) = state
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
        .unwrap();
    state[i] = 0;
    let mut blocks = blocks;
    let mut i = i + 1;
    while blocks > 0 {
        *state.get_mut(i % state.len()).unwrap() += 1;
        i += 1;
        blocks -= 1;
    }
}

fn parse_state(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec()
}

fn part1(input: &str) -> u32 {
    let mut state = parse_state(input);

    let mut seen = HashSet::new();
    let mut steps = 0;

    while seen.insert(state.clone()) {
        steps += 1;
        redistribute(&mut state);
    }

    steps
}

fn part2(input: &str) -> u32 {
    let mut state = parse_state(input);

    let mut seen = HashMap::new();
    let mut steps = 0;

    loop {
        if let Some(loop_start) = seen.insert(state.clone(), steps) {
            return steps - loop_start;
        }
        steps += 1;

        redistribute(&mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 4074);
        assert_eq!(part2(input), 2793);
    }
}
