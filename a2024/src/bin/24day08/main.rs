use std::{
    collections::{HashMap, HashSet},
    vec,
};

use lib::{Entity, Grid, IteratorExt, Offset, Point2, Swap, Vec2, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn get_antennas(grid: Grid<char>) -> HashMap<char, Vec<Point2<usize>>> {
    grid.into_enumerate()
        .filter(|(_, c)| *c != '.')
        .map(Swap::swap)
        .collect_hashmap(|v| vec![v], Vec::push)
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();
    let en = Entity::new_on_grid((0, 0), Vec2::EAST, &grid).unwrap();
    let antennas = get_antennas(grid);

    antennas
        .into_values()
        .flat_map(|points| {
            points.into_iter().tuple_combinations().flat_map(|(a, b)| {
                let vector = Vec2::between(a, b).unwrap();
                [
                    en.set_bounded(a, vector.reverse()).unwrap(),
                    en.set_bounded(b, vector).unwrap(),
                ]
                .into_iter()
                .filter_map(Entity::step_bounded)
                .map(Entity::pos)
            })
        })
        .unique()
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();
    let en = Entity::new_on_grid((0, 0), Vec2::EAST, &grid).unwrap();
    let antennas = get_antennas(grid);

    let mut seen = HashSet::new();

    for points in antennas.into_values() {
        for (a, b) in points.into_iter().tuple_combinations() {
            let vector = Vec2::between(a, b).unwrap();
            let mut en1 = en.set_bounded(a, vector.reverse()).unwrap();
            let mut en2 = en.set_bounded(a, vector).unwrap();

            seen.insert(en1.pos());
            seen.insert(en2.pos());

            while let Some(next) = en1.step_bounded() {
                seen.insert(next.pos());
                en1 = next;
            }

            while let Some(next) = en2.step_bounded() {
                seen.insert(next.pos());
                en2 = next;
            }
        }
    }

    seen.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 289);
        assert_eq!(part2(input), 1030);
    }
}
