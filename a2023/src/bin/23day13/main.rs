use std::iter::zip;

use lib::Grid;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn find_reflection_line<'a>(
    num_dims: usize,
    get_dim: impl Fn(usize) -> &'a Vec<char>,
) -> Option<usize> {
    (0..num_dims - 1).find(|&d| {
        // (d, d - 1 ... 0) zipped with (d + 1, d + 2 ... height - 1)
        // zip ends at the shorter iter, so it should always be in bounds
        zip((0..=d).rev(), d + 1..num_dims).all(|(low, high)| get_dim(low) == get_dim(high))
    })
}

fn find_reflection_line_with_smudge<'a>(
    num_dims: usize,
    get_dim: impl Fn(usize) -> &'a Vec<char>,
) -> Option<usize> {
    (0..num_dims - 1).find(|&d| {
        let mut used_smudge = false;
        (0..=d).rev().zip(d + 1..num_dims).all(|(low, high)| {
            zip(get_dim(low), get_dim(high)).all(|(a, b)| {
                if used_smudge {
                    a == b
                } else {
                    if a != b {
                        used_smudge = true;
                    }
                    true
                }
            })
        }) && used_smudge
    })
}

fn part1(input: &str) -> u32 {
    input
        .split("\r\n\r\n")
        .map(|g| {
            let grid = Grid::from_chars_transpose(g).unwrap();
            let get_row = |row| grid.row(row).unwrap();
            let get_col = |col| grid.col(col).unwrap();
            find_reflection_line(grid.height(), get_row)
                .map(|i| ((i + 1) * 100))
                .or_else(|| find_reflection_line(grid.width(), get_col).map(|i| (i + 1)))
                .unwrap()
        })
        .sum::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    input
        .split("\r\n\r\n")
        .map(|g| {
            let grid = Grid::from_chars_transpose(g).unwrap();
            let get_row = |row| grid.row(row).unwrap();
            let get_col = |col| grid.col(col).unwrap();
            find_reflection_line_with_smudge(grid.height(), get_row)
                .map(|i| ((i + 1) * 100))
                .or_else(|| {
                    find_reflection_line_with_smudge(grid.width(), get_col).map(|i| (i + 1))
                })
                .unwrap()
        })
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 30575);
        assert_eq!(part2(input), 37478);
    }
}
