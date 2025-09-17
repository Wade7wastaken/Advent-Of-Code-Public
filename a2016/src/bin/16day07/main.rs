use std::collections::HashSet;

use lib::{
    IteratorExt, SwapIf,
    itertools::{Either, Itertools},
    tern,
};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn contains_abba(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a != b && a == d && b == c)
}

fn partition(l: &str) -> (Vec<&str>, Vec<&str>) {
    l.split(['[', ']'])
        .enumerate()
        .partition_map(|(i, x)| tern!(i % 2 == 0, Either::Left, Either::Right)(x))
}

fn supports_tls(l: &str) -> bool {
    let (outside, inside) = partition(l);
    outside.into_iter().any(contains_abba) && !inside.into_iter().any(contains_abba)
}

fn part1(input: &str) -> u32 {
    input.lines().count_where(supports_tls) as u32
}

fn find_abas(v: Vec<&str>, swap: bool) -> HashSet<(char, char)> {
    v.into_iter()
        .flat_map(|x| {
            x.chars()
                .tuple_windows()
                .filter(|(a, b, c)| a != b && a == c)
                .map(|(a, b, _)| (a, b).swap_if(swap))
        })
        .collect::<HashSet<_>>()
}

fn supports_ssl(l: &str) -> bool {
    let (outside, inside) = partition(l);
    let abas = find_abas(outside, false);
    let babs = find_abas(inside, true);
    abas.intersection(&babs).next().is_some()
}

fn part2(input: &str) -> u32 {
    input.lines().count_where(supports_ssl) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 118);
        assert_eq!(part2(input), 260);
    }
}
