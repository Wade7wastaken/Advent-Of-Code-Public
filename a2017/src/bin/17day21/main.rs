use std::collections::HashMap;

use lib::{Grid, Inline, cycle, itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_grid(input: &str) -> Grid<bool> {
    Grid::from_double_iter_transpose(input.split('/').map(|row| row.chars()))
        .unwrap()
        .map(|c| c == '#')
}

fn enhance(g: Grid<bool>, maps: &HashMap<Grid<bool>, Grid<bool>>) -> Grid<bool> {
    assert_eq!(g.width(), g.height());
    let len = g.width();
    if len % 2 == 0 {
        let mut res = Grid::new_filled(false, len / 2 * 3, len / 2 * 3);
        for y in 0..len / 2 {
            for x in 0..len / 2 {
                let sub = g.subgrid(x * 2, y * 2, 2, 2);
                let mapped = maps.get(&sub).unwrap().clone();
                res.paste(mapped, x * 3, y * 3).unwrap();
            }
        }
        res
    } else if len % 3 == 0 {
        let mut res = Grid::new_filled(false, len / 3 * 4, len / 3 * 4);
        for y in 0..len / 3 {
            for x in 0..len / 3 {
                let sub = g.subgrid(x * 3, y * 3, 3, 3);
                let mapped = maps.get(&sub).unwrap().clone();
                res.paste(mapped, x * 4, y * 4).unwrap();
            }
        }
        res
    } else {
        panic!();
    }
}

fn grid_to_id(grid: Grid<bool>) -> u16 {
    let mut ans = 0;
    for c in grid.into_iter() {
        if c {
            ans += 1;
        }
        ans *= 2;
    }
    ans
}

fn parse_maps(input: &str) -> HashMap<u16, u16> {
    input
        .lines()
        .flat_map(|l| {
            let (mut before, after) = l
                .split("=>")
                .map(|g| parse_grid(g.trim()))
                .collect_tuple()
                .unwrap();
            (0..4)
                .map(move |_| {
                    before.retranspose_cols();
                    before.rotate();
                    before.clone()
                })
                .flat_map(|rotated| [rotated.clone(), { rotated.inline(Grid::flip_vertical) }])
                .map(|before| (grid_to_id(before), grid_to_id(after.clone())))
                .collect_vec()
        })
        .collect()
}

fn split_4(n: u16) -> [u16; 4] {
    
}

fn part1(input: &str) -> u32 {
    let maps = parse_maps(input);

    let g = grid_to_id(
        Grid::from_chars(
            ".#.
..#
###",
        )
        .unwrap()
        .map(|c| c == '#'),
    );

    let mut sections = HashMap::new();
    sections.insert(g, 1usize);

    let mut three = true;

    for i in 0..5 {
        if three {
            sections.into_iter().map(|(k, v)| {
                let new_key = *maps.get(&k).unwrap();
            })
        } else {
            sections = sections
                .into_iter()
                .map(|(k, v)| (*maps.get(&k).unwrap(), v))
                .collect();
        }

        three = !three;
    }

    cycle(g, 5, |g| enhance(g, &maps)).count(&true) as u32
}

fn part2(input: &str) -> u32 {
    let maps = parse_maps(input);

    let g = Grid::from_chars(
        ".#.
..#
###",
    )
    .unwrap()
    .map(|c| c == '#');

    cycle(g, 18, |g| enhance(g, &maps)).count(&true) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        // assert_eq!(part1(input), todo!());
        // assert_eq!(part2(input), todo!());
    }
}
