use lib::{Grid, IteratorExt, Offset, Point2, Vec2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn next_char(grid: &Grid<u8>, p: Point2<usize>, offset: Vec2, ch: u8) -> Option<Point2<usize>> {
    p.apply(offset)
        .filter(|q| grid.get(*q).filter(|c| **c == ch).is_some())
}

fn words_starting_from(grid: &Grid<u8>, p: Point2<usize>) -> u32 {
    Vec2::SURROUNDING.into_iter().count_where(|dirs| {
        next_char(grid, p, dirs, b'M')
            .and_then(|p| next_char(grid, p, dirs, b'A'))
            .and_then(|p| next_char(grid, p, dirs, b'S'))
            .is_some()
    }) as u32
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_bytes(input).unwrap();

    grid.enumerate()
        .filter(|(_, c)| **c == b'X')
        .map(|(p, _)| words_starting_from(&grid, p))
        .sum()
}

fn check_diagonal(grid: &Grid<u8>, p: Point2<usize>, dir: Vec2) -> bool {
    let reversed = grid.get_offset(p, dir.reverse());

    match grid.get_offset(p, dir) {
        Some(b'M') => reversed == Some(&b'S'),
        Some(b'S') => reversed == Some(&b'M'),
        _ => false,
    }
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_bytes(input).unwrap();

    grid.enumerate().count_where(|(p, &c)| {
        c == b'A'
            && check_diagonal(&grid, p, Vec2::new(1, -1))
            && check_diagonal(&grid, p, Vec2::new(-1, -1))
    }) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 2618);
        assert_eq!(part2(input), 2011);
    }
}
