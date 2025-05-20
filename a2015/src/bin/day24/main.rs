use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

// 516 too low

fn part1(input: &str) -> u64 {
    let packages = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();
    let total_weight: u64 = packages.iter().sum();
    let goal_weight = total_weight / 3;
    for k in 1.. {
        let mut possible_groups = vec![];
        for combination in packages.iter().combinations(k) {
            if combination.iter().copied().sum::<u64>() == goal_weight {
                possible_groups.push(combination);
            }
        }
        if !possible_groups.is_empty() {
            return possible_groups
                .into_iter()
                .map(|group| group.into_iter().product())
                .min()
                .unwrap();
        }
    }

    panic!();
}

fn part2(input: &str) -> u64 {
    let packages = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();
    let total_weight: u64 = packages.iter().sum();
    let goal_weight = total_weight / 4;
    for k in 1.. {
        let mut possible_groups = vec![];
        for combination in packages.iter().combinations(k) {
            if combination.iter().copied().sum::<u64>() == goal_weight {
                possible_groups.push(combination);
            }
        }
        if !possible_groups.is_empty() {
            return possible_groups
                .into_iter()
                .map(|group| group.into_iter().product())
                .min()
                .unwrap();
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 11266889531);
        assert_eq!(part2(input), 77387711);
    }
}
