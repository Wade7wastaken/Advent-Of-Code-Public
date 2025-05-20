use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_sue(line: &str) -> (u32, HashMap<&str, u32>) {
    let (id_str, data) = line.split_once(": ").unwrap();
    let n = id_str.split_once(' ').unwrap().1.parse().unwrap();

    (
        n,
        data.split(", ")
            .map(|data| {
                let (name, amount_str) = data.split_once(": ").unwrap();
                (name, amount_str.parse().unwrap())
            })
            .collect(),
    )
}

fn part1(input: &str) -> u32 {
    let constants = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    'outer: for (n, properties) in input.lines().map(parse_sue) {
        for (name, amount) in properties {
            if *constants.get(name).unwrap() != amount {
                continue 'outer;
            }
        }
        return n;
    }

    panic!();
}

fn part2(input: &str) -> u32 {
    let constants = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    'outer: for (n, properties) in input.lines().map(parse_sue) {
        for (name, amount) in properties {
            match name {
                "cats" | "trees" => {
                    if *constants.get(name).unwrap() >= amount {
                        continue 'outer;
                    }
                }
                "pomeranians" | "goldfish" => {
                    if *constants.get(name).unwrap() <= amount {
                        continue 'outer;
                    }
                }
                _ => {
                    if *constants.get(name).unwrap() != amount {
                        continue 'outer;
                    }
                }
            }
        }
        return n;
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 213);
        assert_eq!(part2(input), 323);
    }
}
