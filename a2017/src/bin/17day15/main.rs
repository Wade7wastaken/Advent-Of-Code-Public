use lib::{CountWhere, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

const MOD: u64 = 2147483647;
const A_MULT: u64 = 16807;
const B_MULT: u64 = 48271;

fn generators(input: &str) -> (impl Iterator<Item = u64>, impl Iterator<Item = u64>) {
    let (a_state, b_state) = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .collect_tuple::<(_, _, _, _, _)>()
                .unwrap()
                .4
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let a_iter = std::iter::successors(Some(a_state), |state| Some((state * A_MULT) % MOD)).skip(1);
    let b_iter = std::iter::successors(Some(b_state), |state| Some((state * B_MULT) % MOD)).skip(1);

    (a_iter, b_iter)
}

fn part1(input: &str) -> u32 {
    let (a_iter, b_iter) = generators(input);

    a_iter
        .zip(b_iter)
        .take(40_000_000)
        .count_where(|(a, b)| a & 0xFFFF == b & 0xFFFF) as u32
}

fn part2(input: &str) -> u32 {
    let (a_iter, b_iter) = generators(input);

    a_iter
        .filter(|x| x % 4 == 0)
        .zip(b_iter.filter(|x| x % 8 == 0))
        .take(5_000_000)
        .count_where(|(a, b)| a & 0xFFFF == b & 0xFFFF) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 567);
        assert_eq!(part2(input), 323);
    }
}
