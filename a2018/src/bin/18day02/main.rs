use lib::{IteratorExt, equal_combine, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    // println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut two = 0;
    let mut three = 0;
    for l in input.lines() {
        let counts = l.bytes().counts();
        if counts.iter().any(|c| *c.1 == 2) {
            two += 1;
        }
        if counts.iter().any(|c| *c.1 == 3) {
            three += 1;
        }
    }

    two * three
}

fn part2(input: &str) -> String {
    input
        .lines()
        .tuple_combinations()
        .filter(|(a, b)| a.bytes().zip_eq(b.bytes()).count_where(|(a, b)| a != b) == 1)
        .map(|(a, b)| {
            a.chars()
                .zip_eq(b.chars())
                .filter_map(|(a, b)| equal_combine(a, b))
                .collect()
        })
        .exactly_one()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 7134);
        assert_eq!(part2(input), "kbqwtcvzhmhpoelrnaxydifyb");
    }
}
