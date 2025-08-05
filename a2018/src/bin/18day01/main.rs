use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .sum()
}

struct Seq<I: Iterator<Item = i32>>(I, i32);

impl<I: Iterator<Item = i32>> Iterator for Seq<I> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.0.next()?;
        self.1 += n;
        Some(self.1)
    }
}

fn part2(input: &str) -> i32 {
    Seq(
        input
            .lines()
            .map(|l| l.trim().parse::<i32>().unwrap())
            .cycle(),
        0,
    )
    .duplicates()
    .next()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 518);
        assert_eq!(part2(input), 72889);
    }
}
