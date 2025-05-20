use lib::{CountWhere, Grid, Point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn gol(grid: &mut Grid<bool>) {
    let mut swap = vec![];
    for (p, &c) in grid.enumerate() {
        let neighbors = grid.surrounding(p, lib::Surround::All).values().count_where(|c| *c);
        if (c && neighbors != 2 && neighbors != 3) || (!c && neighbors == 3) {
            swap.push(p);
        } 
    }
    for p in swap {
        grid.update(p, |c| !*c).unwrap();
    }
}

fn part1(input: &str) -> u32 {
    let mut grid = Grid::from_chars(input).unwrap().map(|c| c == '#');

    for _ in 0..100 {
        gol(&mut grid);
    }

    grid.count(&true) as u32
}

fn part2(input: &str) -> u32 {
    let mut grid = Grid::from_chars(input).unwrap().map(|c| c == '#');

    let c1 = Point2::new(0, 0);
    let c2 = Point2::new(grid.width() - 1, 0);
    let c3 = Point2::new(0, grid.height() - 1);
    let c4 = Point2::new(grid.width() - 1, grid.height() - 1);

    for _ in 0..100 {
        grid.set(c1, true);
        grid.set(c2, true);
        grid.set(c3, true);
        grid.set(c4, true);
        gol(&mut grid);
    }

    grid.set(c1, true);
    grid.set(c2, true);
    grid.set(c3, true);
    grid.set(c4, true);

    grid.count(&true) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 814);
        assert_eq!(part2(input), 924);
    }
}

