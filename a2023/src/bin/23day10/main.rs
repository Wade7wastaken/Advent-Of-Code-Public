use std::collections::HashSet;

use lib::{Dir, Entity, Grid, Point2, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

const STARTING_CONNECTIONS: [(Dir, [char; 3]); 4] = [
    (Dir::North, ['|', '7', 'F']),
    (Dir::South, ['|', 'L', 'J']),
    (Dir::East, ['-', 'J', '7']),
    (Dir::West, ['-', 'L', 'F']),
];

fn starting_entities(
    grid: &Grid<char>,
    starting_point: Point2<usize>,
) -> (Entity<usize>, Entity<usize>) {
    STARTING_CONNECTIONS
        .into_iter()
        .filter_map(|(dir, chars)| {
            let p = starting_point.apply(dir)?;
            if !chars.contains(grid.get(p)?) {
                return None;
            }
            Some(Entity::new_on_grid(p, dir, grid).unwrap())
        })
        .collect_tuple()
        .unwrap()
}

fn step(grid: &Grid<char>, en: Entity<usize>) -> Entity<usize> {
    let (pos, dir) = en.tuple();
    let new_dir = match grid.get(pos).unwrap() {
        '|' | '-' => dir, // continue in the same direction
        'L' => match dir {
            Dir::South => Dir::East, // coming in from north (going south)
            Dir::West => Dir::North, // coming in from east
            _ => unreachable!(),
        },
        'J' => match dir {
            Dir::South => Dir::West,
            Dir::East => Dir::North,
            _ => unreachable!(),
        },
        '7' => match dir {
            Dir::North => Dir::West,
            Dir::East => Dir::South,
            _ => unreachable!(),
        },
        'F' => match dir {
            Dir::North => Dir::East,
            Dir::West => Dir::South,
            _ => unreachable!(),
        },
        _ => unreachable!("{} \r\n {:?}", grid, pos),
    };

    en.set_dir(new_dir).step_bounded().unwrap()
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars_transpose(input).unwrap();

    let (mut en1, mut en2) = starting_entities(&grid, grid.find(&'S').unwrap());

    let mut num_steps = 1; // we have already taken one step

    while en1.pos() != en2.pos() {
        en1 = step(&grid, en1);
        en2 = step(&grid, en2);
        num_steps += 1;
    }

    num_steps
}

fn part2(input: &str) -> u32 {
    let mut grid = Grid::from_chars_transpose(input).unwrap();
    let starting_point = grid.find(&'S').unwrap();

    let (mut en1, mut en2) = starting_entities(&grid, starting_point);

    grid.set(
        starting_point,
        match (en1.dir(), en2.dir()) {
            // if the paths started going () and (), then set the starting point to the right char
            // order depends on which way find_starting_paths returned the starting paths
            (Dir::North, Dir::South) => '|',
            (Dir::East, Dir::West) => '-',
            (Dir::North, Dir::East) => 'L',
            (Dir::North, Dir::West) => 'J',
            (Dir::South, Dir::West) => '7',
            (Dir::South, Dir::East) => 'F',
            _ => unreachable!(),
        },
    );

    let mut points_on_loop = HashSet::new();
    points_on_loop.insert(starting_point);

    while en1.pos() != en2.pos() {
        points_on_loop.insert(en1.pos());
        points_on_loop.insert(en2.pos());
        en1 = step(&grid, en1);
        en2 = step(&grid, en2);
    }
    points_on_loop.insert(en1.pos()); // the loop will end before the ending point is added

    let mut num_inside = 0;
    let mut parity = false;
    let mut current_run: Option<char> = None;

    for (p, c) in grid.into_enumerate() {
        if points_on_loop.contains(&p) {
            match (c, current_run) {
                ('-', Some(_)) => {}
                ('|', None) => parity = !parity,
                ('F' | 'L', None) => current_run = Some(c),
                ('7', Some('F')) | ('J', Some('L')) => current_run = None, // don't change parity
                (_, Some('F' | 'L')) => {
                    parity = !parity;
                    current_run = None;
                }
                _ => unreachable!(),
            }
        } else if parity {
            num_inside += 1;
        }
    }

    num_inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 7145);
        assert_eq!(part2(input), 445);
    }
}
