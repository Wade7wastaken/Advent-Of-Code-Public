use lib::{Grid, IteratorExt, Vec2, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_bytes(input).unwrap();
    let mut ans = 0;
    for (p, c) in grid.enumerate() {
        if *c == b'.' {
            continue;
        }
        let count = Vec2::SURROUNDING
            .into_iter()
            .filter_map(|v| p.apply(v).and_then(|a| grid.get(a)))
            .count_where(|a| *a == b'@');
        if count < 4 {
            ans += 1;
        }
    }
    ans
}

fn part2(input: &str) -> u32 {
    let mut grid = Grid::from_bytes(input).unwrap();
    let mut ans = 0;
    let mut removed_any = true;

    while removed_any {
        removed_any = false;
        for (p, c) in grid.enumerate().map(|(a, b)| (a, *b)).collect_vec() {
            if c == b'.' {
                continue;
            }
            let count = Vec2::SURROUNDING
                .into_iter()
                .filter_map(|v| p.apply(v).and_then(|a| grid.get(a)))
                .count_where(|a| *a == b'@');
            if count < 4 {
                ans += 1;
                removed_any = true;
                grid.set(p, b'.');
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1564);
        assert_eq!(part2(input), 9401);
    }
}
