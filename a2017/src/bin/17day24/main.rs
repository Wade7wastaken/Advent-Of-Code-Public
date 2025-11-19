use std::cmp::Ordering;

use lib::{itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_connectors(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            l.split('/')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn max_strength(prev_port: u32, connectors: &mut Vec<(u32, u32)>) -> u32 {
    let mut max = 0;
    for i in 0..connectors.len() {
        let c_ref = connectors.get(i).unwrap();
        if c_ref.0 != prev_port && c_ref.1 != prev_port {
            continue;
        }
        let c = connectors.remove(i);
        let res = c.0 + c.1 + max_strength(tern!(c.0 == prev_port, c.1, c.0), connectors);
        max = max.max(res);
        connectors.insert(i, c);
    }

    max
}

fn part1(input: &str) -> u32 {
    max_strength(0, &mut parse_connectors(input))
}

#[derive(Debug, Default)]
struct BridgeInfo {
    strength: u32,
    length: u32,
}

impl BridgeInfo {
    const fn add_connector(mut self, connector: (u32, u32)) -> Self {
        self.strength += connector.0 + connector.1;
        self.length += 1;
        self
    }
    fn add_bridge_candidate(&mut self, candidate: Self) {
        match candidate.length.cmp(&self.length) {
            Ordering::Greater => *self = candidate,
            Ordering::Equal => self.strength = self.strength.max(candidate.strength),
            Ordering::Less => {}
        }
    }
}

fn max_length(prev_ports: u32, connectors: &mut Vec<(u32, u32)>) -> BridgeInfo {
    let mut max = BridgeInfo::default();
    for i in 0..connectors.len() {
        let c_ref = connectors.get(i).unwrap();
        if !(c_ref.0 == prev_ports || c_ref.1 == prev_ports) {
            continue;
        }
        let c = connectors.remove(i);
        let res = max_length(tern!(c.0 == prev_ports, c.1, c.0), connectors).add_connector(c);

        max.add_bridge_candidate(res);

        connectors.insert(i, c);
    }

    max
}

fn part2(input: &str) -> u32 {
    max_length(0, &mut parse_connectors(input)).strength
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1906);
        assert_eq!(part2(input), 1824);
    }
}
