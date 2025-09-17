use lib::{Digest, DigitIter, Md5, digest::Output, rayon::prelude::*};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn search(input: &str, is_valid: impl Fn(&Output<Md5>) -> bool + Sync) -> u32 {
    (1..u32::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .map_init(
            || {
                let mut ctx = Md5::new();
                ctx.update(input);
                ctx
            },
            |ctx, i| {
                let mut c = ctx.clone();
                for d in DigitIter::new(i) {
                    c.update([d + b'0']);
                }
                (i, c.finalize())
            },
        )
        .find_first(|(_, hash)| is_valid(hash))
        .unwrap()
        .0
}

fn part1(input: &str) -> u32 {
    search(input, |hash| {
        hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0
    })
}

fn part2(input: &str) -> u32 {
    search(input, |hash| hash[0] == 0 && hash[1] == 0 && hash[2] == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 346386);
        assert_eq!(part2(input), 9958218);
    }
}
