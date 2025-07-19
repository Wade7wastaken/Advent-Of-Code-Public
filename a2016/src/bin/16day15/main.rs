use lib::itertools::Itertools;

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
    let words = l.split_ascii_whitespace().collect_vec();
    Disc {
        num_positions: words[3].parse().unwrap(),
        start_pos: words[11].strip_suffix('.').unwrap().parse().unwrap(),
    }
}

fn part1(input: &str) -> u32 {
    'outer: for t_drop in 0.. {
        for (i, disc) in input.lines().map(parse_disc).enumerate() {
            let n = i as u32 + 1;
            if (disc.start_pos + t_drop + n) % disc.num_positions != 0 {
                continue 'outer;
            }
        }
        return t_drop;
    }

    panic!();
}

fn part2(input: &str) -> u32 {
    'outer: for t_drop in 0.. {
        for (i, disc) in input
            .lines()
            .map(parse_disc)
            .chain(std::iter::once(Disc {
                num_positions: 11,
                start_pos: 0,
            }))
            .enumerate()
        {
            let n = i as u32 + 1;
            if (disc.start_pos + t_drop + n) % disc.num_positions != 0 {
                continue 'outer;
            }
        }
        return t_drop;
    }

    panic!();
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
