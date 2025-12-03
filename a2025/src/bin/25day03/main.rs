fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn solve(input: &str, batteries: usize) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut dp = vec![None; batteries + 1];
            dp[0] = Some(0);

            for battery in l.bytes().map(|b| u64::from(b - b'0')) {
                for x in (1..=batteries).rev() {
                    dp[x] = dp[x].max(dp[x - 1].map(|joltage| 10 * joltage + battery));
                }
            }

            dp[batteries].unwrap()
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    solve(input, 2)
}

fn part2(input: &str) -> u64 {
    solve(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 17034);
        assert_eq!(part2(input), 168798209663590);
    }
}
