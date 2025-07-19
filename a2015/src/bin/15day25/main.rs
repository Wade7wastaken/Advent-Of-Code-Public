use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
}

fn part1(input: &str) -> u64 {
    let words = input.split_ascii_whitespace().collect_vec();
    let x: u64 = words[17].strip_suffix('.').unwrap().parse().unwrap();
    let y: u64 = words[15].strip_suffix(',').unwrap().parse().unwrap();

    let code_number = u64::midpoint(x * (x + 1), (2 * x + y - 2) * (y - 1)) - 1;

    let mut code = 20151125;

    for _ in 0..code_number {
        code *= 252533;
        code %= 33554393;
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 9132360);
    }
}
