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
            l.split(':')
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .filter(|(index, len)| index % (len + len - 2) == 0)
        .map(|(index, len)| index * len)
        .sum()
}

fn part2(input: &str) -> u32 {
    let scanners = input
        .lines()
        .map(|l| {
            l.split(':')
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    (1..u32::MAX)
        .find(|start| {
            scanners
                .iter()
                .all(|(index, len)| (index + start) % (len + len - 2) != 0)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1728);
        assert_eq!(part2(input), 3946838);
    }
}
