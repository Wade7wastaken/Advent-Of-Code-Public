use std::collections::{HashMap, HashSet};

use lib::{Grid, Surround};

fn main() {
    let input = include_str!("./input.txt").trim();
    part1(input);
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();

    let mut current_number = 0;
    let mut included = false;
    let mut sum = 0;

    for (p, c) in grid.enumerate() {
        if c.is_numeric() {
            current_number *= 10;
            current_number += c.to_digit(10).unwrap();
            let has_symbol = grid
                .surrounding(p, Surround::All)
                .values()
                .any(|c| !c.is_numeric() && *c != '.');
            if has_symbol {
                included = true;
            }
        } else {
            if included {
                sum += current_number;
            }
            current_number = 0;
            included = false;
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();

    let mut current_number = 0;

    let mut gear_pairs = HashMap::new();
    let mut encountered_stars = HashSet::new();

    for (p, c) in grid.enumerate() {
        if let Some(digit) = c.to_digit(10) {
            current_number *= 10;
            current_number += digit;
            for (q, _, c) in grid.surrounding(p, Surround::All) {
                if *c == '*' {
                    encountered_stars.insert(q);
                }
            }
        } else {
            for star in &encountered_stars {
                gear_pairs
                    .entry(*star)
                    .or_insert(vec![])
                    .push(current_number);
            }

            current_number = 0;
            encountered_stars.clear();
        }
    }

    gear_pairs
        .into_values()
        .filter(|b| b.len() == 2)
        .map(|b| b.into_iter().product::<u32>())
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
