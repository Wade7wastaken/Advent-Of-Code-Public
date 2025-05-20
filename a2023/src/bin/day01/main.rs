use lib::ConditionalRev;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn find_digit(l: &str, rev: bool) -> u8 {
    l.bytes().rev_if(rev).find(u8::is_ascii_digit).unwrap() - b'0'
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| u32::from(find_digit(l, false) * 10 + find_digit(l, true)))
        .sum()
}

const DIGIT_WORD_MAP: [(&str, u8); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_word(l: &str, rev: bool) -> u8 {
    for i in (0..l.len()).rev_if(rev) {
        let c = l.as_bytes()[i];
        if c.is_ascii_digit() {
            return c - b'0';
        }
        for (word, digit) in DIGIT_WORD_MAP {
            if l[i..].starts_with(word) {
                return digit;
            }
        }
    }
    unreachable!();
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| u32::from(find_word(l, false) * 10 + find_word(l, true)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 54388);
        assert_eq!(part2(input), 53515);
    }
}
