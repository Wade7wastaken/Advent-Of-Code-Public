use std::collections::HashSet;

use lib::{Dir, Entity, Grid, Point2, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();
    let starting_position = grid.find(&'^').unwrap();
    let mut en = Entity::new_on_grid(starting_position, Dir::North, &grid).unwrap();
    let mut seen = HashSet::new();

    while let Some(moved) = en.step_bounded() {
        seen.insert(en.pos());
        en = tern!(
            *grid.get(moved.pos()).unwrap() == '#',
            en.turn_right(),
            moved
        );
    }
    // we never add the initial position to seen, so +1
    seen.len() as u32 + 1
}

fn step(grid: &Grid<char>, en: Entity<usize>, obs: Option<Point2<usize>>) -> Option<Entity<usize>> {
    en.step_bounded().map(|moved| {
        tern!(
            *grid.get(moved.pos()).unwrap() == '#' || Some(moved.pos()) == obs,
            en.turn_right(),
            moved
        )
    })
}

fn possible_obstacle(
    grid: &Grid<char>,
    en: Entity<usize>,
    seen: &HashSet<Entity<usize>>,
    seen_pos: &HashSet<Point2<usize>>,
    start: Point2<usize>,
) -> Option<Point2<usize>> {
    en.step_bounded()
        .filter(|obstacle| {
            obstacle.pos() != start
                && *grid.get(obstacle.pos()).unwrap() != '#'
                && !seen_pos.contains(&obstacle.pos())
        })
        .and_then(|obstacle| {
            let mut test_en = en.turn_right();
            let mut after_obstacle = HashSet::new();
            while let Some(next) = step(grid, test_en, Some(obstacle.pos())) {
                if after_obstacle.contains(&next) || seen.contains(&next) {
                    return Some(obstacle.pos());
                }
                after_obstacle.insert(next);

                test_en = next;
            }
            None
        })
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_chars(input).unwrap();
    let start = grid.find(&'^').unwrap();
    let mut en = Entity::new_on_grid(start, Dir::North, &grid).unwrap();
    let mut seen = HashSet::new();
    let mut seen_pos = HashSet::new();
    let mut obstacles = HashSet::new();

    seen.insert(en);
    seen_pos.insert(en.pos());

    while let Some(moved) = step(&grid, en, None) {
        if let Some(obstacle) = possible_obstacle(&grid, en, &seen, &seen_pos, start) {
            obstacles.insert(obstacle);
        }

        seen.insert(en);
        seen_pos.insert(en.pos());

        en = moved;
    }

    obstacles.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 4776);
        assert_eq!(part2(input), 1586);
    }
}
