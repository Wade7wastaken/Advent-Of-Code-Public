use std::collections::HashMap;

use lib::{
    DigestHex, cycle,
    itertools::Itertools,
    md5::{self, Context},
};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn last_key_index(iter: impl Iterator<Item = (usize, [u8; 32])>) -> u32 {
    let mut map: HashMap<u8, Vec<usize>> = HashMap::new();

    let mut keys = 0;

    for (i, hash) in iter {
        for v in map.values_mut() {
            v.retain(|key| i - *key <= 1000);
        }
        let valid_keys = hash
            .windows(5)
            .find_map(|w| w.iter().all_equal_value().ok())
            .and_then(|found| map.get(found))
            .into_iter()
            .flatten();

        for valid_key in valid_keys {
            keys += 1;
            if keys >= 64 {
                return *valid_key as u32;
            }
        }

        let possible_key = hash
            .windows(3)
            .find_map(|w| w.iter().all_equal_value().ok());

        if let Some(key) = possible_key {
            map.entry(*key).or_default().push(i);
        }
    }
    panic!();
}

fn part1(input: &str) -> u32 {
    let iter = (0..usize::MAX).map(|i| {
        let mut c = Context::new();
        c.consume(input);
        c.consume(i.to_string());
        (i, c.compute().to_hex())
    });

    last_key_index(iter)
}

fn part2(input: &str) -> u32 {
    let iter = (0..usize::MAX).map(|i| {
        let mut c = Context::new();
        c.consume(input);
        c.consume(i.to_string());
        let hash = c.compute().to_hex();
        (i, cycle(hash, 2016, |hash| md5::compute(hash).to_hex()))
    });

    last_key_index(iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 23890);
        assert_eq!(part2(input), 22696);
    }
}
