use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn is_invalid(n: u64) -> bool {
    let string = n.to_string();
    let len = string.len();
    if len % 2 != 0 {
        return false;
    }
    let middle = len / 2;
    string[..middle] == string[middle..]
}

fn part1(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .flat_map(|(start, end)| start..=end)
        .filter(|n| is_invalid(*n))
        .sum()
}

fn is_invalid2(n: u64) -> bool {
    let string = n.to_string();
    let len = string.len();
    for i in 1..=(len/2) {
        if len % i != 0 {
            continue;
        }
        let num_segments = len / i;
        if string[..i].repeat(num_segments) == string {
            return true;
        }
    }
    
    false
}

fn part2(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .flat_map(|(start, end)| start..=end)
        .filter(|n| is_invalid2(*n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 8576933996);
        assert_eq!(part2(input), 25663320831);
    }
}
