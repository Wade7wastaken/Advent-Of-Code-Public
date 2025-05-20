use lib::{CountWhere, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.lines().count_where(|l| {
        let vowels = l
            .chars()
            .count_where(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'));
        let mut twice = false;
        for (a, b) in l.chars().tuple_windows() {
            if a == b {
                twice = true;
            }
            if matches!((a, b), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')) {
                return false;
            }
        }
        vowels >= 3 && twice
    }) as u32
}

fn part2(input: &str) -> u32 {
    input.lines().count_where(|l| {
        let mut pairs = false;
        let mut three = false;
        for (i, (a, b)) in l.chars().tuple_windows().enumerate() {
            if l.chars()
                .tuple_windows()
                .skip(i + 2)
                .any(|(x, y)| x == a && y == b)
            {
                pairs = true;
            }
        }
        for (a, _, c) in l.chars().tuple_windows() {
            if a == c {
                three = true;
            }
        }
        pairs && three
    }) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 236);
        assert_eq!(part2(input), 51);
    }
}
