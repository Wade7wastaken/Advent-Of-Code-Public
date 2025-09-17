use std::collections::{HashMap, HashSet};

use lib::{IteratorExt, StringTools, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let (replacements, molecule) = input.split_paragraphs_once().unwrap();

    let mut results: HashSet<String> = HashSet::new();

    let mut one_letter: HashMap<u8, Vec<&str>> = HashMap::new();
    let mut two_letter: HashMap<(u8, u8), Vec<&str>> = HashMap::new();

    for (from, to) in replacements.lines().map(|l| l.split_once(" => ").unwrap()) {
        match from.as_bytes() {
            [a] => {
                one_letter.entry(*a).or_default().push(to);
            }
            [a, b] => {
                two_letter.entry((*a, *b)).or_default().push(to);
            }
            _ => panic!(),
        }
    }

    for (i, c) in molecule.bytes().enumerate() {
        if let Some(new_atoms) = one_letter.get(&c) {
            for new_atom in new_atoms {
                let mut res = String::new();
                res += molecule.get(..i).unwrap();
                res += new_atom;
                res += molecule.get(i + 1..).unwrap();
                results.insert(res);
            }
        }
    }

    for ((ia, a), (ib, b)) in molecule.bytes().enumerate().tuple_windows() {
        if let Some(new_atoms) = two_letter.get(&(a, b)) {
            for new_atom in new_atoms {
                let mut res = String::new();
                res += molecule.get(..ia).unwrap();
                res += new_atom;
                res += molecule.get(ib + 1..).unwrap();
                results.insert(res);
            }
        }
    }

    results.len() as u32
}

fn part2(input: &str) -> u32 {
    let (_, elements) = input.split_paragraphs_once().unwrap();
    let element_count = elements.bytes().count_where(|b| b.is_ascii_uppercase()) as u32;
    let rn_ar_count = elements
        .bytes()
        .tuple_windows()
        .count_where(|(a, b)| (a == b'R' && b == b'n') || (a == b'A' && b == b'r'))
        as u32;
    let y_count = elements.bytes().count_where(|b| b == b'Y') as u32;

    element_count - rn_ar_count - 2 * y_count - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 518);
        assert_eq!(part2(input), 200);
    }
}
