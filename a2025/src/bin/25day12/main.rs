use lib::{IteratorExt, StringTools, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut presents = input.paragraphs().collect_vec();
    let regions = presents.pop().unwrap();

    let present_sizes = presents
        .into_iter()
        .map(|p| {
            p.lines()
                .skip(1)
                .flat_map(|l| l.bytes().map(|b| b == b'#'))
                .count_where(|x| x)
        })
        .collect_vec();

    let mut res = 0;

    for region in regions.lines() {
        let (size, num_p) = region.split_once(": ").unwrap();
        let (width, height) = size
            .split('x')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let total = num_p
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .zip(&present_sizes)
            .map(|(num, size)| num * size)
            .sum();

        if width * height >= total {
            res += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 565);
    }
}
