use std::collections::HashMap;

use lib::{itertools::Itertools, Swap};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn wire_value(
    map: &HashMap<String, String>,
    wire: String,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(cached) = cache.get(&wire) {
        return *cached;
    }
    let src_str = map.get(&wire).unwrap();
    let mut get = |s: &str| {
        s.parse()
            .unwrap_or_else(|_| wire_value(map, s.to_string(), cache))
    };
    let res = if let Some((a, b)) = src_str.split_once(" AND ") {
        get(a) & get(b)
    } else if let Some((a, b)) = src_str.split_once(" LSHIFT ") {
        get(a) << get(b)
    } else if let Some(a) = src_str.strip_prefix("NOT ") {
        !get(a)
    } else if let Some((a, b)) = src_str.split_once(" OR ") {
        get(a) | get(b)
    } else if let Some((a, b)) = src_str.split_once(" RSHIFT ") {
        get(a) >> get(b)
    } else {
        get(src_str)
    };
    cache.insert(wire, res);
    res
}

fn part1(input: &str) -> u16 {
    let map: HashMap<_, _> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(ToString::to_string)
                .collect_tuple::<(_, _)>()
                .unwrap()
                .swap()
        })
        .collect();

    let mut cache = HashMap::new();

    wire_value(&map, "a".to_string(), &mut cache)
}

fn part2(input: &str) -> u16 {
    let map: HashMap<_, _> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(ToString::to_string)
                .collect_tuple::<(_, _)>()
                .unwrap()
                .swap()
        })
        .collect();

    let mut cache = HashMap::new();

    let orig_a = wire_value(&map, "a".to_string(), &mut cache);

    cache.clear();

    cache.insert("b".to_string(), orig_a);

    wire_value(&map, "a".to_string(), &mut cache)
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
