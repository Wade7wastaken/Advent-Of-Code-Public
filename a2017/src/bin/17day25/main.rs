use std::collections::HashMap;

use lib::{IteratorExt, StringTools, itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
}

struct Action<'a> {
    value: bool,
    offset: i32,
    new_state: &'a str,
}

struct StateBlueprint<'a> {
    zero: Action<'a>,
    one: Action<'a>,
}

fn parse_start(start: &str) -> (&str, i32) {
    let (mut start_state, mut steps) = start
        .lines()
        .map(str::split_ascii_whitespace)
        .collect_tuple()
        .unwrap();
    (
        start_state.nth(3).unwrap().strip_suffix('.').unwrap(),
        steps.nth(5).unwrap().parse().unwrap(),
    )
}

fn parse_action<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Action<'a> {
    let (_, value, offset, new_state) = lines.next_tuple().unwrap();

    Action {
        value: value.split_ascii_whitespace().last().unwrap() == "1.",
        offset: tern!(
            offset.split_ascii_whitespace().last().unwrap() == "right.",
            1,
            -1
        ),
        new_state: new_state
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .strip_suffix('.')
            .unwrap(),
    }
}

fn parse_blueprint(input: &str) -> (&str, StateBlueprint<'_>) {
    let mut lines = input.lines().map(str::trim);
    let state = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .strip_suffix(':')
        .unwrap();

    let zero_state = parse_action(&mut lines);
    let one_state = parse_action(&mut lines);
    (
        state,
        StateBlueprint {
            zero: zero_state,
            one: one_state,
        },
    )
}

struct TuringMachine<'a> {
    tape: HashMap<i32, bool>,
    pos: i32,
    state: &'a str,
}

impl<'a> TuringMachine<'a> {
    fn get_at(&mut self) -> bool {
        *self.tape.get(&self.pos).unwrap_or(&false)
    }
    fn apply_action(&mut self, action: &Action<'a>) {
        self.tape.insert(self.pos, action.value);
        self.pos += action.offset;
        self.state = action.new_state;
    }
}

fn part1(input: &str) -> u32 {
    let mut paragraphs = input.paragraphs();
    let start_str = paragraphs.next().unwrap();
    let (state, steps) = parse_start(start_str);

    let blueprints: HashMap<_, _> = paragraphs.map(parse_blueprint).collect();

    let mut machine = TuringMachine {
        tape: HashMap::new(),
        pos: 0,
        state,
    };

    for _ in 0..steps {
        let blueprint = blueprints.get(machine.state).unwrap();
        let action = tern!(machine.get_at(), &blueprint.one, &blueprint.zero);
        machine.apply_action(action);
    }

    machine.tape.into_values().count_where(|x| x) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 5744);
    }
}
