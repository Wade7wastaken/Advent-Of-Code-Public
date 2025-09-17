use lib::{IteratorExt, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.lines().count_where(|l| {
        let mut twice = false;
        for (a, b) in l.bytes().tuple_windows() {
            if matches!(
                (a, b),
                (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y')
            ) {
                return false;
            }
            if a == b {
                twice = true;
            }
        }
        twice
            && l.bytes()
                .count_where(|c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
                >= 3
    }) as u32
}

fn part2(input: &str) -> u32 {
    input.lines().count_where(|l| {
        l.chars().tuple_windows().any(|(a, _, c)| a == c)
            && l.chars().tuple_windows().enumerate().any(|(i, (a, b))| {
                l.chars()
                    .tuple_windows()
                    .skip(i + 2)
                    .any(|(x, y)| x == a && y == b)
            })
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
