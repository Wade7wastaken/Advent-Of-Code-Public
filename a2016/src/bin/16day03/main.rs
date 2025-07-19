use lib::{
    CountWhere,
    itertools::{Itertools, multiunzip},
};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.lines().count_where(|line| {
        line.trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .permutations(3)
            .all(|p| (p[0] + p[1] > p[2]))
    }) as u32
}

fn part2(input: &str) -> u32 {
    let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(input.lines().map(|l| {
        l.trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap()
    }));

    a.chunks_exact(3)
        .chain(b.chunks_exact(3))
        .chain(c.chunks_exact(3))
        .count_where(|tri| tri.iter().permutations(3).all(|p| (p[0] + p[1] > *p[2]))) as u32
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
