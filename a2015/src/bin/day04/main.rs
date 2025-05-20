fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    for i in 1.. {
        let hash = lib::md5::compute(format!("{input}{i}"));
        if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0 {
            return i;
        }
    }
    unreachable!()
}

fn part2(input: &str) -> u32 {
    for i in 1.. {
        let hash = lib::md5::compute(format!("{input}{i}"));
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 346386);
        assert_eq!(part2(input), 9958218);
    }
}
