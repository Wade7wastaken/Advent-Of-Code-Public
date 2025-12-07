use lib::{CollectDigits, Grid, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn process(iter: impl Iterator<Item = u64>, op: &str) -> u64 {
    match op {
        "+" => iter.sum::<u64>(),
        "*" => iter.product(),
        _ => panic!(),
    }
}

fn part1(input: &str) -> u64 {
    Grid::from_double_iter_transpose(input.lines().map(|l| l.split_ascii_whitespace()))
        .unwrap()
        .into_cols()
        .into_iter()
        .map(|r| {
            process(
                r[..r.len() - 1].iter().map(|s| s.parse::<u64>().unwrap()),
                r.last().unwrap(),
            )
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let grid = Grid::from_chars_transpose(input).unwrap();

    let mut x = 0;

    let mut problems = grid
        .cols()
        .iter()
        .enumerate()
        .filter(|col| col.1.iter().all(|b| *b == ' '))
        .map(|col| {
            let res = grid.subgrid(x, 0, col.0 - x, grid.height());
            x = col.0 + 1;
            res
        })
        .collect_vec();

    problems.push(grid.subgrid(x, 0, grid.width() - x, grid.height()));

    problems
        .into_iter()
        .map(|p| {
            process(
                (0..p.width()).rev().map(|x| {
                    u64::from(
                        (0..p.height() - 1)
                            .filter_map(|y| p.get((x, y)).unwrap().to_digit(10))
                            .collect_digits(),
                    )
                }),
                &p.get((0, p.height() - 1)).unwrap().to_string(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt");
        assert_eq!(part1(input), 5524274308182);
        assert_eq!(part2(input), 8843673199391);
    }
}
