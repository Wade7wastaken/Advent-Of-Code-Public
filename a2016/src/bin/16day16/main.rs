use lib::{IteratorExt, itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn dragon_curve_step(s: &mut String) {
    let mut b = s.clone();
    let bytes = unsafe { b.as_bytes_mut() };
    bytes.reverse();
    for x in bytes {
        *x = tern!(*x == b'0', b'1', b'0');
    }
    s.push('0');
    s.push_str(&b);
}

fn checksum(s: String) -> String {
    s.into_bytes()
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| tern!(chunk.all_equal(), b'1', b'0'))
        .collect_string()
}

fn solve(input: &str, disc_len: usize) -> String {
    let mut s = input.to_string();
    s.reserve(disc_len - s.len());
    while s.len() < disc_len {
        dragon_curve_step(&mut s);
    }
    s.truncate(disc_len);
    while s.len() % 2 != 1 {
        s = checksum(s);
    }
    s
}

fn part1(input: &str) -> String {
    solve(input, 272)
}

fn part2(input: &str) -> String {
    solve(input, 35651584)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "11101010111100010");
        assert_eq!(part2(input), "01001101001000101");
    }
}
