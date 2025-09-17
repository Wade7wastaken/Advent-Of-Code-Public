use lib::IteratorExt;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn paren(c: u8) -> i32 {
    match c {
        b'(' => 1,
        b')' => -1,
        _ => panic!(),
    }
}

fn part1(input: &str) -> i32 {
    input.bytes().fold(0, |acc, c| acc + paren(c))
}

fn part2(input: &str) -> u32 {
    input
        .bytes()
        .apply(0, |floor, c| floor + paren(c))
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
