use lib::{Dir, IteratorExt, itertools::Itertools, point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .bytes()
        .map(|a| Dir::try_from(a).unwrap())
        .apply(point2(0, 0), |pos, dir| pos.apply(dir).unwrap())
        .unique()
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .bytes()
        .map(|a| Dir::try_from(a).unwrap())
        .tuples()
        .apply((point2(0, 0), point2(0, 0)), |(santa, robot), (a, b)| {
            (santa.apply(a).unwrap(), robot.apply(b).unwrap())
        })
        .flat_map(|(a, b)| [a, b])
        .unique()
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 2572);
        assert_eq!(part2(input), 2631);
    }
}
