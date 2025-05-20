use lib::{Dir, Entity, Grid};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).unwrap();
    let mut en = Entity::new_on_grid((1, 1), Dir::NORTH, &grid).unwrap();
    let mut code = 0;
    for line in input.lines() {
        for c in line.chars() {
            let dir = Dir::try_from(c).unwrap();
            if let Some(next) = en.slide_bounded(dir) {
                en = next;
            }
        }
        code *= 10;
        code += grid.get(en).unwrap();
    }
    code
}

fn part2(input: &str) -> String {
    let grid = Grid::new(vec![
        vec![None, None, Some('1'), None, None],
        vec![None, Some('2'), Some('3'), Some('4'), None],
        vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        vec![None, Some('A'), Some('B'), Some('C'), None],
        vec![None, None, Some('D'), None, None],
    ])
    .unwrap();
    let mut en = Entity::new_on_grid((0, 2), Dir::NORTH, &grid).unwrap();
    let mut code = String::new();
    for line in input.lines() {
        for c in line.chars() {
            let dir = Dir::try_from(c).unwrap();
            if let Some(next) = en.slide_bounded(dir) {
                if grid.get(next).unwrap().is_some() {
                    en = next;
                }
            }
        }
        code.push(grid.get(en).unwrap().unwrap());
    }
    code
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
