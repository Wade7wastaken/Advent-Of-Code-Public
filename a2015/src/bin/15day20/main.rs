fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let input = input.parse::<u32>().unwrap();
    let limit = (input / 10) as usize;

    let mut presents = vec![0; limit];

    for elf in 1..=limit {
        for house in (elf..=limit).step_by(elf) {
            presents[house - 1] += elf as u32 * 10;
        }
    }

    presents
        .into_iter()
        .position(|present| present >= input)
        .unwrap() as u32
        + 1
}

fn part2(input: &str) -> u32 {
    let input = input.parse::<u32>().unwrap();
    let limit = (input / 11) as usize;

    let mut presents = vec![0; limit];

    for elf in 1..=limit {
        for house in (elf..=limit.min(elf * 50)).step_by(elf) {
            presents[house - 1] += elf as u32 * 11;
        }
    }

    presents
        .into_iter()
        .position(|present| present >= input)
        .unwrap() as u32
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 831600);
        assert_eq!(part2(input), 884520);
    }
}
