use lib::{Grid, itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut grid = Grid::new(vec![vec![false; 50]; 6]).unwrap();
    for l in input.lines() {
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
            let (row_s, _, amount_s) = command.split_ascii_whitespace().collect_tuple().unwrap();
            let row = row_s.split_once('=').unwrap().1.parse().unwrap();
            let amount = amount_s.parse().unwrap();
            grid.retranspose_rows();
            grid.row_mut(row).unwrap().rotate_right(amount);
        } else if let Some(command) = l.strip_prefix("rotate column ") {
            let (row_s, _, amount_s) = command.split_ascii_whitespace().collect_tuple().unwrap();
            let row = row_s.split_once('=').unwrap().1.parse().unwrap();
            let amount = amount_s.parse().unwrap();
            grid.retranspose_cols();
            grid.col_mut(row).unwrap().rotate_right(amount);
        }
    }

    grid.retranspose_rows();
    grid.count(&true) as u32
}

fn part2(input: &str) -> String {
    let mut grid = Grid::new_transpose(vec![vec![false; 50]; 6]).unwrap();
    for l in input.lines() {
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
            let (row_s, _, amount_s) = command.split_ascii_whitespace().collect_tuple().unwrap();
            let row = row_s.split_once('=').unwrap().1.parse().unwrap();
            let amount = amount_s.parse().unwrap();
            grid.row_mut(row).unwrap().rotate_right(amount);
            grid.retranspose();
        } else if let Some(command) = l.strip_prefix("rotate column ") {
            let (row_s, _, amount_s) = command.split_ascii_whitespace().collect_tuple().unwrap();
            let row = row_s.split_once('=').unwrap().1.parse().unwrap();
            let amount = amount_s.parse().unwrap();
            grid.col_mut(row).unwrap().rotate_right(amount);
            grid.retranspose();
        }
    }

    println!("{}", grid.map(|c| tern!(c, '#', '.')));
    "CFLELOYFCS".to_string()
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
