use lib::{abs_diff, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn react(polymer: &mut [u8]) -> usize {
    let mut i = 0;
    let mut j = 1;
    let mut removed = 0;
    while j < polymer.len() {
        assert!(!(polymer[i] == 0 || polymer[j] == 0));
        if abs_diff(polymer[i], polymer[j]) == b'a' - b'A' {
            polymer[i] = 0;
            polymer[j] = 0;

            while i != 0 && polymer[i] == 0 {
                i -= 1;
            }
            j += 1;
            if i == 0 && polymer[i] == 0 {
                i = j;
                j += 1;
            }

            removed += 2;
        } else {
            i = j;
            j += 1;
        }
    }
    polymer.len() - removed
}

fn part1(input: &str) -> u32 {
    react(&mut input.bytes().collect_vec()) as u32
}

fn part2(input: &str) -> u32 {
    let mut bytes = input.bytes().collect_vec();

    let _ = react(&mut bytes);

    bytes.retain(|x| *x != 0);

    (0..26)
        .map(|polymer| {
            let mut new_bytes = bytes.clone();
            new_bytes.retain(|x| *x != polymer + b'a' && *x != polymer + b'A');
            react(&mut new_bytes)
        })
        .min()
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 11264);
        assert_eq!(part2(input), 4552);
    }
}
