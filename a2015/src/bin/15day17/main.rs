use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

const TARGET: usize = 150;

fn part1(input: &str) -> u32 {
    let containers = input.lines().map(|l| l.parse::<usize>().unwrap());

    let mut dp = vec![0; TARGET + 1];
    dp[0] = 1;

    for container in containers {
        for left in (container..=TARGET).rev() {
            dp[left] += dp[left - container];
        }
    }

    dp[TARGET]
}

fn part2(input: &str) -> u32 {
    let containers = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();
    let len = containers.len();

    let mut dp = vec![vec![0; TARGET + 1]; len + 1];
    dp[0][0] = 1;

    for container in containers {
        for k in (1..=len).rev() {
            for left in (container..=TARGET).rev() {
                dp[k][left] += dp[k - 1][left - container];
            }
        }
    }

    (1..=len)
        .map(|k| dp[k][TARGET])
        .find(|counter| *counter != 0)
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1304);
        assert_eq!(part2(input), 18);
    }
}
