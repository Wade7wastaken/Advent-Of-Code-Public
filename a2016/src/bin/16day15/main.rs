use lib::{itertools::Itertools, select};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Disc {
    num_positions: u32,
    start_pos: u32,
}

fn parse_disc(l: &str) -> Disc {
    let (num_positions_str, start_pos_str) = select!(l.split_ascii_whitespace(); 3, 11);
    Disc {
        num_positions: num_positions_str.parse().unwrap(),
        start_pos: start_pos_str.strip_suffix('.').unwrap().parse().unwrap(),
    }
}

fn find_time(discs: &[Disc]) -> u32 {
    (0..u32::MAX)
        .find(|t_drop| {
            discs
                .iter()
                .enumerate()
                .all(|(i, disc)| (disc.start_pos + t_drop + i as u32 + 1) % disc.num_positions == 0)
        })
        .unwrap()
}

fn part1(input: &str) -> u32 {
    let discs = input.lines().map(parse_disc).collect_vec();
    find_time(&discs)
}

fn part2(input: &str) -> u32 {
    let mut discs = input.lines().map(parse_disc).collect_vec();
    discs.push(Disc {
        num_positions: 11,
        start_pos: 0,
    });

    find_time(&discs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 400589);
        assert_eq!(part2(input), 3045959);
    }
}
