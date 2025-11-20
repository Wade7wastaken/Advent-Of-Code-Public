use std::collections::{HashMap, HashSet};

use lib::{IteratorExt, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let valid = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input.paragraphs().count_where(|p| {
        let mut fields = HashSet::new();
        for field in p.split_ascii_whitespace() {
            fields.insert(field.split_once(':').unwrap().0);
        }
        valid.iter().all(|field| fields.contains(field))
    }) as u32
}

fn validate_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => value
            .parse::<u32>()
            .ok()
            .is_some_and(|y| (1920..=2002).contains(&y)),
        "iyr" => value
            .parse::<u32>()
            .ok()
            .is_some_and(|y| (2010..=2020).contains(&y)),
        "eyr" => value
            .parse::<u32>()
            .ok()
            .is_some_and(|y| (2020..=2030).contains(&y)),
        "hgt" => {
            if let Some(inches) = value.strip_suffix("in") {
                inches
                    .parse::<u32>()
                    .ok()
                    .is_some_and(|inches| (59..=76).contains(&inches))
            } else if let Some(centimeters) = value.strip_suffix("cm") {
                centimeters
                    .parse::<u32>()
                    .ok()
                    .is_some_and(|centimeters| (150..=193).contains(&centimeters))
            } else {
                false
            }
        }
        "hcl" => value
            .strip_prefix('#')
            .is_some_and(|color| color.bytes().all(|b| b.is_ascii_hexdigit())),
        "ecl" => {
            matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
        }
        "pid" => value.len() == 9 && value.bytes().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => panic!(),
    }
}

fn part2(input: &str) -> u32 {
    let valid = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input.paragraphs().count_where(|p| {
        let fields = p
            .split_ascii_whitespace()
            .map(|field| field.split_once(':').unwrap())
            .collect::<HashMap<_, _>>();
        valid.iter().all(|field| fields.contains_key(field))
            && fields.iter().all(|(key, value)| validate_field(key, value))
    }) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 245);
        assert_eq!(part2(input), 133);
    }
}
