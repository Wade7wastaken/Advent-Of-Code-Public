use std::collections::HashMap;

use lib::{Dir, Entity, Vec2, abs_diff, point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let data = input.parse::<u32>().unwrap();
    let size = f64::from(data).sqrt().ceil() as u32;
    let center = (f64::from(size - 1) / 2.0).ceil() as u32;
    center - 1 + abs_diff(center, data % size)
}

fn part2(input: &str) -> u32 {
    let target = input.parse().unwrap();
    let mut grid = HashMap::new();
    grid.insert(point2(0, 0), 1);
    let mut en = Entity::new(point2(0, 0), Dir::East);
    let mut step = 1;
    loop {
        for _ in 0..2 {
            for _ in 0..step {
                en = en.step().unwrap();

                let value = Vec2::SURROUNDING
                    .into_iter()
                    .filter_map(|dir| grid.get(&en.pos().apply(dir).unwrap()))
                    .sum();

                if value > target {
                    return value;
                }

                grid.insert(en.pos(), value);
            }
            en = en.turn_left();
        }
        step += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 419);
        assert_eq!(part2(input), 295229);
    }
}
