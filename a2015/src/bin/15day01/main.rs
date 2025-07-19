fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> i32 {
    input.bytes().fold(0, |acc, c| match c {
        b'(' => acc + 1,
        b')' => acc - 1,
        _ => panic!("invalid char: {c}"),
    })
}

fn part2(input: &str) -> u32 {
    let mut floor: i32 = 0;
    input
        .bytes()
        .map(|c| {
            match c {
                b'(' => floor += 1,
                b')' => floor -= 1,
                _ => panic!("invalid char: {c}"),
            }
            floor
        })
        .position(|f| f < 0)
        .unwrap() as u32
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 232);
        assert_eq!(part2(input), 1783);
    }
}
