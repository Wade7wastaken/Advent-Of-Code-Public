use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let containers = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect_vec();

    let n = containers.len();

    let mut counter = 0;

    for i in 1..=n {
        for combination in containers.iter().combinations(i) {
            if combination.iter().copied().sum::<u32>() == 150 {
                counter += 1;
            }
        }
    }

    counter
}

fn part2(input: &str) -> u32 {
    let containers = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect_vec();

    let n = containers.len();

    for i in 1..=n {
        let mut counter = 0;
        for combination in containers.iter().combinations(i) {
            if combination.iter().copied().sum::<u32>() == 150 {
                counter += 1;
            }
        }
        if counter != 0 {
            return counter;
        }
    }

    panic!()
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
