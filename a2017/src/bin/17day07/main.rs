use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

fn part1(input: &str) -> String {
    let mut holding_up = HashSet::new();
    let mut held_up = HashSet::new();
    for l in input.lines() {
        if let Some((info, children_str)) = l.split_once(" -> ") {
            holding_up.insert(info.split_once(' ').unwrap().0);
            for child in children_str.split(',').map(|c| c.trim()) {
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
