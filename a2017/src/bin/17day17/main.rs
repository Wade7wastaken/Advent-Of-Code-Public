fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let steps: usize = input.parse().unwrap();
    let mut buffer = Vec::with_capacity(2018);
    buffer.push(0);
    let mut i = 0;
    for x in 1..=2017 {
        i = ((i + steps) % x) + 1;
        buffer.insert(i, x as u32);
    }
    buffer[i + 1]
}

fn part2(input: &str) -> u32 {
    let steps: usize = input.parse().unwrap();
    let mut after_zero = 0;
    let mut i = 0;
    for x in 1..=50000000 {
        i = ((i + steps) % x) + 1;
        if i == 1 {
            after_zero = x as u32;
        }
    }
    after_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1311);
        assert_eq!(part2(input), 39170601);
    }
}
