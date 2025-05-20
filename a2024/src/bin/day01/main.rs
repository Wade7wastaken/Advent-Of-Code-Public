use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn part1(input: &str) -> u32 {
    let (list_a, list_b) = parse_lists(input);
    list_a
        .into_iter()
        .sorted_unstable()
        .zip(list_b.into_iter().sorted_unstable())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part2(input: &str) -> u32 {
    let (list_a, list_b) = parse_lists(input);

    let map = list_b.into_iter().counts();

    list_a
        .into_iter()
        .map(|n| n * map.get(&n).copied().unwrap_or(0) as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 3569916);
        assert_eq!(part2(input), 26407426);
    }
}
