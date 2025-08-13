use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Room<'a> {
    name: &'a str,
    checksum: &'a str,
    id: u32,
}

fn parse_room(l: &str) -> Room {
    let (name_id, checksum) = l.split_once('[').unwrap();
    let (name, room_id) = name_id.rsplit_once('-').unwrap();
    Room {
        name,
        checksum: checksum.strip_suffix(']').unwrap(),
        id: room_id.parse().unwrap(),
    }
}

fn is_correct_checksum(room: &Room) -> bool {
    room.name
        .bytes()
        .counts()
        .into_iter()
        .filter(|(k, _)| *k != b'-')
        .k_largest_by(5, |(char_a, count_a), (char_b, count_b)| {
            count_a.cmp(count_b).then(char_b.cmp(char_a))
        })
        .map(|x| x.0)
        .eq(room.checksum.bytes())
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_room)
        .filter(is_correct_checksum)
        .map(|room| room.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_room)
        // .filter(is_correct_checksum)
        .find_map(|room| {
            let shift = (room.id % 26) as u8;
            room.name
                .bytes()
                .map(|b| match b {
                    b'-' => b' ',
                    _ => ((b - b'a' + shift) % 26) + b'a',
                })
                .eq("northpole object storage".bytes())
                .then_some(room.id)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 245102);
        assert_eq!(part2(input), 324);
    }
}
