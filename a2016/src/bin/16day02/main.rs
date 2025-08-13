use lib::{CollectDigits, CollectString, Dir, Entity, Grid};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).unwrap();
    let en = Entity::new_on_grid((1, 1), Dir::North, &grid).unwrap();
    input
        .lines()
        .scan(en, |en, line| {
            *en = line.bytes().fold(*en, |en, c| {
                en.slide_bounded(Dir::try_from(c).unwrap()).unwrap_or(en)
            });
            Some(*grid.get(*en).unwrap())
        })
        .collect_digits()
}

fn part2(input: &str) -> String {
    let grid = Grid::new(vec![
        vec![None, None, Some(b'1'), None, None],
        vec![None, Some(b'2'), Some(b'3'), Some(b'4'), None],
        vec![Some(b'5'), Some(b'6'), Some(b'7'), Some(b'8'), Some(b'9')],
        vec![None, Some(b'A'), Some(b'B'), Some(b'C'), None],
        vec![None, None, Some(b'D'), None, None],
    ])
    .unwrap();
    let en = Entity::new_on_grid((0, 2), Dir::North, &grid).unwrap();

    input
        .lines()
        .scan(en, |en, line| {
            *en = line.bytes().fold(*en, |en, c| {
                en.slide_bounded(Dir::try_from(c).unwrap())
                    .filter(|en| grid.get(*en).unwrap().is_some())
                    .unwrap_or(en)
            });
            Some(grid.get(*en).unwrap().unwrap())
        })
        .collect_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 12578);
        assert_eq!(part2(input), "516DD");
    }
}
