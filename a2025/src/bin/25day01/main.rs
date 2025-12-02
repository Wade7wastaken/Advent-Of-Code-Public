fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

// 2316 low

fn part1(input: &str) -> i32 {
    let mut dial = 50;

    let mut at_0 = 0;

    for inst in input.lines() {
        let (a, b) = inst.split_at(1);
        let n = b.parse::<i32>().unwrap();

        match a {
            "R" => {
                dial += n;
                dial = dial.rem_euclid(100);
            }
            "L" => {
                dial -= n;
                dial = dial.rem_euclid(100);
            }
            _ => panic!(),
        }
        if dial == 0 {
            at_0 += 1;
        }
    }

    at_0
}

fn part2(input: &str) -> u32 {
    let mut dial = 50i32;

    let mut at_0 = 0;

    for inst in input.lines() {
        let (a, b) = inst.split_at(1);
        let n = b.parse::<i32>().unwrap();

        match a {
            "R" => {
                for _ in 0..n {
                    dial += 1;
                    dial = dial.rem_euclid(100);
                    if dial == 0 {
                        at_0 += 1;
                    }
                }
            }
            "L" => {
                for _ in 0..n {
                    dial -= 1;
                    dial = dial.rem_euclid(100);
                    if dial == 0 {
                        at_0 += 1;
                    }
                }
            }
            _ => panic!(),
        }
    }

    at_0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1158);
        assert_eq!(part2(input), 6860);
    }
}
