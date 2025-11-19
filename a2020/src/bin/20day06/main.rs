use std::collections::HashSet;

use lib::StringTools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .paragraphs()
        .map(|p| {
            p.lines()
                .flat_map(|l| l.bytes())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    input
        .paragraphs()
        .map(|p| {
            p.lines()
                .map(|l| l.bytes().collect::<HashSet<_>>())
                .reduce(|acc, x| acc.intersection(&x).copied().collect::<HashSet<_>>())
                .unwrap()
                .len()
        })
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 6351);
        assert_eq!(part2(input), 3143);
    }
}
