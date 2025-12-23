use std::rc::Rc;

use lib::{a_star_score, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Machine {
    lights: u32,
    buttons: Rc<Vec<Vec<usize>>>,
}

fn parse_machine(l: &str) -> Machine {
    let seg = l.split_ascii_whitespace().collect_vec();
    let lights_str = seg[0];
    let buttons_str = &seg[1..seg.len() - 1];

    let mut lights = 0;

    for b in lights_str
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap()
        .bytes()
        .rev()
    {
        lights *= 2;
        if b == b'#' {
            lights += 1;
        }
    }

    let buttons = Rc::new(
        buttons_str
            .iter()
            .map(|b| {
                b.strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect(),
    );

    Machine { lights, buttons }
}

fn neighbors(machine: &Machine) -> Vec<(Machine, u32)> {
    let mut res = vec![];
    for b in machine.buttons.iter() {
        let mut new_state = machine.clone();
        for c in b {
            new_state.lights ^= 1 << *c;
        }
        res.push((new_state, 1));
    }

    res
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    for machine in input.lines().map(parse_machine) {
        res += a_star_score(
            vec![Machine {
                lights: 0,
                buttons: machine.buttons.clone(),
            }],
            |m| m.lights == machine.lights,
            neighbors,
            |_| 0,
        )
        .unwrap();
    }

    res
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
