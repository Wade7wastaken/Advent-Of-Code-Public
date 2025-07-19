use std::collections::HashSet;

use lib::{Dir, Entity, Point2, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .split(", ")
        .map(|inst| inst.split_at(1))
        .fold(Entity::new((0i32, 0i32), Dir::North), |en, inst| {
            tern!(inst.0 == "R", en.turn_right(), en.turn_left())
                .step_n(inst.1.parse().unwrap())
                .unwrap()
        })
        .pos()
        .manhattan_dist(Point2::new(0, 0)) as u32
}

fn part2(input: &str) -> u32 {
    let mut en = Entity::new((0i32, 0i32), Dir::North);
    let mut visited = HashSet::new();
    visited.insert(en.pos());
    for inst in input.split(", ").map(|inst| inst.split_at(1)) {
        en = tern!(inst.0 == "R", en.turn_right(), en.turn_left());
        for _ in 0..inst.1.parse().unwrap() {
            en = en.step().unwrap();
            if !visited.insert(en.pos()) {
                return en.pos().manhattan_dist(Point2::new(0, 0)) as u32;
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 273);
        assert_eq!(part2(input), 115);
    }
}
