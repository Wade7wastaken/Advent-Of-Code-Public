use std::collections::{HashMap, HashSet};

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (k, v) in input.lines().map(|l| l.split_once('-').unwrap()) {
        map.entry(k).or_default().push(v);
        map.entry(v).or_default().push(k);
    }

    let mut sets = HashSet::new();

    for (ka, cons_to_ka) in &map {
        for (kb, kc) in cons_to_ka.iter().tuple_combinations() {
            if map.get(kb).unwrap().contains(kc)
                && (ka.starts_with('t') || kb.starts_with('t') || kc.starts_with('t'))
            {
                let mut buf = [ka, kb, kc];
                buf.sort();
                sets.insert(buf);
            }
        }
    }

    sets.len() as u32
}

fn part2(input: &str) -> String {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (k, v) in input.lines().map(|l| l.split_once('-').unwrap()) {
        map.entry(k).or_default().push(v);
        map.entry(v).or_default().push(k);
    }

    let mut highest_set = HashSet::new();

    for ka in map.keys() {
        let mut cur_set = HashSet::new();
        cur_set.insert(ka);
        for (k_next, cons_to_k_next) in &map {
            if cur_set.iter().all(|cur| cons_to_k_next.contains(cur)) {
                cur_set.insert(k_next);
            }
        }
        if cur_set.len() > highest_set.len() {
            highest_set = cur_set;
        }
    }
    highest_set.into_iter().sorted().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1149);
        assert_eq!(part2(input), "as,co,do,kh,km,mc,np,nt,un,uq,wc,wz,yo");
    }
}
