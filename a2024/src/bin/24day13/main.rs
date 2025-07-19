use lib::{Point2, StringTools, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct ClawMachine {
    prize: Point2<i64>,
    a: Point2<i64>,
    b: Point2<i64>,
}

fn parse_button(input: &str) -> Point2<i64> {
    let data = input.split_once(": ").unwrap().1;
    let end = data.chars().position(|c| c == ',').unwrap();
    let x = data.get(2..end).unwrap().parse().unwrap();
    let y = data.get(end + 4..).unwrap().parse().unwrap();
    Point2::new(x, y)
}

fn parse_claw_machine(input: &str, add: bool) -> ClawMachine {
    let (a_str, b_str, prize_str) = input.lines().collect_tuple().unwrap();
    let a = parse_button(a_str);
    let b = parse_button(b_str);
    let mut prize = parse_button(prize_str);
    if add {
        prize += Point2::new(10000000000000, 10000000000000);
    }
    ClawMachine { prize, a, b }
}

fn press_buttons(ClawMachine { prize, a, b }: ClawMachine) -> Option<(i64, i64)> {
    let x_numer = b.x * prize.y - prize.x * b.y;
    let denom = b.x * a.y - a.x * b.y;
    if x_numer % denom != 0 {
        return None;
    }
    let x = x_numer / denom;

    let y_numer = prize.x * a.y - a.x * prize.y;
    if y_numer % denom != 0 {
        return None;
    }
    let y = y_numer / denom;
    Some((x, y))
}

fn solve(input: &str, add: bool) -> i64 {
    input
        .paragraphs()
        .map(|input| parse_claw_machine(input, add))
        .filter_map(press_buttons)
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn part1(input: &str) -> i64 {
    solve(input, false)
}

fn part2(input: &str) -> i64 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 36838);
        assert_eq!(part2(input), 83029436920891);
    }
}
