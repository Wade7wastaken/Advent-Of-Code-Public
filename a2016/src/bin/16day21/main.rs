use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

fn part1(input: &str) -> String {
    let mut s = "abcdefgh".to_string();
    for l in input.lines() {
        let bytes = unsafe { s.as_bytes_mut() };
        match l.split_ascii_whitespace().collect_vec()[..] {
            // swap position
            ["swap", "position", x, _, _, y] => {
                bytes.swap(x.parse().unwrap(), y.parse().unwrap());
            }
            // swap letter
            [_, "letter", x, _, _, y] => {
                s = s.replace(x, " ").replace(y, x).replace(" ", y);
            }
            // rotate x steps
            [_, dir, x, _] => {
                if dir == "left" {
                    bytes.rotate_left(x.parse().unwrap());
                } else {
                    bytes.rotate_right(x.parse().unwrap());
                }
            }
            // rotate based on x
            [_, _, _, _, _, _, x] => {
                let mut i = s.find(x).unwrap();
                let len = s.len();
                let bytes = unsafe { s.as_bytes_mut() };
                if i >= 4 {
                    i += 1;
                }
                i += 1;
                bytes.rotate_right(i % len);
            }
            // reverse positions
            [_, _, x, _, y] => {
                bytes[x.parse().unwrap()..=y.parse().unwrap()].reverse();
            }
            // move position
            ["move", _, x, _, _, y] => {
                let ch = s.remove(x.parse().unwrap());
                s.insert(y.parse().unwrap(), ch);
            }
            _ => panic!("unknown command {l}"),
        }
    }
    s
}

fn part2(input: &str) -> String {
    let mut s = "fbgdceah".to_string();
    for l in input.lines().rev() {
        let bytes = unsafe { s.as_bytes_mut() };
        match l.split_ascii_whitespace().collect_vec()[..] {
            // swap position
            ["swap", "position", x, _, _, y] => {
                bytes.swap(y.parse().unwrap(), x.parse().unwrap());
            }
            // swap letter
            [_, "letter", x, _, _, y] => {
                s = s.replace(y, " ").replace(x, y).replace(" ", x);
            }
            // rotate x steps
            [_, dir, x, _] => {
                if dir == "left" {
                    bytes.rotate_right(x.parse().unwrap());
                } else {
                    bytes.rotate_left(x.parse().unwrap());
                }
            }
            // rotate based on x
            [_, _, _, _, _, _, x] => {
                let mut i = s.find(x).unwrap();
                let len = s.len();
                let bytes = unsafe { s.as_bytes_mut() };
                if i >= 4 {
                    i += 1;
                }
                i += 1;
                bytes.rotate_right(i % len);
            }
            // reverse positions
            [_, _, x, _, y] => {
                bytes[y.parse().unwrap()..=x.parse().unwrap()].reverse();
            }
            // move position
            ["move", _, x, _, _, y] => {
                let ch = s.remove(y.parse().unwrap());
                s.insert(x.parse().unwrap(), ch);
            }
            _ => panic!("unknown command {l}"),
        }
    }
    s
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
