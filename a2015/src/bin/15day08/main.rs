use lib::CountWhere;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn memory_rep(line: &str) -> u32 {
    let initial_len = line.len() as u32;
    let mut chars = line.bytes();
    let mut count = 0;
    while let Some(next) = chars.next() {
        if next == b'\\' {
            let escaped = chars.next().unwrap();
            if escaped == b'x' {
                chars.nth(1); // advance 2
            }
        }
        count += 1;
    }

    initial_len - (count - 2) // begin and end quote
}

fn part1(input: &str) -> u32 {
    input.lines().map(memory_rep).sum()
}

fn code_rep(line: &str) -> u32 {
    line.bytes().count_where(|c| c == b'"' || c == b'\\') as u32 + 2 // begin and end quote
}

fn part2(input: &str) -> u32 {
    input.lines().map(code_rep).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1342);
        assert_eq!(part2(input), 2074);
    }
}
