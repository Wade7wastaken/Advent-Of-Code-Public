use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .bytes()
        .circular_tuple_windows()
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| u32::from(a - b'0'))
        .sum()
}

fn part2(input: &str) -> u32 {
    let vec = input.bytes().map(|x| x - b'0').collect_vec();
    let len = vec.len();
    let mut count = 0;
    for (i, x) in vec.iter().enumerate() {
        if vec.get(i) == vec.get((i + len / 2) % len) {
            count += u32::from(*x);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1031);
        assert_eq!(part2(input), 1080);
    }
}
