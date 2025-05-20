use lib::{itertools::Itertools, Grid, Point2, Surround};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn ending_points(grid: &Grid<u32>, p: Point2<usize>, target_digit: u32) -> Vec<Point2<usize>> {
    if target_digit == 10 {
        return vec![p];
    }
    grid.surrounding(p, Surround::Ortho)
        .enumerate()
        .filter(|(_, c)| **c == target_digit)
        .flat_map(|(p, c)| ending_points(grid, p, c + 1))
        .collect()
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars(input)
        .unwrap()
        .map(|c| c.to_digit(10).unwrap());

    grid.find_all(&0)
        .map(|p| ending_points(&grid, p, 1).into_iter().unique().count() as u32)
        .sum()
}

fn ending_paths(grid: &Grid<u32>, p: Point2<usize>, target_digit: u32) -> u32 {
    if target_digit == 10 {
        return 1;
    }
    grid.surrounding(p, Surround::Ortho)
        .enumerate()
        .filter(|(_, c)| **c == target_digit)
        .map(|(p, c)| ending_paths(grid, p, c + 1))
        .sum()
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_chars(input)
        .unwrap()
        .map(|c| c.to_digit(10).unwrap());

    grid.find_all(&0).map(|p| ending_paths(&grid, p, 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 501);
        assert_eq!(part2(input), 1017);
    }
}
