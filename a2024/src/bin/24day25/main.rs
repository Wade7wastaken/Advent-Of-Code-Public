use lib::{itertools::Itertools, Grid, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut locks = vec![];
    let mut keys = vec![];
    for s in input.paragraphs() {
        let grid = Grid::from_bytes_transpose(s).unwrap();
        let height = grid.height();
        let is_key = *grid.get((0, 0)).unwrap() == b'.';
        if is_key {
            let key = grid
                .into_cols()
                .into_iter()
                .map(|col| height - 1 - col.into_iter().position(|c| c == b'#').unwrap())
                .collect_vec();
            keys.push((key, height));
        } else {
            let lock = grid
                .into_cols()
                .into_iter()
                .map(|col| col.into_iter().position(|c| c == b'.').unwrap() - 1)
                .collect_vec();
            locks.push((lock, height));
        }
    }

    let mut counter = 0;

    for lock in locks {
        for key in &keys {
            assert_eq!(lock.1, key.1);
            if lock
                .0
                .iter()
                .zip(key.0.iter())
                .all(|(l, k)| l + k < lock.1 - 1)
            {
                counter += 1;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 3356);
    }
}
