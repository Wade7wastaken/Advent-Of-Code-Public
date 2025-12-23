use std::collections::HashSet;

use lib::{Inline, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point3 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Point3 {
    const fn from_tuple(t: (u64, u64, u64)) -> Self {
        Self {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }

    const fn dist(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

fn parse_boxes(input: &str) -> Vec<Point3> {
    input
        .lines()
        .map(|x| {
            Point3::from_tuple(
                x.split(',')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect_vec()
}

fn create_pairs(input: &str) -> Vec<(Point3, Point3)> {
    parse_boxes(input)
        .into_iter()
        .tuple_combinations()
        .collect_vec()
        .inline(|pairs| pairs.sort_by_key(|(a, b)| a.dist(b)))
}

fn part1(input: &str) -> u64 {
    let mut circuits: Vec<HashSet<Point3>> = vec![];

    for closest in create_pairs(input).into_iter().take(10) {
        let zero = circuits.iter().position(|c| c.contains(&closest.0));
        let one = circuits.iter().position(|c| c.contains(&closest.1));

        match (zero, one) {
            (Some(i), None) => {
                circuits[i].insert(closest.1);
            }
            (None, Some(i)) => {
                circuits[i].insert(closest.0);
            }
            (Some(a), Some(b)) if a != b => {
                for x in circuits[a].clone() {
                    assert!(circuits[b].insert(x));
                }
                circuits.remove(a);
            }
            (None, None) => {
                circuits.push(HashSet::from([closest.0, closest.1]));
            }
            _ => {}
        }
    }

    println!("{:?}", circuits.iter().map(|c| c.len()).collect_vec());

    circuits
        .into_iter()
        .map(|c| c.len() as u64)
        .k_largest(3)
        .product()
}

fn part2(input: &str) -> u64 {
    let mut circuits: Vec<HashSet<Point3>> = vec![];

    for closest in create_pairs(input) {
        let zero = circuits.iter().position(|c| c.contains(&closest.0));
        let one = circuits.iter().position(|c| c.contains(&closest.1));

        match (zero, one) {
            (Some(i), None) => {
                circuits[i].insert(closest.1);
            }
            (None, Some(i)) => {
                circuits[i].insert(closest.0);
            }
            (Some(a), Some(b)) if a != b => {
                for x in circuits[a].clone() {
                    assert!(circuits[b].insert(x));
                }
                circuits.remove(a);

                if circuits.len() == 1 {
                    return closest.0.x * closest.1.x;
                }
            }
            (None, None) => {
                circuits.push(HashSet::from([closest.0, closest.1]));
            }
            _ => {}
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 81536);
        assert_eq!(part2(input), 7017750530);
    }
}
