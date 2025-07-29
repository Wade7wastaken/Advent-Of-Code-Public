use std::collections::{HashMap, HashSet};

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn find_group(start: u32, connections: &HashMap<u32, Vec<u32>>) -> HashSet<u32> {
    let mut programs = HashSet::new();
    let mut queue = vec![start];
    while let Some(next) = queue.pop() {
        if !programs.insert(next) {
            continue;
        }
        for child in connections.get(&next).unwrap() {
            queue.push(*child);
        }
    }
    programs
}

fn parse_connections(input: &str) -> HashMap<u32, Vec<u32>> {
    input
        .lines()
        .map(|l| l.split_once(" <-> ").unwrap())
        .map(|(program, children)| {
            (
                program.parse().unwrap(),
                children
                    .split(',')
                    .map(|c| c.trim().parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    find_group(0, &parse_connections(input)).len() as u32
}

fn part2(input: &str) -> u32 {
    let connections = parse_connections(input);
    let mut starts = connections.keys().copied().collect::<HashSet<_>>();
    let mut groups = 0;
    while !starts.is_empty() {
        let group = find_group(*starts.iter().next().unwrap(), &connections);
        for x in group {
            starts.remove(&x);
        }
        groups += 1;
    }

    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 283);
        assert_eq!(part2(input), 195);
    }
}
