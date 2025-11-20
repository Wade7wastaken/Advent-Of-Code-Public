use std::collections::HashMap;

use lib::{Inline, a_star_score, defer, itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_floor(l: &str) -> Vec<(&str, bool)> {
    if l.ends_with("relevant.") {
        return vec![];
    }

    let start = l.find("contains").unwrap();

    l[start + 9..]
        .strip_suffix('.')
        .unwrap()
        .split(", ")
        .map(|it| {
            match it
                .strip_prefix("a ")
                .or_else(|| it.strip_prefix("and a "))
                .unwrap()
                .split_ascii_whitespace()
                .collect_tuple()
                .unwrap()
            {
                (element, "generator") => (element, true),
                (element, "microchip") => (element.strip_suffix("-compatible").unwrap(), false),
                _ => panic!(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    elevator: u8,
    pairs: Vec<(u8, u8)>,
}

impl State {
    const fn new(pairs: Vec<(u8, u8)>) -> Self {
        Self { elevator: 0, pairs }
    }

    fn iter(&self) -> impl Iterator<Item = u8> {
        self.pairs.iter().flat_map(|(a, b)| [*a, *b])
    }

    fn get_by_index_mut(&mut self, i: usize) -> &mut u8 {
        let pair = self.pairs.get_mut(i / 2).unwrap();
        tern!(i.is_multiple_of(2), &mut pair.0, &mut pair.1)
    }
}

fn neighbors(state: &State) -> Vec<(State, u32)> {
    let mut output = vec![];

    let mut add_neighbor = |state: State| {
        if is_valid(&state) {
            output.push((state.inline(|v| v.pairs.sort_unstable()), 1));
        }
    };

    let indices = state
        .iter()
        .enumerate()
        .filter_map(|(i, x)| (x == state.elevator).then_some(i))
        .collect_vec();

    for (i1, i2) in indices.iter().copied().tuple_combinations() {
        if state.elevator != 3 {
            let mut cloned = state.clone();

            *cloned.get_by_index_mut(i1) += 1;
            *cloned.get_by_index_mut(i2) += 1;
            cloned.elevator += 1;

            add_neighbor(cloned);
        }
        if state.elevator != 0 {
            let mut cloned = state.clone();

            *cloned.get_by_index_mut(i1) -= 1;
            *cloned.get_by_index_mut(i2) -= 1;
            cloned.elevator -= 1;

            add_neighbor(cloned);
        }
    }

    for obj1 in indices {
        if state.elevator != 3 {
            let mut cloned = state.clone();

            *cloned.get_by_index_mut(obj1) += 1;
            cloned.elevator += 1;

            add_neighbor(cloned);
        }
        if state.elevator != 0 {
            let mut cloned = state.clone();

            *cloned.get_by_index_mut(obj1) -= 1;
            cloned.elevator -= 1;

            add_neighbor(cloned);
        }
    }

    output
}

fn is_valid(state: &State) -> bool {
    let mut f_ok = [true; 4];
    for s in &state.pairs {
        f_ok[s.0 as usize] = false;
    }

    state
        .pairs
        .iter()
        .filter(|(a, b)| a != b)
        .all(|s| f_ok[s.1 as usize])
}

fn starting_pairs(input: &str) -> Vec<(u8, u8)> {
    let mut map: HashMap<i32, (Option<u8>, Option<u8>)> = HashMap::new();

    let mut name_map = HashMap::new();
    let mut next_id = 0;

    for (floor, objects) in input.lines().map(parse_floor).enumerate() {
        for (name, obj) in objects {
            let id = *name_map
                .entry(name)
                .or_insert(defer!(next_id; next_id += 1));

            let pair = map.entry(id).or_default();
            *tern!(obj, &mut pair.0, &mut pair.1) = Some(floor as u8);
        }
    }

    map.into_values()
        .map(|(a, b)| (a.unwrap(), b.unwrap()))
        .collect_vec()
        .inline(|v| v.sort_unstable())
}

fn all_on_top(state: &State) -> bool {
    state.pairs.iter().all(|(a, b)| *a == 3 && *b == 3)
}

fn heuristic(state: &State) -> u32 {
    state
        .pairs
        .iter()
        .map(|(a, b)| u32::from(6 - (a + b)))
        .sum::<u32>()
        / 2
}

fn part1(input: &str) -> u32 {
    let pairs = starting_pairs(input);

    let s = State::new(pairs);

    a_star_score(vec![s], all_on_top, neighbors, heuristic).unwrap()
}

fn part2(input: &'static str) -> u32 {
    let mut pairs = starting_pairs(input);

    pairs.extend([(0, 0); 2]);

    let s = State::new(pairs);

    a_star_score(vec![s], all_on_top, neighbors, heuristic).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 47);
        assert_eq!(part2(input), 71);
    }
}
