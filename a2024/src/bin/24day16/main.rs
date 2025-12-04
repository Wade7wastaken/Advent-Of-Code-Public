use lib::{AStarMulti, Dir, Entity, Grid, Offset, a_star_score, itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

// fn find_path(input: &str) -> SinglePathResult<Entity<usize>> {
//     let grid = Grid::from_chars(input).unwrap();

//     let start = grid.find(&'S').unwrap();
//     let end = grid.find(&'E').unwrap();

//     AStarSinglePath::new(
//         vec![Entity::new_on_grid(start, Dir::EAST, &grid).unwrap()],
//         |c| c.pos() == end,
//         |en| {
//             grid.surrounding(en.pos(), Surround::Ortho)
//                 .into_iter()
//                 .filter(|(_, dir, c)| **c != '#' && !dir.is_reverse(en.dir()))
//                 .map(|(p, dir, _)| {
//                     (
//                         en.set_bounded(p, dir).unwrap(),
//                         tern!(dir == en.dir(), 1, 1001),
//                     )
//                 })
//                 .collect_vec()
//         },
//         |_| 0,
//     )
//     .next()
//     .unwrap()
// }

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();

    let start = grid.find(&'S').unwrap();
    let end = grid.find(&'E').unwrap();

    a_star_score(
        vec![Entity::new_on_grid(start, Dir::East, &grid).unwrap()],
        |en| en.pos() == end,
        |en| {
            grid.with_offsets(en.pos(), Dir::ORTHO)
                .all()
                .filter(|(_, dir, c)| **c != '#' && !dir.is_reverse_of(en.dir()))
                .map(|(p, dir, _)| {
                    (
                        en.set_bounded(p, dir).unwrap(),
                        tern!(dir == en.dir(), 1, 1001),
                    )
                })
                .collect_vec()
        },
        |en| {
            en.pos().manhattan_dist(end) as u32
                + match en.dir() {
                    Dir::North if en.pos().x == end.x => 0,
                    Dir::East if en.pos().y == end.y => 0,
                    Dir::North | Dir::East => 1000,
                    Dir::South | Dir::West => 2000,
                }
        },
    )
    .unwrap()
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_chars_transpose(input).unwrap();

    let start = grid.find(&'S').unwrap();
    let end = grid.find(&'E').unwrap();

    let mut finder = AStarMulti::new(
        vec![Entity::new_on_grid(start, Dir::East, &grid).unwrap()],
        |en| en.pos() == end,
        |en| {
            grid.with_offsets(en.pos(), Dir::ORTHO)
                .all()
                .filter(|(_, dir, c)| **c != '#' && !dir.is_reverse_of(en.dir()))
                .map(|(p, dir, _)| {
                    (
                        en.set_bounded(p, dir).unwrap(),
                        tern!(dir == en.dir(), 1, 1001),
                    )
                })
                .collect_vec()
        },
        |en| {
            en.pos().manhattan_dist(end) as u32
                + match en.dir() {
                    Dir::North if en.pos().x == end.x => 0,
                    Dir::East if en.pos().y == end.y => 0,
                    Dir::North | Dir::East => 1000,
                    Dir::South | Dir::West => 2000,
                }
        },
    );
    let end = finder.next().unwrap();
    finder
        .reconstruct(end)
        .reconstruct_paths()
        .into_iter()
        .flatten()
        .map(Entity::pos)
        .unique()
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 91464);
        assert_eq!(part2(input), 494);
    }
}
