fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn score_group(group: &mut impl Iterator<Item = char>, parent_score: u32) -> u32 {
    let mut score = parent_score + 1;

    loop {
        match group.next().unwrap() {
            '<' => loop {
                match group.next().unwrap() {
                    '!' => drop(group.next().unwrap()),
                    '>' => break,
                    _ => {}
                }
            },
            '{' => score += score_group(group, parent_score + 1),
            '}' => break,
            _ => {}
        }
    }

    score
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| score_group(&mut l[1..].chars(), 0))
        .sum()
}

fn count_garbage(group: &mut impl Iterator<Item = char>) -> u32 {
    let mut garbage = 0;

    loop {
        match group.next().unwrap() {
            '<' => loop {
                match group.next().unwrap() {
                    '!' => drop(group.next().unwrap()),
                    '>' => break,
                    _ => garbage += 1,
                }
            },
            '{' => garbage += count_garbage(group),
            '}' => break,
            _ => {}
        }
    }

    garbage
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| count_garbage(&mut l[1..].chars()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 10820);
        assert_eq!(part2(input), 5547);
    }
}
