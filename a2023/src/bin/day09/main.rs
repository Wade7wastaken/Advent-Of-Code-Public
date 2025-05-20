use lib::Point2;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_nums(l: &str) -> Vec<Point2<f64>> {
    l.split_whitespace()
        .map(|n| n.parse().unwrap())
        .enumerate()
        .map(|(a, b)| (a as f64, b).into())
        .collect()
}

fn lagrange_interpolation(points: &[Point2<f64>], x: f64) -> f64 {
    points
        .iter()
        .enumerate()
        .map(|(j, p_j)| {
            p_j.y
                * (points
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != j)
                    .map(|(_, p_i)| (x - p_i.x) / (p_j.x - p_i.x))
                    .product::<f64>())
        })
        .sum()
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let nums = parse_nums(l);
            lagrange_interpolation(&nums, nums.len() as f64).round()
        })
        .sum::<f64>() as u32
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| lagrange_interpolation(&parse_nums(l), -1.0).round())
        .sum::<f64>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1877825184);
        assert_eq!(part2(input), 1108);
    }
}
