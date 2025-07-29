use std::collections::HashMap;

use lib::{Dir, Entity, Grid, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();
    let width = grid.width() as i32;
    let height = grid.width() as i32;
    let mut map = HashMap::new();
    for (p, _) in grid.into_enumerate().filter(|(_, c)| *c == '#') {
        map.insert(p.map(|d| d as i32), true);
    }

    let mut en = Entity::new((width / 2, height / 2), Dir::North);

    let mut caused_infection = 0;

    for _ in 0..10000 {
        let c = map.entry(en.into()).or_insert(false);
        en = tern!(*c, en.turn_right(), en.turn_left());

        *c = !*c;

        if *c {
            caused_infection += 1;
        }

        en = en.step().unwrap();
    }

    caused_infection
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State {
    fn advance(&mut self) {
        *self = match *self {
            State::Clean => State::Weakened,
            State::Weakened => State::Infected,
            State::Infected => State::Flagged,
            State::Flagged => State::Clean,
        }
    }
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();
    let width = grid.width() as i32;
    let height = grid.width() as i32;
    let mut map = HashMap::new();
    for (p, _) in grid.into_enumerate().filter(|(_, c)| *c == '#') {
        map.insert(p.map(|d| d as i32), State::Infected);
    }

    let mut en = Entity::new((width / 2, height / 2), Dir::North);

    let mut caused_infection = 0;

    for _ in 0..10000000 {
        let c = map.entry(en.into()).or_insert(State::Clean);
        en = match *c {
            State::Clean => en.turn_left(),
            State::Weakened => en,
            State::Infected => en.turn_right(),
            State::Flagged => en.reverse(),
        };

        c.advance();

        if *c == State::Infected {
            caused_infection += 1;
        }

        en = en.step().unwrap();
    }

    caused_infection
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 5352);
        assert_eq!(part2(input), 2511475);
    }
}
