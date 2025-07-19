use lib::{
    regex::{Match, Regex},
    tern,
};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn execute_mul_command(s: Match) -> u32 {
    s.as_str()
        .strip_prefix("mul(")
        .and_then(|s| s.strip_suffix(')'))
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .product()
}

fn part1(input: &str) -> u32 {
    Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
        .unwrap()
        .find_iter(input)
        .map(execute_mul_command)
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut enabled = true;

    Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)")
        .unwrap()
        .find_iter(input)
        .map(|s| match s.as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => tern!(enabled, execute_mul_command(s), 0),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 161289189);
        assert_eq!(part2(input), 83595109);
    }
}
