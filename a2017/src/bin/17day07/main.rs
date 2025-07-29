use std::collections::{HashMap, HashSet};

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> String {
    let mut holding_up = HashSet::new();
    let mut held_up = HashSet::new();
    for l in input.lines() {
        if let Some((info, children_str)) = l.split_once(" -> ") {
            holding_up.insert(info.split_once(' ').unwrap().0);
            for child in children_str.split(',').map(str::trim) {
                held_up.insert(child);
            }
        }
    }
    for h in holding_up {
        if !held_up.contains(h) {
            return h.to_string();
        }
    }

    panic!();
}

fn weight_of(start_name: &str, programs: &HashMap<&str, Program>) -> Result<u32, u32> {
    let start_program = programs.get(&start_name).expect(start_name);

    let mut weight = start_program.weight;

    if start_program.children.is_empty() {
        return Ok(weight);
    }

    let mut child_weights = vec![];

    for child in &start_program.children {
        let child_weight = weight_of(child, programs)?;
        child_weights.push((child_weight, child));
        weight += child_weight;
    }

    if !child_weights.iter().map(|x| x.0).all_equal() {
        assert!(child_weights.len() >= 3);
        let common = child_weights
            .iter()
            .tuple_combinations()
            .find(|(a, b)| a.0 == b.0)
            .unwrap()
            .0;
        let outlier = child_weights.iter().find(|a| a.0 != common.0).unwrap();
        let diff = common.0 as i32 - outlier.0 as i32;
        let orig_weight = programs.get(outlier.1).unwrap().weight;
        return Err((orig_weight as i32 + diff) as u32);
    }

    Ok(weight)
}

struct Program<'a> {
    weight: u32,
    children: Vec<&'a str>,
}

fn determine_first<'a>(holding_up: HashSet<&'a str>, held_up: HashSet<&'a str>) -> &'a str {
    for h in holding_up {
        if !held_up.contains(h) {
            return h;
        }
    }
    drop(held_up);

    panic!();
}

fn part2(input: &str) -> u32 {
    let mut programs = HashMap::new();
    let mut holding_up = HashSet::new();
    let mut held_up = HashSet::new();
    for l in input.lines() {
        if let Some((info, children_str)) = l.split_once(" -> ") {
            let (name, weight_str) = info.split_once(' ').unwrap();
            holding_up.insert(name);
            let mut children = vec![];
            for child in children_str.split(',').map(str::trim) {
                held_up.insert(child);
                children.push(child);
            }
            programs.insert(
                name,
                Program {
                    weight: weight_str
                        .trim()
                        .strip_prefix('(')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .parse()
                        .unwrap(),
                    children,
                },
            );
        } else {
            let (name, weight_str) = l.split_once(' ').unwrap();
            programs.insert(
                name,
                Program {
                    weight: weight_str
                        .trim()
                        .strip_prefix('(')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .parse()
                        .unwrap(),
                    children: vec![],
                },
            );
        }
    }
    let first = determine_first(holding_up, held_up);
    weight_of(first, &programs).unwrap_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "vtzay");
        assert_eq!(part2(input), 910);
    }
}
