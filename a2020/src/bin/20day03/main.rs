use lib::Grid;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn check_slope(grid: &Grid<u8>, right: usize, down: usize) -> u64 {
    let mut x = right;
    let mut y = down;
    let mut count = 0;
    while y < grid.height() {
        if *grid.get((x, y)).unwrap() == b'#' {
            count += 1;
        }
        x += right;
        x %= grid.width();
        y += down;
    }
    count
}

fn part1(input: &str) -> u64 {
    let grid = Grid::from_bytes(input).unwrap();

    check_slope(&grid, 3, 1)
}

fn part2(input: &str) -> u64 {
    let grid = Grid::from_bytes(input).unwrap();

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| check_slope(&grid, right, down))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 207);
        assert_eq!(part2(input), 2655892800);
    }
}
