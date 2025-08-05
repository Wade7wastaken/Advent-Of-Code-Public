use std::collections::HashMap;

use lib::{itertools::Itertools, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

struct CellAction<'a> {
    value: bool,
    offset: i32,
    state_out: &'a str
}

struct StateAction<'a> {
    zero:CellAction<'a>,
    one: CellAction<'a>
}

fn part1(input: &str) -> u32 {
    let mut tape = HashMap::new();
    let mut paragraphs = input.paragraphs();
    let start = paragraphs.next().unwrap();
    let (start_state_str, steps_str) = start.lines().collect_tuple().unwrap();
    let start_state = start_state_str.strip_prefix("Begin in state ").unwrap().strip_suffix('.').unwrap();
    let steps = steps_str.strip_prefix("Perform a diagnostic checksum after ").unwrap().strip_suffix(" steps.").unwrap().parse().unwrap();

    paragraphs.map(|p| {
        let mut lines = p.lines().map(|l| l.trim());
        let state_str = lines.next().unwrap();
        let state = state_str.strip_prefix("In state ").unwrap().strip_prefix(':').unwrap();
    })
}

fn part2(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        // assert_eq!(part1(input), todo!());
        // assert_eq!(part2(input), todo!());
    }
}

