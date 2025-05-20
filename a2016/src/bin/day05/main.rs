use lib::{CollectString, md5};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> String {
    (0..)
        .map(|i| format!("{:x}", md5::compute(format!("{input}{i}"))))
        .filter(|hex| hex.starts_with("00000"))
        .take(8)
        .map(|hex| hex.as_bytes()[5])
        .collect_string()
}

fn part2(input: &str) -> String {
    let mut password = [0u8; 8];
    let mut filled = [false; 8];

    let hexs = (0..)
        .map(|i| format!("{:x}", md5::compute(format!("{input}{i}"))))
        .filter(|hex| hex.starts_with("00000"));

    for hex in hexs {
        let bytes = hex.as_bytes();
        let position = bytes[5];
        let position_i = (position - b'0') as usize;
        if !(b'0'..=b'7').contains(&position) || filled[position_i] {
            continue;
        }
        password[position_i] = bytes[6];
        filled[position_i] = true;
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
