use std::collections::HashMap;

use lib::{Dir, Grid, IteratorExt};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    Grid::from_chars_transpose(input)
        .unwrap()
        .applied_gravity(Dir::North, &'.', &['#'])
        .into_rows()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(load, row)| row.into_iter().count_where(|c| c == 'O') as u32 * (load as u32 + 1))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut grid = Grid::from_chars_transpose(input).unwrap();

    let mut i = 0;

    let mut map = HashMap::new();

    let (intro_length, loop_length) = loop {
        if let Some(&n) = map.get(&grid) {
            break (n, i - n);
        }
        map.insert(grid.clone(), i);
        grid.apply_gravity(Dir::North, &'.', &['#'])
            .apply_gravity(Dir::West, &'.', &['#'])
            .apply_gravity(Dir::South, &'.', &['#'])
            .apply_gravity(Dir::East, &'.', &['#']);
        i += 1;
    };

    let i = ((1000000000 - i) % loop_length) + intro_length;

    map.into_iter()
        .find(|(_, v)| *v == i)
        .unwrap()
        .0
        .into_rows()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(load, row)| row.into_iter().count_where(|c| c == 'O') as u32 * (load as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 109654);
        assert_eq!(part2(input), 94876);
    }
}
