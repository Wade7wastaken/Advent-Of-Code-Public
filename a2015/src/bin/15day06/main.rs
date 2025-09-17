use lib::{Grid, Point2, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

enum Operation {
    On,
    Off,
    Toggle,
}

fn parse_operation(op: &str) -> Operation {
    match op {
        "turn on" => Operation::On,
        "turn off" => Operation::Off,
        "toggle" => Operation::Toggle,
        _ => panic!(),
    }
}

fn parse_point(p: &str) -> Point2<usize> {
    p.split(',')
        .map(|p| p.parse().unwrap())
        .collect_tuple::<(_, _)>()
        .unwrap()
        .into()
}

fn parse_command(l: &str) -> (Point2<usize>, Point2<usize>, Operation) {
    let (start_command_str, end_str) = l.split_once(" through ").unwrap();
    let (operation_str, start_str) = start_command_str.rsplit_once(' ').unwrap();
    (
        parse_point(start_str),
        parse_point(end_str),
        parse_operation(operation_str),
    )
}

fn part1(input: &str) -> u32 {
    let mut grid = Grid::new_filled(false, 1000, 1000);
    for (start, end, operation) in input.lines().map(parse_command) {
        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let p: Point2<usize> = (x, y).into();
                match operation {
                    Operation::On => {
                        grid.set(p, true).unwrap();
                    }
                    Operation::Off => {
                        grid.set(p, false).unwrap();
                    }
                    Operation::Toggle => {
                        grid.update(p, |c| !c);
                    }
                }
            }
        }
    }
    grid.count(&true) as u32
}

fn part2(input: &str) -> u32 {
    let mut grid = Grid::new_filled(0u8, 1000, 1000);
    for (start, end, operation) in input.lines().map(parse_command) {
        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let p: Point2<usize> = (x, y).into();
                match operation {
                    Operation::On => {
                        grid.update(p, |c| c.saturating_add(1));
                    }
                    Operation::Off => {
                        grid.update(p, |c| c.saturating_sub(1));
                    }
                    Operation::Toggle => {
                        grid.update(p, |c| c.saturating_add(2));
                    }
                }
            }
        }
    }
    grid.into_iter().map(u32::from).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 377891);
        assert_eq!(part2(input), 14110788);
    }
}
