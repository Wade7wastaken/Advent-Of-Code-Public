use lib::{Dir, Entity, Grid, StringTools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let (grid_str, moves_str) = input.split_paragraphs_once().unwrap();
    let mut grid = Grid::from_chars(grid_str).unwrap();

    let starting_pos = grid.find(&'@').unwrap();
    grid.set(starting_pos, '.');

    let mut robot = Entity::new_on_grid(starting_pos, Dir::North, &grid).unwrap();

    let moves = moves_str
        .lines()
        .flat_map(|l| l.chars().map(|a| Dir::try_from(a).unwrap()));

    for m in moves {
        robot = robot.set_dir(m);
        let mut test_en = robot.step_bounded().unwrap();

        while *grid.get(test_en.pos()).unwrap() == 'O' {
            test_en = test_en.step_bounded().unwrap();
        }

        if *grid.get(test_en.pos()).unwrap() == '#' {
            continue;
        }

        robot = robot.step_bounded().unwrap();
        grid.set(test_en.pos(), 'O').unwrap();
        grid.set(robot.pos(), '.');
    }

    grid.find_all(&'O').map(|p| 100 * p.y + p.x).sum::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    let (grid_str, moves_str) = input.split_once("\r\n\r\n").unwrap();
    let resized_str = grid_str
        .replace('#', "##")
        .replace('.', "..")
        .replace('O', "[]")
        .replace('@', "@.");
    let mut grid = Grid::from_chars(&resized_str).unwrap();

    let starting_pos = grid.find(&'@').unwrap();
    grid.set(starting_pos, '.');

    let mut robot = Entity::new_on_grid(starting_pos, Dir::North, &grid).unwrap();

    let moves = moves_str
        .lines()
        .flat_map(|l| l.chars().map(|a| Dir::try_from(a).unwrap()));

    for m in moves {
        if matches!(m, Dir::East | Dir::West) {
            robot = robot.set_dir(m);
            let mut test_en = robot.step_bounded().unwrap();
            let end = *grid.get(test_en.step_bounded().unwrap().pos()).unwrap();
            let mut to_switch = vec![];
            while *grid.get(test_en.pos()).unwrap() == '['
                || *grid.get(test_en.pos()).unwrap() == ']'
            {
                to_switch.push(test_en.pos());
                test_en = test_en.step_bounded().unwrap();
            }
            if *grid.get(test_en.pos()).unwrap() == '#' {
                continue;
            }
            robot = robot.step_bounded().unwrap();
            for p in to_switch {
                let c = tern!(*grid.get(p).unwrap() == '[', ']', '[');
                grid.set(p, c).unwrap();
            }
            grid.set(test_en.pos(), end).unwrap();
            grid.set(robot.pos(), '.');
        } else {
            robot = robot.set_dir(m);
            if can_apply_force(&grid, robot.step_bounded().unwrap()) {
                apply_force(&mut grid, robot.step_bounded().unwrap());
                robot = robot.step_bounded().unwrap();
            }
        }
    }

    grid.into_enumerate()
        .filter(|(_, c)| *c == '[')
        .map(|(p, _)| 100 * p.y + p.x)
        .sum::<usize>() as u32
}

fn can_apply_force_single(grid: &Grid<char>, en: Entity<usize>) -> bool {
    let next = en.step_bounded().unwrap();
    let c = *grid.get(next.pos()).unwrap();
    match c {
        '#' => false,
        '[' | ']' => can_apply_force(grid, next),
        '.' => true,
        _ => unreachable!(),
    }
}

fn can_apply_force(grid: &Grid<char>, en: Entity<usize>) -> bool {
    let c = *grid.get(en.pos()).unwrap();
    let other = match c {
        '[' => en.slide_bounded(Dir::East).unwrap(),
        ']' => en.slide_bounded(Dir::West).unwrap(),
        '#' => {
            return false;
        }
        '.' => {
            return true;
        }
        _ => unreachable!(),
    };
    can_apply_force_single(grid, en) && can_apply_force_single(grid, other)
}

fn apply_force_single(grid: &mut Grid<char>, en: Entity<usize>) {
    let next = en.step_bounded().unwrap();
    let c = *grid.get(next.pos()).unwrap();
    match c {
        '[' | ']' => apply_force(grid, next),
        '.' => (),
        _ => unreachable!(),
    }
    let cur = *grid.get(en.pos()).unwrap();
    grid.set(next.pos(), cur);
    grid.set(en.pos(), '.');
}

fn apply_force(grid: &mut Grid<char>, en: Entity<usize>) {
    let c = *grid.get(en.pos()).unwrap();
    let other = match c {
        '[' => en.slide_bounded(Dir::East).unwrap(),
        ']' => en.slide_bounded(Dir::West).unwrap(),
        '#' | '.' => return,
        _ => unreachable!(),
    };
    apply_force_single(grid, en);
    apply_force_single(grid, other);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1509074);
        assert_eq!(part2(input), 1521453);
    }
}
