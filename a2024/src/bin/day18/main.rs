use std::collections::HashSet;

use lib::{AStarScore, AStarSingle, Grid, Point2, Surround, itertools::Itertools};

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

fn part1(input: &str) -> u32 {
    let width = 71;
    let height = 71;
    let first_n = 1024;

    let end = Point2::new(width - 1, height - 1);

    let mut grid = Grid::new_transpose(vec![vec!['.'; width]; height]).unwrap();
    for p in input.lines().take(first_n).map(parse_point) {
        grid.set(p, '#').unwrap();
    }

    AStarScore::new(
        vec![Point2::new(0, 0)],
        |p| *p == end,
        |p| {
            grid.surrounding(*p, Surround::Ortho)
                .enumerate()
                .filter(|(_, c)| **c != '#')
                .map(|(p, _)| (p, 1))
        },
        |p| p.manhattan_dist(end) as u32,
    )
    .first()
}

fn part2(input: &str) -> String {
    let width = 71;
    let height = 71;
    let first_n = 1024;

    let end = Point2::new(width - 1, height - 1);

    let mut grid = Grid::new_transpose(vec![vec!['.'; width]; height]).unwrap();

    let mut cur_path = HashSet::new();

    for (i, p) in input.lines().map(parse_point).enumerate() {
        grid.set(p, '#').unwrap();
        if i <= first_n {
            continue;
        }
        if !cur_path.is_empty() && !cur_path.contains(&p) {
            continue;
        }
        let path_result = AStarSingle::new(
            vec![Point2::new(0, 0)],
            |p| *p == end,
            |p| {
                grid.surrounding(*p, Surround::Ortho)
                    .enumerate()
                    .filter(|(_, c)| **c != '#')
                    .map(|(p, _)| (p, 1))
            },
            |p| p.manhattan_dist(end) as u32,
        )
        .next();

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
