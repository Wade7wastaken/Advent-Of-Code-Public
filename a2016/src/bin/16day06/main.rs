use std::cmp::Reverse;

use lib::{
    CollectString, Grid,
    itertools::{Either, Itertools},
    tern,
};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn solve(input: &str, reversed: bool) -> String {
    Grid::from_bytes_transpose(input)
        .unwrap()
        .into_cols()
        .into_iter()
        .map(|col| {
            col.into_iter()
                .counts()
                .into_iter()
                .max_by_key(|x| tern!(reversed, Either::Left(Reverse(x.1)), Either::Right(x.1)))
                .unwrap()
                .0
        })
        .collect_string()
}

fn part1(input: &str) -> String {
    solve(input, false)
}

fn part2(input: &str) -> String {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "dzqckwsd");
        assert_eq!(part2(input), "lragovly");
    }
}
