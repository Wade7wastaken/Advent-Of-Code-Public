use std::collections::HashMap;

use lib::{num::Integer, Dir, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Map {
    dirs: Vec<Dir>,
    nodes: HashMap<String, (String, String)>,
}

fn parse_map(s: &str) -> Map {
    let (dirs_str, nodes_str) = s.split_paragraphs_once().unwrap();
    let dirs = dirs_str
        .chars()
        .map(|c| match c {
            'R' => Dir::EAST,
            'L' => Dir::WEST,
            _ => unreachable!("Unknown direction: {}", c),
        })
        .collect();

    let nodes = nodes_str
        .lines()
        .map(|node| {
            let (src, dests_str) = node.split_once(" = ").unwrap();
            let (dest_a, dest_b) = dests_str
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.split_once(", "))
                .unwrap();

            (src.to_string(), (dest_a.to_string(), dest_b.to_string()))
        })
        .collect();
    Map { dirs, nodes }
}

fn traverse_map(map: &Map, starting_node: &str, break_condition: impl Fn(&str) -> bool) -> u64 {
    let mut count = 0;
    let mut cur_node = starting_node;
    for dir in map.dirs.iter().cycle() {
        if break_condition(cur_node) {
            break;
        }
        let node = map.nodes.get(cur_node).unwrap();
        cur_node = match *dir {
            Dir::WEST => &node.0,
            Dir::EAST => &node.1,
            _ => unreachable!(),
        };
        count += 1;
    }
    count
}

fn part1(input: &str) -> u64 {
    traverse_map(&parse_map(input), "AAA", |node| node == "ZZZ")
}

fn part2(input: &str) -> u64 {
    let map = parse_map(input);

    map.nodes
        .iter()
        .map(|l| l.0)
        .filter(|n| n.ends_with('A'))
        .map(|node| traverse_map(&map, node, |node| node.ends_with('Z')))
        .reduce(|acc, e| acc.lcm(&e))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 13301);
        assert_eq!(part2(input), 7309459565207);
    }
}
