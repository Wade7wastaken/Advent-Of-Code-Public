use std::collections::HashMap;

use lib::{Dir, Point2, a_star_score, point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

const fn is_open(p: Point2<usize>, n: usize) -> bool {
    let Point2 { x, y } = p;
    (x * x + 3 * x + 2 * x * y + y + y * y + n)
        .count_ones()
        .is_multiple_of(2)
}

fn neighbors(p: Point2<usize>, n: usize) -> impl Iterator<Item = Point2<usize>> {
    Dir::ORTHO
        .into_iter()
        .filter_map(move |dir| p.apply(dir))
        .filter(move |p| is_open(*p, n))
}

const GOAL: Point2<usize> = point2(31, 39);

fn part1(input: &str) -> u32 {
    let n = input.parse().unwrap();
    a_star_score(
        vec![Point2::new(1, 1)],
        |p| *p == GOAL,
        |p| neighbors(*p, n).map(|p| (p, 1)),
        |p| p.manhattan_dist(GOAL) as u32,
    )
    .unwrap()
}

fn step(p: Point2<usize>, n: usize, depth: usize, seen: &mut HashMap<Point2<usize>, usize>) {
    if let Some(depth_encountered) = seen.get(&p)
        && *depth_encountered >= depth
    {
        // we already found a faster way to get to this point
        return;
    }
    seen.insert(p, depth);
    if depth == 0 {
        return;
    }
    for next in neighbors(p, n) {
        step(next, n, depth - 1, seen);
    }
}

fn part2(input: &str) -> u32 {
    let n = input.parse().unwrap();
    let mut seen = HashMap::new();
    step(Point2::new(1, 1), n, 50, &mut seen);
    seen.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 96);
        assert_eq!(part2(input), 141);
    }
}
