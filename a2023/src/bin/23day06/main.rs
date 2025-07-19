use lib::{itertools::Itertools, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct RaceInfo {
    time: f64,
    dist: f64,
}

fn parse_line(l: &str) -> Vec<f64> {
    l.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_race_info(s: &str) -> Vec<RaceInfo> {
    let (times_line, dists_line) = s.split_lines_once().unwrap();
    let times = parse_line(times_line);
    let dists = parse_line(dists_line);
    times
        .into_iter()
        .zip(dists)
        .map(|(time, dist)| RaceInfo { time, dist })
        .collect()
}

fn num_ways_to_beat_record(RaceInfo { time: r, dist: x }: RaceInfo) -> u32 {
    let root = (r * r - 4.0 * x).sqrt();
    (((r + root) / 2.0).ceil() - ((r - root) / 2.0).floor() - 1.0).round() as u32
}

fn part1(input: &str) -> u32 {
    parse_race_info(input)
        .into_iter()
        .map(num_ways_to_beat_record)
        .product::<u32>()
}

fn part2(input: &str) -> u32 {
    let (time, dist) = input
        .replace(' ', "")
        .lines()
        .map(|l| l.split_once(':').unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap();
    num_ways_to_beat_record(RaceInfo { time, dist })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1083852);
        assert_eq!(part2(input), 23501589);
    }
}
