use lib::{borrow_loop, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}


fn part1(input: &str) -> u32 {
    let mut chars = input.chars();
    let mut count = 0;

    borrow_loop!(chars, next, {
        count += if next == '(' {
            let (n, times) = chars
                .by_ref()
                .take_while(|c| *c != ')')
                .collect::<String>()
                .split('x')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            chars.nth(n - 1).unwrap();
            n * times
        } else {
            1
        };
    });

    count as u32
}

fn decompress_len(input: &str) -> usize {
    let mut chars = input.chars();
    let mut count = 0;

    borrow_loop!(chars, next, {
        count += if next == '(' {
            let (n, times) = chars
                .by_ref()
                .take_while(|c| *c != ')')
                .collect::<String>()
                .split('x')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let data = chars.by_ref().take(n).collect::<String>();
            decompress_len(&data) * times
        } else {
            1
        };
    });

    count
}

fn part2(input: &str) -> u64 {
    decompress_len(input) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 99145);
        assert_eq!(part2(input), 10943094568);
    }
}
