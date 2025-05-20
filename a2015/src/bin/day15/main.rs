#![allow(clippy::many_single_char_names)]

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_ingredient(input: &str) -> Ingredient {
    let (capacity, durability, flavor, texture, calories) = input
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|prop| prop.split_once(' ').unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Ingredient {
        capacity,
        durability,
        flavor,
        texture,
        calories,
    }
}

fn part1(input: &str) -> u32 {
    let (a, b, c, d) = input.lines().map(parse_ingredient).collect_tuple().unwrap();
    let mut max = 0;
    for w in 0..100 {
        for x in 0..(100 - w) {
            for y in 0..(100 - w - x) {
                let z = 100 - w - x - y;
                if z < 0 {
                    continue;
                }
                let capacity = w * a.capacity + x * b.capacity + y * c.capacity + z * d.capacity;
                let durability =
                    w * a.durability + x * b.durability + y * c.durability + z * d.durability;
                let flavor = w * a.flavor + x * b.flavor + y * c.flavor + z * d.flavor;
                let texture = w * a.texture + x * b.texture + y * c.texture + z * d.texture;
                if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                    continue;
                }
                let score = capacity * durability * flavor * texture;
                if score > max {
                    max = score;
                }
            }
        }
    }
    max as u32
}

fn part2(input: &str) -> u32 {
    let (a, b, c, d) = input.lines().map(parse_ingredient).collect_tuple().unwrap();
    let mut max = 0;
    for w in 0..100 {
        for x in 0..(100 - w) {
            for y in 0..(100 - w - x) {
                let z = 100 - w - x - y;
                if z < 0 {
                    continue;
                }
                let calories = w * a.calories + x * b.calories + y * c.calories + z * d.calories;
                if calories != 500 {
                    continue;
                }
                let capacity = w * a.capacity + x * b.capacity + y * c.capacity + z * d.capacity;
                let durability =
                    w * a.durability + x * b.durability + y * c.durability + z * d.durability;
                let flavor = w * a.flavor + x * b.flavor + y * c.flavor + z * d.flavor;
                let texture = w * a.texture + x * b.texture + y * c.texture + z * d.texture;
                if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                    continue;
                }
                let score = capacity * durability * flavor * texture;
                if score > max {
                    max = score;
                }
            }
        }
    }
    max as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 222870);
        assert_eq!(part2(input), 117936);
    }
}
