use lib::{Dir, Grid, Point2, a_star_score, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    p: Point2<usize>,
    visited: Vec<bool>,
}

fn neighbors(state: &State, grid: &Grid<u8>) -> Vec<(State, u32)> {
    Dir::ORTHO
        .into_iter()
        .map(move |dir| state.p.apply(dir).unwrap())
        .filter(|p| *grid.get(*p).unwrap() != b'#')
        .map(|p| {
            let mut cloned = state.visited.clone();
            let c = grid.get(p).unwrap();
            if c.is_ascii_digit() && *c != b'0' {
                cloned[(c - b'0' - 1) as usize] = true;
            }
            (State { p, visited: cloned }, 1)
        })
        .collect_vec()
}

fn setup(input: &str) -> (Grid<u8>, State, Point2<usize>) {
    let grid = Grid::from_bytes(input).unwrap();
    let start_pos = grid.find(&b'0').unwrap();
    let num_points = grid.count_where(u8::is_ascii_digit);

    let start = State {
        p: start_pos,
        visited: vec![false; num_points - 1],
    };

    (grid, start, start_pos)
}

fn part1(input: &str) -> u32 {
    let (grid, start, _) = setup(input);

    a_star_score(
        vec![start],
        |state| state.visited.iter().all(|x| *x),
        |state| neighbors(state, &grid),
        |_| 0,
    )
    .unwrap()
}

fn part2(input: &str) -> u32 {
    let (grid, start, start_pos) = setup(input);

    a_star_score(
        vec![start],
        |state| state.visited.iter().all(|x| *x) && state.p == start_pos,
        |state| neighbors(state, &grid),
        |_| 0,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 490);
        assert_eq!(part2(input), 744);
    }
}
