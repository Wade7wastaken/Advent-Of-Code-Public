use lib::{IteratorExt, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

// doesn't check for overflows
fn increment(password: &mut str) {
    let mut n = password.len() - 1;
    let bytes = unsafe { password.as_bytes_mut() };
    loop {
        if bytes[n] == b'z' {
            bytes[n] = b'a';
            n -= 1;
        } else {
            bytes[n] += 1;
            return;
        }
    }
}

fn is_valid(p: &str) -> bool {
    // has increasing
    p.bytes()
        .tuple_windows()
        .any(|(a, b, c)| a == b - 1 && b == c - 1)
    // contains banned letters
        && !(p.contains('i') || p.contains('o') || p.contains('l'))
    // contains pairs
        && p.bytes()
            .tuple_windows()
            .enumerate()
            .count_where(|(i, (a, b))| a == b && (i == 0 || p.as_bytes()[i - 1] != a))
            >= 2
}

fn part1(input: &str) -> String {
    let mut password = input.to_string();
    while !is_valid(&password) {
        increment(&mut password);
    }
    password
}

fn part2(input: &str) -> String {
    let mut password = input.to_string();
    while !is_valid(&password) {
        increment(&mut password);
    }
    increment(&mut password);
    while !is_valid(&password) {
        increment(&mut password);
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "hepxxyzz");
        assert_eq!(part2(input), "heqaabcc");
    }
}
