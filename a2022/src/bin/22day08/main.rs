use lib::{Grid, IteratorExt};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut grid = Grid::from_chars_transpose(input)
        .unwrap()
        .map(|c| (c.to_digit(10).unwrap() as i32, false));
    for row in grid.rows_mut() {
        let mut highest = -1;

        for (c, visible) in row.iter_mut() {
            if *c > highest {
                *visible = true;
                highest = *c;
            }
        }

        highest = -1;
        for (c, visible) in row.iter_mut().rev() {
            if *c > highest {
                *visible = true;
                highest = *c;
            }
        }
    }

    for cols in grid.cols_mut() {
        let mut highest = -1;

        for (c, visible) in cols.iter_mut() {
            if *c > highest {
                *visible = true;
                highest = *c;
            }
        }

        highest = -1;
        for (c, visible) in cols.iter_mut().rev() {
            if *c > highest {
                *visible = true;
                highest = *c;
            }
        }
    }

    println!("{}", grid.map(|c| c.1));

    // grid.into_iter().count_where(|(_, visible)| visible) as u32

    4
}

fn part2(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        // assert_eq!(part1(input), todo!());
        // assert_eq!(part2(input), todo!());
    }
}
