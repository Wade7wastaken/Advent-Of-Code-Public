use std::collections::{HashMap, HashSet};

use lib::{DigitIter, Grid, Vec2, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    part1(input);
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();

    let mut current_number = DigitIter::default();
    let mut included = false;
    let mut sum = 0;

    for (p, c) in grid.enumerate() {
        if let Some(digit) = c.to_digit(10) {
            current_number.add_right(digit);
            let has_symbol = grid
                .with_offsets(p, Vec2::SURROUNDING)
                .values()
                .any(|c| !c.is_numeric() && *c != '.');
            if has_symbol {
                included = true;
            }
        } else {
            if included {
                sum += current_number.value();
            }
            current_number.clear();
            included = false;
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_chars_transpose(input).unwrap();

    let mut current_number = DigitIter::default();

    let mut gear_pairs = HashMap::new();
    let mut encountered_stars = HashSet::new();

    for (p, c) in grid.enumerate() {
        if let Some(digit) = c.to_digit(10) {
            current_number.add_right(digit);
            for (q, c) in grid.with_offsets(p, Vec2::SURROUNDING).enumerate() {
                if *c == '*' {
                    encountered_stars.insert(q);
                }
            }
        } else {
            for star in &encountered_stars {
                gear_pairs
                    .entry(*star)
                    .or_insert(vec![])
                    .push(current_number.value());
            }

            current_number.clear();
            encountered_stars.clear();
        }
    }

    gear_pairs
        .into_values()
        .filter_map(|b| b.into_iter().collect_tuple())
        .map(|(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 559667);
        assert_eq!(part2(input), 86841457);
    }
}
