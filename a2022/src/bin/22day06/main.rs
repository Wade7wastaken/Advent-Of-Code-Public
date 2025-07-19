use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    (input
        .as_bytes()
        .windows(4)
        .position(|w| w.iter().all_unique())
        .unwrap()
        + 4) as u32
}

fn part2(input: &str) -> u32 {
    (input
        .as_bytes()
        .windows(14)
        .position(|w| w.iter().all_unique())
        .unwrap()
        + 14) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1142);
        assert_eq!(part2(input), 2803);
    }
}
