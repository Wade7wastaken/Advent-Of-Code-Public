use lib::{a_star_score, Dir, Entity, Grid, Point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

type NeighborList = Vec<(Entity<usize>, u32)>;

fn next_crucibles(en: &Entity<usize>, grid: &Grid<u32>) -> NeighborList {
    let mut en = *en;
    let mut res = Vec::with_capacity(6);
    let mut loss = 0;
    for _ in 0..3 {
        en = match en.step_bounded() {
            None => break,
            Some(x) => x,
        };
        loss += grid.get(en).unwrap();
        res.push((en.turn_left(), loss));
        res.push((en.turn_right(), loss));
    }
    res
}

fn next_ultra_crucibles(en: &Entity<usize>, grid: &Grid<u32>) -> NeighborList {
    let mut en = *en;
    let mut res = Vec::with_capacity(14);
    let mut loss = 0;
    for i in 0..10 {
        en = match en.step_bounded() {
            None => break,
            Some(x) => x,
        };
        loss += grid.get(en).unwrap();
        if i >= 3 {
            res.push((en.turn_left(), loss));
            res.push((en.turn_right(), loss));
        }
    }
    res
}

fn shortest_path(input: &str, next: fn(&Entity<usize>, &Grid<u32>) -> NeighborList) -> u32 {
    let grid = Grid::from_chars_transpose(input)
        .unwrap()
        .map(|c| c.to_digit(10).unwrap());

    let end = Point2::new(grid.width() - 1, grid.height() - 1);

    let starting = vec![
        Entity::new_on_grid((0, 0), Dir::East, &grid).unwrap(),
        Entity::new_on_grid((0, 0), Dir::South, &grid).unwrap(),
    ];

    a_star_score(
        starting,
        |en| en.pos() == end,
        |en| next(en, &grid),
        |en| en.pos().manhattan_dist(end) as u32,
    )
    .unwrap()
}

fn part1(input: &str) -> u32 {
    shortest_path(input, next_crucibles)
}

fn part2(input: &str) -> u32 {
    shortest_path(input, next_ultra_crucibles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 916);
        assert_eq!(part2(input), 1067);
    }
}
