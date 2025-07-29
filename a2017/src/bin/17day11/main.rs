use lib::{Point2, point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut p: Point2<i32> = point2(0, 0);

    for step in input.split(',').map(str::trim) {
        let dir = match step {
            "n" => Point2::new(0, -1),
            "ne" => Point2::new(1, -1),
            "se" => Point2::new(1, 0),
            "s" => Point2::new(0, 1),
            "sw" => Point2::new(-1, 1),
            "nw" => Point2::new(-1, 0),
            _ => panic!(),
        };
        p += dir;
    }

    p.x.unsigned_abs()
        .max(p.y.unsigned_abs())
        .max((p.x + p.y).unsigned_abs())
}

fn part2(input: &str) -> u32 {
    let mut p: Point2<i32> = point2(0, 0);

    let mut max = 0;

    for step in input.split(',').map(str::trim) {
        let dir = match step {
            "n" => Point2::new(0, -1),
            "ne" => Point2::new(1, -1),
            "se" => Point2::new(1, 0),
            "s" => Point2::new(0, 1),
            "sw" => Point2::new(-1, 1),
            "nw" => Point2::new(-1, 0),
            _ => panic!(),
        };
        p += dir;
        max = max.max(
            p.x.unsigned_abs()
                .max(p.y.unsigned_abs())
                .max((p.x + p.y).unsigned_abs()),
        );
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 670);
        assert_eq!(part2(input), 1426);
    }
}
