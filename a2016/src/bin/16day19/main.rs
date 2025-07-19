fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let n: u32 = input.parse().unwrap();
    2 * n - n.next_power_of_two() + 1
}

fn part2(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1834471);
        // assert_eq!(part2(input), todo!());
    }
}
