use std::collections::HashMap;

use lib::{itertools::Itertools, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Attr {
    X,
    M,
    A,
    S,
}

fn parse_attr(attr: char) -> Attr {
    match attr {
        'x' => Attr::X,
        'm' => Attr::M,
        'a' => Attr::A,
        's' => Attr::S,
        _ => unreachable!("unknown attr: {}", attr),
    }
}

#[derive(Debug)]
struct Rule<'a> {
    attr: Attr,
    greater_than: bool,
    value: u32,
    dest: &'a str,
}

fn parse_rule(rule: &str) -> Rule {
    let (attr, cond) = rule[..2].chars().collect_tuple().unwrap();
    let (value_str, dest) = rule[2..].split_once(':').unwrap();

    Rule {
        attr: parse_attr(attr),
        greater_than: cond == '>',
        value: value_str.parse().unwrap(),
        dest,
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    end_location: &'a str,
}

fn parse_workflow(input: &str) -> (&str, Workflow) {
    let (name, rules_str) = input.split_once('{').unwrap();
    let mut rules_iter = rules_str.strip_suffix('}').unwrap().split(',');
    let end_location = rules_iter.next_back().unwrap();
    let rules = rules_iter.map(parse_rule).collect();

    (
        name,
        Workflow {
            rules,
            end_location,
        },
    )
}

#[derive(Debug, Default)]
struct Part(HashMap<Attr, u32>);

fn parse_part(input: &str) -> Part {
    let map = input
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .map(|part_str| {
            let attr = parse_attr(part_str.chars().next().unwrap());
            let value = part_str.get(2..).unwrap().parse().unwrap();
            (attr, value)
        })
        .collect();
    Part(map)
}

fn process_part_step<'a>(workflow: &'a Workflow<'a>, part: &Part) -> &'a str {
    workflow
        .rules
        .iter()
        .find(|rule| {
            let part_value = *part.0.get(&rule.attr).unwrap();
            (rule.greater_than && part_value > rule.value)
                || (!rule.greater_than && part_value < rule.value)
        })
        .map_or(workflow.end_location, |rule| rule.dest)
}

fn process_part(part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut cur = "in";

    loop {
        let workflow = workflows.get(cur).unwrap();
        cur = process_part_step(workflow, part);
        if cur == "R" {
            return false;
        }
        if cur == "A" {
            return true;
        }
    }
}

fn part1(input: &str) -> u32 {
    let (workflows_str, parts_str) = input.paragraphs().collect_tuple().unwrap();
    let workflows = workflows_str
        .lines()
        .map(parse_workflow)
        .collect::<HashMap<_, _>>();

    parts_str
        .lines()
        .map(parse_part)
        .filter(|part| process_part(part, &workflows))
        .map(|part| part.0.into_values().sum::<u32>())
        .sum()
}

// fn part2(input: &str) -> u32 {
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 377025);
        // assert_eq!(part2(input), todo!());
    }
}
