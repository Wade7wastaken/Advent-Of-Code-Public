use lib::{a_star_score, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Object {
    Generator(&'static str),
    Microchip(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    f1: Vec<Object>,
    f2: Vec<Object>,
    f3: Vec<Object>,
    f4: Vec<Object>,
    elevator: u8,
}

impl State {
    fn floor(&self, f: u8) -> &Vec<Object> {
        match f {
            1 => &self.f1,
            2 => &self.f2,
            3 => &self.f3,
            4 => &self.f4,
            _ => panic!("{f}"),
        }
    }
    fn cur_floor(&self) -> &Vec<Object> {
        self.floor(self.elevator)
    }

    const fn floor_mut(&mut self, f: u8) -> &mut Vec<Object> {
        match f {
            1 => &mut self.f1,
            2 => &mut self.f2,
            3 => &mut self.f3,
            4 => &mut self.f4,
            _ => panic!(),
        }
    }
    const fn cur_floor_mut(&mut self) -> &mut Vec<Object> {
        self.floor_mut(self.elevator)
    }

    fn is_valid(&self) -> bool {
        for i in 1..=4 {
            let floor = self.floor(i);
            if has_rtg(floor) && has_unprotected_chip(floor) {
                return false;
            }
        }
        true
    }
}

fn has_rtg(floor: &[Object]) -> bool {
    floor.iter().any(|it| match it {
        Object::Generator(_) => true,
        Object::Microchip(_) => false,
    })
}

fn has_unprotected_chip(floor: &Vec<Object>) -> bool {
    for it in floor {
        match it {
            Object::Generator(_) => {}
            Object::Microchip(chip) => {
                let has_connected = floor.contains(&Object::Generator(chip));
                if !has_connected {
                    return true;
                }
            }
        }
    }
    false
}

fn parse_floor(l: &'static str) -> Vec<Object> {
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
                (element, "generator") => Object::Generator(element),
                (element, "microchip") => {
                    Object::Microchip(element.strip_suffix("-compatible").unwrap())
                }
                _ => panic!(),
            }
        })
        .sorted()
        .collect()
}

fn neighbors(state: &State) -> Vec<(State, u32)> {
    let mut next = vec![];
    for i in 0..state.cur_floor().len() {
        if state.elevator != 1 {
            let new_elevator = state.elevator - 1;
            let mut new_state = state.clone();
            let removed = new_state.cur_floor_mut().remove(i);
            let next_floor = new_state.floor_mut(new_elevator);
            next_floor.push(removed);
            if new_state.is_valid() {
                let mut cloned = new_state.clone();
                cloned.elevator = new_elevator;
                cloned.cur_floor_mut().sort();
                next.push((cloned, 1));
            }
            for i2 in 0..new_state.cur_floor().len() {
                let mut new_state_2 = new_state.clone();
                let removed2 = new_state_2.cur_floor_mut().remove(i2);
                let next_floor2 = new_state_2.floor_mut(new_elevator);
                next_floor2.push(removed2);
                if new_state_2.is_valid() {
                    let mut cloned = new_state_2.clone();
                    cloned.elevator = new_elevator;
                    cloned.cur_floor_mut().sort();
                    next.push((cloned, 1));
                }
            }
        }
        if state.elevator != 4 {
            let new_elevator = state.elevator + 1;
            let mut new_state = state.clone();
            let removed = new_state.cur_floor_mut().remove(i);
            let next_floor = new_state.floor_mut(new_elevator);
            next_floor.push(removed);
            if new_state.is_valid() {
                let mut cloned = new_state.clone();
                cloned.elevator = new_elevator;
                cloned.cur_floor_mut().sort();
                next.push((cloned, 1));
            }
            for i2 in 0..new_state.cur_floor().len() {
                let mut new_state_2 = new_state.clone();
                let removed2 = new_state_2.cur_floor_mut().remove(i2);
                let next_floor2 = new_state_2.floor_mut(new_elevator);
                next_floor2.push(removed2);
                if new_state_2.is_valid() {
                    let mut cloned = new_state_2.clone();
                    cloned.elevator = new_elevator;
                    cloned.cur_floor_mut().sort();
                    next.push((cloned, 1));
                }
            }
        }
    }

    next
}

fn part1(input: &'static str) -> u32 {
    let (f1, f2, f3, f4) = input.lines().map(parse_floor).collect_tuple().unwrap();
    let total_items = f1.len() + f2.len() + f3.len() + f4.len();
    let start = State {
        f1,
        f2,
        f3,
        f4,
        elevator: 1,
    };

    a_star_score(
        vec![start],
        |state| state.f4.len() == total_items,
        neighbors,
        |state| (total_items - state.f4.len()) as u32 / 2,
    )
    .unwrap()
}

fn part2(input: &'static str) -> u32 {
    let (mut f1, f2, f3, f4) = input.lines().map(parse_floor).collect_tuple().unwrap();
    f1.append(&mut vec![
        Object::Generator("elerium"),
        Object::Microchip("elerium"),
        Object::Generator("dilithium"),
        Object::Microchip("dilithium"),
    ]);
    let total_items = f1.len() + f2.len() + f3.len() + f4.len();
    let start = State {
        f1,
        f2,
        f3,
        f4,
        elevator: 1,
    };

    a_star_score(
        vec![start],
        |state| state.f4.len() == total_items,
        neighbors,
        |state| (total_items - state.f4.len()) as u32 / 2,
    )
    .unwrap()
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
