use lib::{CountWhere, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn next_row(row: &[bool]) -> Vec<bool> {
    row.iter()
        .enumerate()
        .map(|(i, &center)| {
            let left = i
                .checked_sub(1)
                .and_then(|i| row.get(i))
                .copied()
                .unwrap_or(false);
            let right = row.get(i + 1).copied().unwrap_or(false);
            matches!(
                (left, center, right),
                (true, true | false, false) | (false, true | false, true)
            )
        })
        .collect_vec()
}

fn solve(input: &str, num_rows: usize) -> u32 {
    std::iter::successors(Some(input.bytes().map(|b| b == b'^').collect_vec()), |a| {
        Some(next_row(a))
    })
    .take(num_rows)
    .map(|row| row.into_iter().count_where(|x| !x) as u32)
    .sum()
}

fn part1(input: &str) -> u32 {
    solve(input, 40)
}

fn part2(input: &str) -> u32 {
    solve(input, 400000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1913);
        assert_eq!(part2(input), 19993564);
    }
}
