use std::collections::{HashMap, HashSet};

use lib::{Dir, Grid, IteratorExt, Point2, itertools::Itertools, point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    // println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_bytes(input).unwrap();
    let start = grid.find(&b'S').unwrap();
    let grid = grid.map(|c| c != b'#');
    let mut positions: HashSet<Point2<usize>> = HashSet::new();
    let mut next_pos = HashSet::new();

    positions.insert(start);

    for _ in 0..64 {
        next_pos.clear();
        for pos in &positions {
            for next in Dir::ORTHO
                .into_iter()
                .filter_map(|dir| pos.apply(dir))
                .filter(|p| grid.get(*p).is_some_and(|x| *x))
            {
                next_pos.insert(next);
            }
        }
        std::mem::swap(&mut positions, &mut next_pos);
    }

    positions.len() as u32
}

fn get_wrapping(g: &Grid<bool>, p: Point2<i32>) -> Option<&bool> {
    let Point2 { x, y } = p;
    g.get(point2(
        x.rem_euclid(g.width() as i32) as usize,
        y.rem_euclid(g.height() as i32) as usize,
    ))
}

fn neighbors(
    g: &Grid<bool>,
    p: Point2<i32>,
    cache: &mut HashMap<Point2<i32>, Vec<Dir>>,
) -> Vec<Dir> {
    let Point2 { x, y } = p;
    let p_mod = point2(
        x.rem_euclid(g.width() as i32),
        y.rem_euclid(g.height() as i32),
    );
    if let Some(cached) = cache.get(&p_mod) {
        return cached.clone();
    }
    let neighbors = Dir::ORTHO
        .into_iter()
        // .filter_map(|dir| p.apply(dir))
        .filter(|dir| {
            p.apply(*dir)
                .and_then(|p| get_wrapping(g, p))
                .is_some_and(|x| *x)
        })
        .collect_vec();
    cache.insert(p_mod, neighbors.clone());
    neighbors
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_bytes(input).unwrap();
    let start = grid.find(&b'S').unwrap().map(|d| d as i32);
    let grid = grid.map(|c| c != b'#');
    let mut positions: HashSet<Point2<i32>> = HashSet::new();
    let mut next_pos = HashSet::new();

    positions.insert(start);

    let mut neighbor_cache = HashMap::new();

    for _ in 0..500 {
        next_pos.clear();
        for pos in &positions {
            for next_dir in neighbors(&grid, *pos, &mut neighbor_cache) {
                next_pos.insert(pos.apply(next_dir).unwrap());
            }
        }
        std::mem::swap(&mut positions, &mut next_pos);
    }

    positions.len() as u32
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        // assert_eq!(part1(input), 3847);
        black_box(part2(input));
        // assert_eq!(part2(input), todo!());
    }
}
