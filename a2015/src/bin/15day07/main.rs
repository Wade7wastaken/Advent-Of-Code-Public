use std::collections::HashMap;

use lib::{Swap, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn wire_value<'a>(
    wire_map: &HashMap<&str, &'a str>,
    wire: &'a str,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(cached) = cache.get(&wire) {
        return *cached;
    }
    let expr = wire_map.get(wire).unwrap();
    let mut eval = |s: &'a str| s.parse().unwrap_or_else(|_| wire_value(wire_map, s, cache));
    let wire_value = if let Some((a, b)) = expr.split_once(" AND ") {
        eval(a) & eval(b)
    } else if let Some((a, b)) = expr.split_once(" LSHIFT ") {
        eval(a) << eval(b)
    } else if let Some(a) = expr.strip_prefix("NOT ") {
        !eval(a)
    } else if let Some((a, b)) = expr.split_once(" OR ") {
        eval(a) | eval(b)
    } else if let Some((a, b)) = expr.split_once(" RSHIFT ") {
        eval(a) >> eval(b)
    } else {
        eval(expr)
    };
    cache.insert(wire, wire_value);
    wire_value
}

fn build_wire_map(input: &str) -> HashMap<&str, &str> {
    input
        .lines()
        .map(|l| l.split(" -> ").collect_tuple::<(_, _)>().unwrap().swap())
        .collect()
}

fn part1(input: &str) -> u16 {
    let wire_map = build_wire_map(input);

    let mut cache = HashMap::new();
    wire_value(&wire_map, "a", &mut cache)
}

fn part2(input: &str) -> u16 {
    let wire_map = build_wire_map(input);

    let mut cache = HashMap::new();

    let orig_a = wire_value(&wire_map, "a", &mut cache);

    cache.clear();
    cache.insert("b", orig_a);

    wire_value(&wire_map, "a", &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 956);
        assert_eq!(part2(input), 40149);
    }
}
