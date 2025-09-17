use lib::{Dir, Entity, Grid, Offset};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Clone)]
struct Cell {
    c: char,
    visited: [bool; 4],
}

fn reflect_tilted_mirror(dir: Dir) -> Dir {
    match dir {
        Dir::North => Dir::East,
        Dir::South => Dir::West,
        Dir::East => Dir::North,
        Dir::West => Dir::South,
    }
}

fn step(grid: &mut Grid<Cell>, en: Entity<usize, Dir>) -> Vec<Entity<usize, Dir>> {
    grid.get_mut(en.pos()).unwrap().visited[en.dir().idx()] = true;

    let new_dirs = match (grid.get(en.pos()).unwrap().c, en.dir()) {
        ('.', _) => vec![en.dir()],
        ('|', Dir::North | Dir::South) => vec![en.dir()],
        ('|', _) => vec![Dir::North, Dir::South],
        ('-', Dir::East | Dir::West) => vec![en.dir()],
        ('-', _) => vec![Dir::East, Dir::West],
        ('/', _) => vec![reflect_tilted_mirror(en.dir())],
        ('\\', _) => vec![reflect_tilted_mirror(en.dir()).reverse()],
        _ => unreachable!(),
    };

    new_dirs
        .into_iter()
        .filter_map(|new_dir| en.set_dir(new_dir).step_bounded())
        .filter(|en| !grid.get(en.pos()).unwrap().visited[en.dir().idx()])
        .collect()
}

fn beam(grid: &mut Grid<Cell>, starting_en: Entity<usize>) -> u32 {
    let mut entities = vec![starting_en];

    while let Some(entity) = entities.pop() {
        entities.append(&mut step(grid, entity));
    }

    grid.count_where(|c| c.visited.into_iter().any(|d| d)) as u32
}

fn part1(input: &str) -> u32 {
    let mut grid = Grid::from_chars_transpose(input).unwrap().map(|c| Cell {
        c,
        visited: [false; 4],
    });

    let en = Entity::new_on_grid((0, 0), Dir::East, &grid).unwrap();

    beam(&mut grid, en)
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_double_iter_transpose(input.lines().map(|l| {
        l.chars().map(|c| Cell {
            c,
            visited: [false; 4],
        })
    }))
    .unwrap();

    let w = grid.width();
    let h = grid.height();

    (0..h)
        .map(|y| Entity::new_on_grid((0, y), Dir::East, &grid).unwrap())
        .chain((0..h).map(|y| Entity::new_on_grid((w - 1, y), Dir::West, &grid).unwrap()))
        .chain((0..w).map(|x| Entity::new_on_grid((x, 0), Dir::South, &grid).unwrap()))
        .chain((0..w).map(|x| Entity::new_on_grid((x, h - 1), Dir::North, &grid).unwrap()))
        .map(|en| beam(&mut grid.clone(), en))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 7210);
        assert_eq!(part2(input), 7673);
    }
}
