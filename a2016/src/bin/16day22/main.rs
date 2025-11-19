use lib::{Grid, Point2, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

#[derive(Debug, Clone)]
struct Node {
    x: u32,
    y: u32,
    used: u32,
    avail: u32,
}

fn parse_line(l: &str) -> Node {
    let (path, _, used, avail, _) = l.split_ascii_whitespace().collect_tuple().unwrap();
    let (_, x_str, y_str) = path.split('-').collect_tuple().unwrap();
    Node {
        x: x_str.strip_prefix('x').unwrap().parse().unwrap(),
        y: y_str.strip_prefix('y').unwrap().parse().unwrap(),
        used: used.strip_suffix('T').unwrap().parse().unwrap(),
        avail: avail.strip_suffix('T').unwrap().parse().unwrap(),
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .skip(2)
        .map(parse_line)
        .permutations(2)
        .map(|p| p.into_iter().collect_tuple().unwrap())
        .filter(|(a, b)| a.used != 0 && !(a.x == b.x && a.y == b.y) && a.used <= b.avail)
        .count() as u32
}

struct Cell {
    used: u32,
    avail: u32,
}

struct State {
    grid: Grid<Cell>,
    goal: Point2<usize>,
}

fn part2(input: &str) -> u32 {
    // input.lines().map(parse_line).map(|n| Cell {used: n.used, avail: n.avail})
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
