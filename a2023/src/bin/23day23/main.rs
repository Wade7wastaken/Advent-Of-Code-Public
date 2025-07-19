use std::collections::HashSet;

use lib::{Dir, Entity, Grid, Point2, itertools::Either};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

struct Path {
    seen: HashSet<Point2<usize>>,
    en: Entity<usize>,
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars_transpose(input).unwrap();
    let starting_x = grid.row(0).unwrap().iter().position(|c| *c == '.').unwrap();

    let mut paths = vec![Path {
        en: Entity::new_on_grid((starting_x, 0), Dir::South, &grid).unwrap(),
        seen: HashSet::new(),
    }];

    let mut largest_so_far = 0;

    while let Some(mut path) = paths.pop() {
        let mut new_paths = loop {
            if path.en.pos().y == grid.height() - 1 {
                largest_so_far = largest_so_far.max(path.seen.len() as u32);
                break vec![];
            }

            path.seen.insert(path.en.pos());

            let available_dirs = if let Ok(arrow) = Dir::try_from(*grid.get(path.en.pos()).unwrap())
            {
                Either::Left([arrow])
            } else {
                Either::Right(Dir::ORTHO)
            };

            let next_steps = available_dirs
                .into_iter()
                .filter_map(|dir| path.en.set_dir(dir).step_bounded())
                .filter(|en| *grid.get(en.pos()).unwrap() != '#' && !path.seen.contains(&en.pos()))
                .collect::<Vec<_>>();

            if next_steps.is_empty() {
                break vec![];
            }

            if next_steps.len() == 1 {
                let next = next_steps[0];
                path.en = next;
                continue;
            }

            break next_steps
                .into_iter()
                .map(|en| Path {
                    seen: path.seen.clone(),
                    en,
                })
                .collect();
        };
        paths.append(&mut new_paths);
    }

    largest_so_far
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_chars_transpose(input).unwrap();
    let starting_x = grid.row(0).unwrap().iter().position(|c| *c == '.').unwrap();

    let mut paths = vec![Path {
        en: Entity::new_on_grid((starting_x, 0), Dir::South, &grid).unwrap(),
        seen: HashSet::new(),
    }];

    let mut largest_so_far = 0;

    while let Some(mut path) = paths.pop() {
        let mut new_paths = loop {
            if path.en.pos().y == grid.height() - 1 {
                largest_so_far = largest_so_far.max(path.seen.len() as u32);
                break vec![];
            }

            path.seen.insert(path.en.pos());

            let next_steps = Dir::ORTHO
                .into_iter()
                .filter_map(|dir| path.en.set_dir(dir).step_bounded())
                .filter(|en| *grid.get(en.pos()).unwrap() != '#' && !path.seen.contains(&en.pos()))
                .collect::<Vec<_>>();

            if next_steps.is_empty() {
                break vec![];
            }

            if next_steps.len() == 1 {
                let next = next_steps[0];
                path.en = next;
                continue;
            }

            break next_steps
                .into_iter()
                .map(|en| Path {
                    seen: path.seen.clone(),
                    en,
                })
                .collect();
        };
        paths.append(&mut new_paths);
    }

    largest_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 2194);
        // assert_eq!(part2(input), 6410);
    }
}
