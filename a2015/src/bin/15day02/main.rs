use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (a, b, c) = l
                .split('x')
                .map(|n| n.parse::<u32>().unwrap())
                .tuple_combinations()
                .map(|(a, b)| a * b)
                .collect_tuple()
                .unwrap();
            (a + b + c) * 2 + a.min(b).min(c)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let a = l.split('x').map(|n| n.parse::<u32>().unwrap());
            let ribbon = a.clone().k_smallest(2).sum::<u32>() * 2;
            let bow = a.product::<u32>();

            ribbon + bow
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1598415);
        assert_eq!(part2(input), 3812909);
    }
}
