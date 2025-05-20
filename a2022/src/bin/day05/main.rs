use lib::itertools::Itertools;
use lib::{Grid, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim_end();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    Grid::from_chars_gaps(input, 1, 0, 4, 1)
        .unwrap()
        .into_cols_iter()
        .update(|col| col.retain(|x| *x != ' '))
        .update(|col| {
            col.pop();
        })
        .update(|col| col.reverse())
        .collect()
}

struct Move {
    src: usize,
    dest: usize,
    count: usize,
}

fn part1(input: &str) -> String {
    let (crates_str, moves_str) = input.split_paragraphs_once().unwrap();
    let mut crates = parse_crates(crates_str);

    let moves = moves_str.lines().map(|l| {
        let (count, src, dest) = l
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect_tuple()
            .unwrap();
        Move { src, dest, count }
    });

    for m in moves {
        for _ in 0..m.count {
            let removed = crates.get_mut(m.src - 1).unwrap().pop().unwrap();
            crates.get_mut(m.dest - 1).unwrap().push(removed);
        }
    }

    crates
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .join("")
}

fn part2(input: &str) -> String {
    let (crates_str, moves_str) = input.split_paragraphs_once().unwrap();
    let mut crates = parse_crates(crates_str);

    let moves = moves_str.lines().map(|l| {
        let (count, src, dest) = l
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect_tuple()
            .unwrap();
        Move { src, dest, count }
    });

    for m in moves {
        let stack = crates.get_mut(m.src - 1).unwrap();
        let len = stack.len();
        let mut taken = stack.split_off(len - m.count);
        crates.get_mut(m.dest - 1).unwrap().append(&mut taken);
    }

    crates
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim_end();
        assert_eq!(part1(input), "JCMHLVGMG");
        assert_eq!(part2(input), "LVMRWSSPZ");
    }
}
