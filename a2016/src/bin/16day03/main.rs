use lib::{
    IteratorExt,
    itertools::{Itertools, multiunzip},
};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn get_triangles(input: &str) -> impl Iterator<Item = (u32, u32, u32)> {
    input.lines().map(|line| {
        line.trim()
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap()
    })
}

const fn is_valid_triangle((a, b, c): (u32, u32, u32)) -> bool {
    !(a + b <= c || a + c <= b || b + c <= a)
}

fn part1(input: &str) -> u32 {
    get_triangles(input).count_where(is_valid_triangle) as u32
}

fn part2(input: &str) -> u32 {
    let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(get_triangles(input));

    a.into_iter()
        .tuples()
        .chain(b.into_iter().tuples())
        .chain(c.into_iter().tuples())
        .count_where(is_valid_triangle) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 983);
        assert_eq!(part2(input), 1836);
    }
}
