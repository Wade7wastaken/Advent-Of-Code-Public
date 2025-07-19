use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut offsets = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec();
    let mut i: i32 = 0;
    let mut steps = 0;
    while let Some(jumps) = i.try_into().ok().and_then(|i: usize| offsets.get_mut(i)) {
        steps += 1;
        i += *jumps;
        *jumps += 1;
    }
    steps
}

fn part2(input: &str) -> u32 {
    let mut offsets = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec();
    let mut i: i32 = 0;
    let mut steps = 0;
    while let Some(jumps) = i.try_into().ok().and_then(|i: usize| offsets.get_mut(i)) {
        steps += 1;
        i += *jumps;
        if *jumps >= 3 {
            *jumps -= 1;
        } else {
            *jumps += 1;
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 391540);
        assert_eq!(part2(input), 30513679);
    }
}
