use std::collections::HashSet;

use lib::{itertools::Itertools, Point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut seen = HashSet::new();
    let mut pos = Point2::new(0, 0);
    seen.insert(pos);
    for dir in input.chars().map(|a| a.try_into().unwrap()) {
        pos = pos.apply(dir).unwrap();
        seen.insert(pos);
    }
    seen.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut seen = HashSet::new();
    let mut santa = Point2::new(0, 0);
    let mut robot = Point2::new(0, 0);
    seen.insert(santa);
    for (a, b) in input.chars().map(|a| a.try_into().unwrap()).tuples() {
        santa = santa.apply(a).unwrap();
        seen.insert(santa);
        robot = robot.apply(b).unwrap();
        seen.insert(robot);
    }
    seen.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 2572);
        assert_eq!(part2(input), 2631);
    }
}
