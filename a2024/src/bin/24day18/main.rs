use std::collections::HashSet;

use lib::{Dir, Grid, Point2, a_star_score, a_star_single, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_point(line: &str) -> Point2<usize> {
    Point2::from_tuple(
        line.split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap(),
    )
}

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const FIRST_N: usize = 1024;

fn part1(input: &str) -> u32 {
    let end = Point2::new(WIDTH - 1, HEIGHT - 1);

    let mut grid = Grid::new_filled(true, WIDTH, HEIGHT);
    for p in input.lines().take(FIRST_N).map(parse_point) {
        grid.set(p, false).unwrap();
    }

    a_star_score(
        vec![Point2::new(0, 0)],
        |p| *p == end,
        |p| {
            grid.with_offsets(*p, Dir::ORTHO)
                .enumerate()
                .filter(|(_, c)| **c)
                .map(|(p, _)| (p, 1))
        },
        |p| p.manhattan_dist(end) as u32,
    )
    .unwrap()
}

fn part2(input: &str) -> String {
    let end = Point2::new(WIDTH - 1, HEIGHT - 1);

    let mut grid = Grid::new_filled(true, WIDTH, HEIGHT);

    let mut cur_path = HashSet::new();

    for (i, p) in input.lines().map(parse_point).enumerate() {
        grid.set(p, false).unwrap();
        if i <= FIRST_N {
            continue;
        }
        if !cur_path.is_empty() && !cur_path.contains(&p) {
            continue;
        }
        let path_result = a_star_single(
            vec![Point2::new(0, 0)],
            |p| *p == end,
            |p| {
                grid.with_offsets(*p, Dir::ORTHO)
                    .enumerate()
                    .filter(|(_, c)| **c)
                    .map(|(p, _)| (p, 1))
            },
            |p| p.manhattan_dist(end) as u32,
        );

        if let Some(path) = path_result {
            cur_path = path.path().into_iter().collect();
        } else {
            return format!("{},{}", p.x, p.y);
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 246);
        assert_eq!(part2(input), "22,50");
    }
}
