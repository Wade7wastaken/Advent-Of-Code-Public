use lib::{StringTools, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .paragraphs()
        .map(|elf| elf.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    input
        .paragraphs()
        .map(|elf| elf.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .k_largest(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 68923);
        assert_eq!(part2(input), 200044);
    }
}
