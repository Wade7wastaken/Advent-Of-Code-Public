use lib::{abs_diff, Dir, Point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

struct Keypad {
    p: Point2<u8>,
}

impl Keypad {
    fn new() -> Self {
        Keypad {
            p: Point2 { x: 2, y: 3 },
        }
    }

    fn digit_to_position(digit: Option<u8>) -> Point2<u8> {
        match digit {
            None => Point2 { x: 2, y: 3 },
            Some(0) => Point2 { x: 1, y: 3 },
            Some(d) => {
                let y = 2 - ((d - 1) / 3);
                let x = (d - 1) % 3;
                Point2 { x, y }
            }
        }
    }

    fn instructions_to_type(&mut self, v: Vec<Option<u8>>) -> Vec<Option<Dir>> {
        let mut old = self.p;
        let mut output = vec![];
        for next in v {
            let new = Keypad::digit_to_position(next);
            let dx = abs_diff(old.x, new.x) as usize;
            let dy = abs_diff(old.y, new.y) as usize;
            if old.y > new.y {
                output.append(&mut vec![Some(Dir::NORTH); dy]);
            }
            if old.x < new.x {
                output.append(&mut vec![Some(Dir::EAST); dx]);
            }
            if old.y < new.y {
                output.append(&mut vec![Some(Dir::SOUTH); dy]);
            }
            if old.x > new.x {
                output.append(&mut vec![Some(Dir::WEST); dx]);
            }
            output.push(None);
            old = new;
        }

        output
    }
}

struct Dirpad {
    p: Point2<u8>,
}

impl Dirpad {
    fn new() -> Dirpad {
        Dirpad {
            p: Point2 { x: 2, y: 0 },
        }
    }

    fn arrow_to_position(digit: Option<Dir>) -> Point2<u8> {
        match digit {
            None => Point2 { x: 2, y: 0 },
            Some(Dir::SOUTH) => Point2 { x: 1, y: 1 },
            Some(d) => Point2::new(1, 1).apply(d).unwrap(),
        }
    }

    fn instructions_to_type(&mut self, v: Vec<Option<Dir>>) -> Vec<Option<Dir>> {
        let mut old = self.p;
        let mut output = vec![];
        for next in v {
            let new = Self::arrow_to_position(next);
            let dx = abs_diff(old.x, new.x) as usize;
            let dy = abs_diff(old.y, new.y) as usize;
            if old.y < new.y {
                output.append(&mut vec![Some(Dir::SOUTH); dy]);
            }
            if old.x < new.x {
                output.append(&mut vec![Some(Dir::EAST); dx]);
            }
            if old.y > new.y {
                output.append(&mut vec![Some(Dir::NORTH); dy]);
            }
            if old.x > new.x {
                output.append(&mut vec![Some(Dir::WEST); dx]);
            }
            output.push(None);
            old = new;
        }

        output
    }
}

fn print_digit(d: Option<Dir>) -> char {
    match d {
        None => 'A',
        Some(Dir::NORTH) => '^',
        Some(Dir::SOUTH) => 'v',
        Some(Dir::EAST) => '>',
        Some(Dir::WEST) => '<',
        _ => panic!(),
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let code = l
                .chars()
                .map(|c| match c {
                    'A' => None,
                    d => Some(d.to_digit(10).unwrap() as u8),
                })
                .collect();

            let mut k1 = Keypad::new();
            let out = k1.instructions_to_type(code);
            println!(
                "{}",
                out.clone().into_iter().map(print_digit).collect::<String>()
            );

            let mut k2 = Dirpad::new();
            let out = k2.instructions_to_type(out);
            println!(
                "{}",
                out.clone().into_iter().map(print_digit).collect::<String>()
            );

            let mut k3 = Dirpad::new();
            let out = k3.instructions_to_type(out);
            println!(
                "{}",
                out.clone().into_iter().map(print_digit).collect::<String>()
            );
            4
        })
        .sum()
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
