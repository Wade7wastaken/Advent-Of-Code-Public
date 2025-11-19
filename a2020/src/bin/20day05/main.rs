use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn seat_id(ident: &str) -> u32 {
    let mut row = 0;
    for &x in &ident.as_bytes()[0..7] {
        row *= 2;
        if x == b'B' {
            row += 1;
        }
    }

    let mut col = 0;
    for &x in &ident.as_bytes()[7..10] {
        col *= 2;
        if x == b'R' {
            col += 1;
        }
    }

    row * 8 + col
}

fn part1(input: &str) -> u32 {
    input.lines().map(seat_id).max().unwrap()
}

fn part2(input: &str) -> u32 {
    let map = input.lines().map(seat_id).collect::<HashSet<_>>();
    let max_id = map.iter().copied().max().unwrap();
    for id in 1..max_id {
        if !map.contains(&id) && map.contains(&(id + 1)) && map.contains(&(id - 1)) {
            return id;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 890);
        assert_eq!(part2(input), 651);
    }
}
