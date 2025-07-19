use lib::{cycle, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug)]
struct Run {
    count: u8,
    value: u8,
}

fn seq(runs: Vec<Run>) -> Vec<Run> {
    let new_cap = (runs.len() as f32 * 1.32).ceil() as usize;
    let mut output = Vec::with_capacity(new_cap);
    let mut cur = Run { count: 0, value: 0 };
    for run in runs {
        // add count
        if run.count == cur.value {
            cur.count += 1;
        } else {
            if cur.count != 0 {
                output.push(cur);
            }
            cur = Run {
                count: 1,
                value: run.count,
            }
        }
        // add value
        if run.value == cur.value {
            cur.count += 1;
        } else {
            if cur.count != 0 {
                output.push(cur);
            }
            cur = Run {
                count: 1,
                value: run.value,
            }
        }
    }
    output.push(cur);

    output
}

fn rle(s: &str) -> Vec<Run> {
    s.bytes()
        .chunk_by(|x| *x)
        .into_iter()
        .map(|(c, group)| Run {
            count: group.count() as u8,
            value: c - b'0',
        })
        .collect()
}

fn seq_sum(input: Vec<Run>, n: usize) -> u32 {
    cycle(input, n, seq)
        .into_iter()
        .map(|run| u32::from(run.count))
        .sum()
}

fn part1(input: &str) -> u32 {
    seq_sum(rle(input), 40)
}

fn part2(input: &str) -> u32 {
    seq_sum(rle(input), 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 252594);
        assert_eq!(part2(input), 3579328);
    }
}
