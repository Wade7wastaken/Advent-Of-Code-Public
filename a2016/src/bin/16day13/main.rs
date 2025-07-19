use std::collections::HashMap;

use lib::{AStarScore, Dir, Point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn is_open(p: Point2<usize>, n: usize) -> bool {
    let Point2 { x, y } = p;
    (x * x + 3 * x + 2 * x * y + y + y * y + n).count_ones() % 2 == 0
}

fn neighbors(p: Point2<usize>, n: usize) -> impl Iterator<Item = Point2<usize>> {
    Dir::ORTHO
        .into_iter()
        .filter_map(move |dir| p.apply(dir))
        .filter(move |p| is_open(*p, n))
}

fn part1(input: &str) -> u32 {
    let finish = Point2::new(31, 39);
    let n = input.parse().unwrap();
    AStarScore::new(
        vec![Point2::new(1, 1)],
        |p| p == &finish,
        |p| neighbors(*p, n).map(|p| (p, 1)),
        |p| p.manhattan_dist(finish) as u32,
    )
    .first()
}

fn step(p: Point2<usize>, n: usize, depth: usize, ends: &mut HashMap<Point2<usize>, usize>) {
    if let Some(depth_encountered) = ends.get(&p) {
        // we already found a faster way to get to this point
        if *depth_encountered >= depth {
            return;
        }
    }
    ends.insert(p, depth);
    if depth == 0 {
        return;
    }
    for next in neighbors(p, n) {
        step(next, n, depth - 1, ends);
    }
}

fn part2(input: &str) -> u32 {
    let n = input.parse().unwrap();
    let mut ends = HashMap::new();
    step(Point2::new(1, 1), n, 50, &mut ends);
    ends.len() as u32
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
