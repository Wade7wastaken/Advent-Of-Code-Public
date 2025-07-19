use std::collections::HashSet;

use lib::{itertools::Itertools, CountWhere, Dir, Grid, Offset, Point2, Vec2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn separate_regions(mut grid: Grid<u16>) -> Grid<u16> {
    let start = *grid.iter().max().unwrap();
    let mut next = start + 1;

    for plant in grid.iter().copied().unique().collect_vec() {
        while let Some(p) = grid.find(&plant) {
            grid.fill(p, &next);
            next += 1;
        }
    }

    grid
}

fn calc_perimeter(grid: &Grid<u16>, plant_id: u16) -> usize {
    let mut perimeter = 0;
    for p in grid.find_all(&plant_id) {
        for dir in Dir::ORTHO {
            if let Some(adj) = grid.get_offset(p, dir) {
                if *adj != plant_id {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_bytes(input)
        .unwrap()
        .map(u16::from);
    let grid = separate_regions(grid);

    grid.iter()
        .copied()
        .unique()
        .map(|plant_id| grid.count(&plant_id) * calc_perimeter(&grid, plant_id))
        .sum()
}

fn external_vertices(grid: &Grid<u16>, p: Point2<usize>, plant_id: u16) -> usize {
    let edges = Dir::ORTHO
        .into_iter()
        .filter(|dir| grid.get_offset(p, *dir).is_none_or(|adj| *adj != plant_id))
        .collect_vec();

    match edges[..] {
        [a, b] if a.is_ortho(b) => 1,
        [_, _, _] => 2,
        [_, _, _, _] => 4,
        _ => 0,
    }
}

fn internal_vertices(
    grid: &Grid<u16>,
    p: Point2<usize>,
    plant_id: u16,
    checked_external: &mut HashSet<Point2<usize>>,
) -> usize {
    grid.with_offsets(p, Dir::ORTHO)
        .enumerate()
        .map(|(q, c)| {
            if checked_external.contains(&q) || *c == plant_id {
                return 0;
            }
            checked_external.insert(q);

            Dir::ORTHO
                .into_iter()
                .filter(|dir| grid.get_offset(q, *dir).is_some_and(|adj| *adj == plant_id))
                .tuple_combinations()
                .count_where(|(a, b)| {
                    !a.is_reverse(b) && grid.get_offset(q, Vec2::from(a) + Vec2::from(b)) == Some(&plant_id)
                })
        })
        .sum()
}

fn calc_sides(grid: &Grid<u16>, plant_id: u16) -> usize {
    let mut checked_internal = HashSet::new();
    grid.find_all(&plant_id)
        .map(|p| {
            external_vertices(grid, p, plant_id)
                + internal_vertices(grid, p, plant_id, &mut checked_internal)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_bytes(input)
        .unwrap()
        .map(u16::from);
    let grid = separate_regions(grid);

    grid.iter()
        .copied()
        .unique()
        .map(|plant_id| grid.count(&plant_id) * calc_sides(&grid, plant_id))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1363484);
        assert_eq!(part2(input), 838988);
    }
}
