use lib::{
    CollectString, DigestHex,
    md5::{Context, Digest},
};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn five_zeros(input: &str, ctx: &mut Context) -> impl Iterator<Item = Digest> {
    ctx.consume(input);

    (0..)
        .map(|i| {
            let mut c = ctx.clone();
            c.consume(i.to_string());
            c.compute()
        })
        .filter(|hash| hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0)
}

fn part1(input: &str) -> String {
    five_zeros(input, &mut Context::new())
        .take(8)
        .map(|hash| hash.hex_digit(5))
        .collect_string()
}

fn part2(input: &str) -> String {
    let mut password = [0u8; 8];
    let mut filled = [false; 8];

    let mut ctx = Context::new();

    let iter = five_zeros(input, &mut ctx).map(|hash| (hash[2] as usize, hash.hex_digit(6)));

    for (position, char) in iter {
        if !(0..=7).contains(&position) || filled[position] {
            continue;
        }
        password[position] = char;
        filled[position] = true;
        if filled.iter().all(|x| *x) {
            break;
        }
    }

    String::from_utf8_lossy(&password).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "4543c154");
        assert_eq!(part2(input), "1050cbbd");
    }
}
