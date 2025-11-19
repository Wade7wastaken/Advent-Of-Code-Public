use std::{collections::HashMap, ops::RangeInclusive};

use lib::{Dir, Grid, Swap, Vec2, a_star_single};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

const fn range(r: isize) -> RangeInclusive<isize> {
    -r..=r
}

fn all_offsets_within(r: isize) -> impl Iterator<Item = (isize, Vec2)> {
    range(r).flat_map(move |dy| {
        range(r - dy.abs()).map(move |dx| (dy.abs() + dx.abs(), Vec2::new(dx, dy)))
    })
}

fn solve(input: &str, r: isize) -> u32 {
    let grid = Grid::from_bytes(input).unwrap();

    let start = grid.find(&b'S').unwrap();
    let end = grid.find(&b'E').unwrap();

    let path = a_star_single(
        vec![start],
        |p| *p == end,
        |p| {
            grid.with_offsets(*p, Dir::ORTHO)
                .enumerate()
                .filter(|(_, c)| **c != b'#')
                .map(|(p, _)| (p, 1))
        },
        |_| 0,
    )
    .unwrap()
    .path()
    .0;

    let numbered_path = path.into_iter().enumerate();

    let distances = &numbered_path
        .clone()
        .map(Swap::swap)
        .collect::<HashMap<_, _>>();

    numbered_path
        .flat_map(|(i, c)| {
            all_offsets_within(r).filter_map(move |(cheat_dist, dir)| {
                c.apply(dir)
                    .and_then(|cheat_end| distances.get(&cheat_end))
                    .filter(|cheat_end_score| {
                        **cheat_end_score as isize - i as isize - cheat_dist >= 100
                    })
            })
        })
        .count() as u32
}

fn part1(input: &str) -> u32 {
    solve(input, 2)
}

fn part2(input: &str) -> u32 {
    solve(input, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1355);
        assert_eq!(part2(input), 1007335);
    }
}
