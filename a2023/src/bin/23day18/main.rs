use lib::{Dir, Point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn calc_area(instructions: Vec<Instr>) -> i64 {
    let (area, perimeter, _) = instructions.into_iter().fold(
        (0, 0, Point2::new(0, 0)),
        |(area, perimeter, pos), Instr { dir, len }| {
            let new_pos = pos.apply_n(dir, len).unwrap();
            let new_area = area + (pos.x * new_pos.y - new_pos.x * pos.y);
            let new_perimeter = (new_pos.x - pos.x).abs() + (new_pos.y - pos.y).abs() + perimeter;
            (new_area, new_perimeter, new_pos)
        },
    );

    (area.abs() + perimeter) / 2 + 1
}

struct Instr {
    dir: Dir,
    len: i64,
}

fn parse_instr(instr: &str) -> Instr {
    let [dir_str, len_str, _] = instr.split_whitespace().collect::<Vec<_>>()[..] else {
        panic!()
    };

    let dir = match dir_str {
        "R" => Dir::East,
        "L" => Dir::West,
        "U" => Dir::North,
        "D" => Dir::South,
        _ => unreachable!(),
    };
    Instr {
        dir,
        len: len_str.parse().unwrap(),
    }
}

fn parse_instr_hex(instr: &str) -> Instr {
    let hex = instr.split_whitespace().nth(2).unwrap();

    let len = i64::from_str_radix(&hex[2..7], 16).unwrap();

    let dir = match &hex[7..8] {
        "0" => Dir::East,
        "1" => Dir::South,
        "2" => Dir::West,
        "3" => Dir::North,
        _ => unreachable!(),
    };
    Instr { dir, len }
}

fn part1(input: &str) -> i64 {
    calc_area(input.lines().map(parse_instr).collect())
}

fn part2(input: &str) -> i64 {
    calc_area(input.lines().map(parse_instr_hex).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 49061);
        assert_eq!(part2(input), 92556825427032);
    }
}
