use std::collections::HashMap;

use lib::{CollectString, Grid, itertools::Itertools, select};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_rotate(command: &str) -> (usize, usize) {
    let (row_str, amount_str) = select!(command.split_ascii_whitespace(); 0, 2);
    (
        row_str.split_once('=').unwrap().1.parse().unwrap(),
        amount_str.parse().unwrap(),
    )
}

fn run_commands(commands: &str) -> Grid<bool> {
    let mut grid = Grid::new(vec![vec![false; 50]; 6]).unwrap();
    for l in commands.lines() {
        if let Some(dim) = l.strip_prefix("rect ") {
            let (i, j) = dim
                .split('x')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            for y in 0..j {
                for x in 0..i {
                    grid.set((x, y), true).unwrap();
                }
            }
        } else if let Some(command) = l.strip_prefix("rotate row ") {
            let (row, amount) = parse_rotate(command);
            grid.retranspose_rows();
            grid.row_mut(row).unwrap().rotate_right(amount);
        } else if let Some(command) = l.strip_prefix("rotate column ") {
            let (row, amount) = parse_rotate(command);
            grid.retranspose_cols();
            grid.col_mut(row).unwrap().rotate_right(amount);
        }
    }
    grid
}

fn part1(input: &str) -> u32 {
    run_commands(input).count(&true) as u32
}

const C_DATA: &str = "
.##..
#..#.
#....
#....
#..#.
.##..";
const E_DATA: &str = "
####.
#....
###..
#....
#....
####.";
const F_DATA: &str = "
####.
#....
###..
#....
#....
#....";
const L_DATA: &str = "
#....
#....
#....
#....
#....
####.";
const O_DATA: &str = "
.##..
#..#.
#..#.
#..#.
#..#.
.##..";
const S_DATA: &str = "
.###.
#....
#....
.##..
...#.
###..";
const Y_DATA: &str = "
#...#
#...#
.#.#.
..#..
..#..
..#..";

fn recognize_text(grid: &Grid<bool>) -> String {
    fn create_grid(data: &str) -> Grid<bool> {
        Grid::from_bytes(data.trim()).unwrap().map(|c| c == b'#')
    }
    let map = HashMap::from([
        (create_grid(C_DATA), b'C'),
        (create_grid(E_DATA), b'E'),
        (create_grid(F_DATA), b'F'),
        (create_grid(L_DATA), b'L'),
        (create_grid(O_DATA), b'O'),
        (create_grid(S_DATA), b'S'),
        (create_grid(Y_DATA), b'Y'),
    ]);
    assert!(grid.width() % 5 == 0);
    (0..grid.width())
        .step_by(5)
        .map(|i| {
            let subgrid = grid.subgrid(i, 0, 5, 6);
            *map.get(&subgrid).unwrap()
        })
        .collect_string()
}

fn part2(input: &str) -> String {
    recognize_text(&run_commands(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 106);
        assert_eq!(part2(input), "CFLELOYFCS");
    }
}
