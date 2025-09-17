use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn scramble(instructions: &str, mut s: String) -> String {
    for l in instructions.lines() {
        let bytes = unsafe { s.as_bytes_mut() };
        match l.split_ascii_whitespace().collect_vec()[..] {
            // swap position
            ["swap", "position", x, _, _, y] => {
                bytes.swap(x.parse().unwrap(), y.parse().unwrap());
            }
            // swap letter
            [_, "letter", x, _, _, y] => {
                s = s.replace(x, " ").replace(y, x).replace(' ', y);
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

fn part1(input: &str) -> String {
    scramble(input, "abcdefgh".to_string())
}

fn part2(input: &str) -> String {
    "abcdefgh"
        .bytes()
        .permutations(8)
        .map(|x| String::from_utf8(x).unwrap())
        .find(|perm| scramble(input, perm.clone()) == "fbgdceah")
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "bfheacgd");
        assert_eq!(part2(input), "gcehdbfa");
    }
}
