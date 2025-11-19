fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Operation {
    target: u64,
    nums: Vec<u64>,
}

fn parse_operation(line: &str) -> Operation {
    let (target_str, nums_str) = line.split_once(": ").unwrap();
    let target = target_str.parse().unwrap();
    let nums = nums_str
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    Operation { target, nums }
}

const fn concat(rhs: u64, lhs: u64) -> u64 {
    rhs * 10u64.pow(lhs.ilog10() + 1) + lhs
}

fn is_solvable(op: &Operation, i: usize, v: u64) -> bool {
    if v > op.target {
        return false;
    }
    if i == op.nums.len() {
        return v == op.target;
    }
    let num = op.nums[i];
    is_solvable(op, i + 1, v + num) || is_solvable(op, i + 1, v * num)
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(parse_operation)
        .filter(|op| is_solvable(op, 1, op.nums[0]))
        .map(|op| op.target)
        .sum()
}

fn is_solvable_concat(op: &Operation, i: usize, v: u64) -> bool {
    if v > op.target {
        return false;
    }
    if i == op.nums.len() {
        return v == op.target;
    }
    let num = op.nums[i];
    is_solvable_concat(op, i + 1, v + num)
        || is_solvable_concat(op, i + 1, v * num)
        || is_solvable_concat(op, i + 1, concat(v, num))
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(parse_operation)
        .filter(|op| is_solvable_concat(op, 1, op.nums[0]))
        .map(|op| op.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 2299996598890);
        assert_eq!(part2(input), 362646859298554);
    }
}
