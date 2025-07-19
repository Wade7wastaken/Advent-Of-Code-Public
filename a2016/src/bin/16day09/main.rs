use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut chars = input.chars();
    let mut count = 0;
    while let Some(next) = chars.next() {
        if next.is_ascii_whitespace() {
            continue;
        }
        if next == '(' {
            let s = chars.take_while_ref(|c| *c != ')').collect::<String>();
            chars.next().unwrap(); // consume )
            let (n, times) = s
                .split('x')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            chars.nth(n - 1).unwrap();
            count += n * times;
        } else {
            count += 1;
        }
    }

    count as u32
}

fn decompress_len(input: &str) -> usize {
    let mut chars = input.chars();
    let mut count = 0;

    while let Some(next) = chars.next() {
        if next.is_ascii_whitespace() {
            continue;
        }
        if next == '(' {
            let s = chars.take_while_ref(|c| *c != ')').collect::<String>();
            chars.next().unwrap(); // consume )
            let (n, times) = s
                .split('x')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let data = (&mut chars).take(n).collect::<String>();
            let decompressed_len = decompress_len(&data) * times;
            count += decompressed_len;
        } else {
            count += 1;
        }
    }

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
