use lib::{Dir, Entity, Grid, Offset};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn turn(grid: &Grid<char>, en: Entity<usize>) -> Option<Entity<usize>> {
    grid.with_offsets(en, Dir::ORTHO)
        .all()
        .filter(|(_, d, c)| **c != ' ' && *d != en.dir().reverse())
        .map(|(_, d, _)| en.set_dir(d))
        .next()
}

fn setup(input: &str) -> (Grid<char>, Entity<usize>) {
    let grid = Grid::from_chars(input).unwrap();
    let start_x = grid.row(0).unwrap().iter().position(|c| *c == '|').unwrap();
    let en = Entity::new_on_grid((start_x, 0), Dir::South, &grid).unwrap();
    (grid, en)
}

fn part1(input: &str) -> String {
    let mut seen = String::new();
    let (grid, mut en) = setup(input);

    loop {
        while let Some(next) = en.step_bounded()
            && *grid.get(next).unwrap() != ' '
        {
            if grid.get(next).unwrap().is_ascii_uppercase() {
                seen.push(*grid.get(next).unwrap());
            }
            en = next;
        }

        let turned = turn(&grid, en);

        if let Some(n) = turned {
            en = n;
        } else {
            return seen;
        }
    }
}

fn part2(input: &str) -> u32 {
    let mut steps = 1;
    let (grid, mut en) = setup(input);

    loop {
        while let Some(next) = en.step_bounded()
            && *grid.get(next).unwrap() != ' '
        {
            en = next;
            steps += 1;
        }

        let turned = turn(&grid, en);

        if let Some(n) = turned {
            en = n;
        } else {
            return steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt");
        assert_eq!(part1(input), "DWNBGECOMY");
        assert_eq!(part2(input), 17228);
    }
}
