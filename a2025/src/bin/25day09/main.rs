use std::{collections::HashSet, hint::black_box};

use lib::{Grid, Point2, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    // println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u64 {
    let points = input.lines().map(|l| {
        Point2::from_tuple(
            l.split(',')
                .map(|l| l.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap(),
        )
    });

    points
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let points_2 = input
        .lines()
        .map(|l| {
            Point2::from_tuple(
                l.split(',')
                    .map(|l| l.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect_vec();

    let mut points = points_2.clone().into_iter();

    let first = points.next().unwrap();

    let mut prev = first;

    // let mut edges = HashSet::new();

    // for p in points {
    //     if p.x == prev.x {
    //         for y in p.y.min(prev.y)..=p.y.max(prev.y) {
    //             edges.insert(Point2::new(p.x, y));
    //         }
    //     } else if p.y == prev.y {
    //         for x in p.x.min(prev.x)..=p.x.max(prev.x) {
    //             edges.insert(Point2::new(x, p.y));
    //         }
    //     } else {
    //         panic!();
    //     }
    //     prev = p;
    // }

    // {
    //     let p = first;
    //     if p.x == prev.x {
    //         for y in p.y.min(prev.y)..=p.y.max(prev.y) {
    //             edges.insert(Point2::new(p.x, y));
    //         }
    //     } else if p.y == prev.y {
    //         for x in p.x.min(prev.x)..=p.x.max(prev.x) {
    //             edges.insert(Point2::new(x, p.y));
    //         }
    //     } else {
    //         panic!();
    //     }
    // }

    let pt = points_2
        .into_iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            a.y >= 50089 && b.y >= 50089
            // let min = a.y.min(b.y);
            // let max = a.y.max(b.y);
            // if min <= 48701 && max >= 50089 {
            //     return false;
            // }
            // return true;
        })
        .k_largest_by_key(10, |(a, b)| {
            (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
        });

    for x in pt {
        println!("{}", x.0);
        println!("{}", x.1);
        println!()
    }

    4
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
