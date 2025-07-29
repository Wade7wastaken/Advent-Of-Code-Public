use a2017::knot_hash;
use lib::Grid;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    (0..=127)
        .flat_map(|i| {
            knot_hash(&format!("{input}-{i}"))
                .into_iter()
                .map(u8::count_ones)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let iter = (0..=127).map(|y| {
        let hash = knot_hash(&format!("{input}-{y}"));
        hash.into_iter()
            .flat_map(|byte| (0..8).rev().map(move |x| (byte & (1 << x)) != 0))
    });
    let mut grid = Grid::from_double_iter(iter).unwrap();

    let mut count: u32 = 0;

    while let Some(start) = grid.find(&true) {
        count += 1;
        grid.fill(start, &false);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 8194);
        assert_eq!(part2(input), 1141);
    }
}
