use std::collections::{HashMap, VecDeque};

use lib::{itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

#[derive(Debug, Default)]
struct FlipFlop {
    state: bool,
}

#[derive(Debug, Default)]
struct Conjunction<'a> {
    inputs: HashMap<&'a str, bool>,
}

trait Module<'a> {
    fn receive(&mut self, pulse: bool, from: &'a str) -> Option<bool>;
}

impl Module<'_> for FlipFlop {
    fn receive(&mut self, pulse: bool, _from: &str) -> Option<bool> {
        (!pulse).then(|| {
            self.state = !self.state;
            tern!(self.state, true, false)
        })
    }
}

impl<'a> Module<'a> for Conjunction<'a> {
    fn receive(&mut self, pulse: bool, from: &'a str) -> Option<bool> {
        self.inputs.insert(from, pulse);
        Some(!self.inputs.values().all(|v| *v))
    }
}

fn part1(input: &str) -> u32 {
    let mut flip_flops: HashMap<&str, FlipFlop> = HashMap::new();
    let mut conjunctions: HashMap<&str, Conjunction> = HashMap::new();
    let mut parents: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut children = HashMap::new();
    let mut starts = vec![];
    for l in input.lines() {
        let (src, dest) = l.split_once(" -> ").unwrap();
        if let Some(name) = src.strip_prefix('%') {
            flip_flops.insert(name, FlipFlop::default());
            let child = dest.split(", ").collect_vec();
            for c in &child {
                parents.entry(c).or_default().push(name);
            }
            children.insert(name, child);
        } else if let Some(name) = src.strip_prefix('&') {
            conjunctions.insert(name, Conjunction::default());
            let child = dest.split(", ").collect_vec();
            for c in &child {
                parents.entry(c).or_default().push(name);
            }
            children.insert(name, child);
        } else if src == "broadcaster" {
            assert_eq!(starts.len(), 0);
            starts = dest
                .split(", ")
                .map(|name| (name, false, "broadcaster"))
                .collect_vec();
        } else {
            panic!();
        }
    }

    for (name, conjunction) in &mut conjunctions {
        let parent = parents.get(name).unwrap();
        for p in parent {
            conjunction.inputs.insert(p, false);
        }
    }

    let mut combined: HashMap<&str, &mut dyn Module> = HashMap::new();
    for (name, flip_flop) in &mut flip_flops {
        combined.insert(name, flip_flop);
    }
    for (name, conjunction) in &mut conjunctions {
        combined.insert(name, conjunction);
    }

    let mut low_sent = 0;
    let mut high_sent = 0;

    for _ in 0..1000 {
        low_sent += 1;
        let mut pulses: VecDeque<_> = starts.clone().into();

        while let Some((next, pulse, from)) = pulses.pop_front() {
            if pulse {
                high_sent += 1;
            } else {
                low_sent += 1;
            }

            if let Some(module) = combined.remove(next) {
                let output = module.receive(pulse, from);
                if let Some(sent_pulse) = output {
                    for c in children.get(next).unwrap() {
                        pulses.push_back((c, sent_pulse, next));
                    }
                }
                combined.insert(next, module);
            }
        }
    }

    low_sent * high_sent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 856482136);
        // assert_eq!(part2(input), todo!());
    }
}
