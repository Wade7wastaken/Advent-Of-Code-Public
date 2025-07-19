use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn hash(input: &str) -> u32 {
    input
        .bytes()
        .fold(0, |acc, c| ((acc + u32::from(c)) * 17) % 256)
}

fn part1(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

enum Action {
    Add(u32), // focal length
    Remove,
}

struct Step<'a> {
    label: &'a str,
    action: Action,
}

fn parse_step(s: &str) -> Step<'_> {
    let last = s.as_bytes().last().unwrap();
    let (action, label) = if last.is_ascii_digit() {
        (
            Action::Add(u32::from(last - b'0')),
            s.get(0..s.len() - 2).unwrap(),
        )
    } else {
        (Action::Remove, s.get(0..s.len() - 1).unwrap())
    };
    Step { label, action }
}

struct Lens<'a> {
    label: &'a str,
    focal_len: u32,
}

fn part2(input: &str) -> u32 {
    let mut map: HashMap<u32, Vec<Lens>> = HashMap::new();

    for Step { label, action } in input.split(',').map(parse_step) {
        let box_num = hash(label);
        match action {
            Action::Add(focal_len) => {
                let lenses = map.entry(box_num).or_default();
                let new_lens = Lens { label, focal_len };
                if let Some(i) = lenses.iter().position(|lens| lens.label == label) {
                    lenses[i] = new_lens;
                } else {
                    lenses.push(new_lens);
                }
            }
            Action::Remove => {
                if let Some(lenses) = map.get_mut(&box_num) {
                    if let Some(i) = lenses.iter().position(|lens| lens.label == label) {
                        lenses.remove(i);
                    }
                }
            }
        }
    }

    map.into_iter()
        .flat_map(|(box_num, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(i, lens)| (box_num + 1) * (i as u32 + 1) * lens.focal_len)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 519041);
        assert_eq!(part2(input), 260530);
    }
}
