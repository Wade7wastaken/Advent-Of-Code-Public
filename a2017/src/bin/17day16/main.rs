use lib::{DetectCycle, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

const NUM_PROGRAMS: usize = 16;
const TOTAL_DANCES: usize = 1000000000;

#[derive(Debug, Clone)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn parse_dance_move(input: &str) -> DanceMove {
    if let Some(rest) = input.strip_prefix('s') {
        let x: usize = rest.parse().unwrap();
        DanceMove::Spin(x)
    } else if let Some(rest) = input.strip_prefix('x') {
        let (a, b) = rest
            .split('/')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        DanceMove::Exchange(a, b)
    } else if let Some(rest) = input.strip_prefix('p') {
        let (a, _, b) = rest.bytes().collect_tuple().unwrap();
        DanceMove::Partner(a, b)
    } else {
        panic!();
    }
}

fn dance(mut positions: Vec<u8>, moves: &Vec<DanceMove>) -> Vec<u8> {
    for m in moves {
        match m {
            DanceMove::Spin(x) => {
                for removed in positions.drain((NUM_PROGRAMS - x)..).rev().collect_vec() {
                    positions.insert(0, removed);
                }
            }
            DanceMove::Exchange(a, b) => {
                positions.swap(*a, *b);
            }
            DanceMove::Partner(a, b) => {
                let a_pos = positions.iter().position(|x| *x == a - b'a').unwrap();
                let b_pos = positions.iter().position(|x| *x == b - b'a').unwrap();
                positions.swap(a_pos, b_pos);
            }
        }
    }
    positions
}

fn part1(input: &str) -> String {
    dance(
        (0u8..(NUM_PROGRAMS as u8)).collect_vec(),
        &input
            .split(',')
            .map(|m| parse_dance_move(m.trim()))
            .collect_vec(),
    )
    .into_iter()
    .map(|p| (p + b'a') as char)
    .collect()
}

fn part2(input: &str) -> String {
    let moves = input
        .split(',')
        .map(|m| parse_dance_move(m.trim()))
        .collect_vec();

    std::iter::successors(Some((0u8..=15).collect_vec()), |positions| {
        Some(dance(positions.clone(), &moves))
    })
    .nth_cyclic(TOTAL_DANCES)
    .unwrap()
    .into_iter()
    .map(|p| (p + b'a') as char)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "jkmflcgpdbonihea");
        assert_eq!(part2(input), "ajcdefghpkblmion");
    }
}
