use lib::{cycle, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn rle(s: String) -> String {
    s.into_bytes()
        .into_iter()
        .chunk_by(|x| *x)
        .into_iter()
        .map(|(c, group)| {
            let mut count = group.count().to_string();
            count.push(c as char);
            count
        })
        .collect()
}

fn run(input: &str, n: usize) -> u32 {
    cycle(input.to_string(), n, rle).len() as u32
}

fn part1(input: &str) -> u32 {
    run(input, 40)
}

fn part2(input: &str) -> u32 {
    run(input, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 252594);
        assert_eq!(part2(input), 3579328);
    }
}
