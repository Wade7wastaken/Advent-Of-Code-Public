use lib::{itertools::Itertools, select};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_list(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn sum_of_metadata(mut list: &mut impl Iterator<Item = u32>) -> u32 {
    let (num_children, num_metadata) = select!(list; 0, 1);
    (0..num_children)
        .map(|_| sum_of_metadata(list))
        .sum::<u32>()
        + (0..num_metadata).map(|_| list.next().unwrap()).sum::<u32>()
}

fn part1(input: &str) -> u32 {
    sum_of_metadata(&mut parse_list(input).into_iter())
}

fn node_value(mut list: &mut impl Iterator<Item = u32>) -> u32 {
    let (num_children, num_metadata) = select!(list; 0, 1);
    if num_children == 0 {
        (0..num_metadata).map(|_| list.next().unwrap()).sum::<u32>()
    } else {
        let children = (0..num_children).map(|_| node_value(list)).collect_vec();
        (0..num_metadata)
            .map(|_| {
                children
                    .get(list.next().unwrap() as usize - 1)
                    .copied()
                    .unwrap_or(0)
            })
            .sum()
    }
}

fn part2(input: &str) -> u32 {
    node_value(&mut parse_list(input).into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 45868);
        assert_eq!(part2(input), 19724);
    }
}
