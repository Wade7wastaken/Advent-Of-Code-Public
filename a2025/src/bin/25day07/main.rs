use lib::{Grid, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut iter = input.lines();
    let first = iter.next().unwrap();
    let width = first.len();
    let starting = first.bytes().position(|x| x == b'S').unwrap();
    let mut beam = vec![false; width];
    beam[starting] = true;

    let mut splits = 0;

    for l in iter {
        let mut new_beam = vec![false; width];
        for (i, (ch, b)) in l.bytes().zip_eq(beam).enumerate() {
            if b {
                if ch == b'^' && b {
                    splits += 1;
                    new_beam[i + 1] = true;
                    new_beam[i - 1] = true;
                } else {
                    new_beam[i] = true;
                }
            }
        }
        beam = new_beam;
    }

    splits
}

fn part2(input: &str) -> u64 {
    let grid = Grid::from_bytes(input).unwrap();
    let start = grid
        .row(0)
        .unwrap()
        .iter()
        .position(|x| *x == b'S')
        .unwrap();

    let mut dp_y1 = vec![1; grid.width()];

    for y in (1..grid.height() - 1).rev() {
        let mut dp_y = vec![0; grid.width()];
        for x in 0..grid.width() {
            let res = if *grid.get((x, y)).unwrap() == b'^' {
                dp_y1[x + 1] + dp_y1[x - 1]
            } else {
                dp_y1[x]
            };
            dp_y[x] = res;
        }
        dp_y1 = dp_y;
    }

    dp_y1[start]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1566);
        assert_eq!(part2(input), 5921061943075);
    }
}
