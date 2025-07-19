use lib::{
    AStarMulti, AStarSingle, CollectString, Dir, Point2, itertools::Itertools, md5::Context,
    pathfinding, point2,
};

fn main() {
    let input = include_str!("./input.txt").trim();
    // println!("{}", part1(input));
    // println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    p: Point2<usize>,
    path: Vec<Dir>,
}

fn new_state(p: Point2<usize>, path: Vec<Dir>) -> State {
    State { p, path }
}

impl From<State> for Point2<usize> {
    fn from(val: State) -> Self {
        val.p
    }
}

fn dir_char(d: Dir) -> u8 {
    match d {
        Dir::East => b'R',
        Dir::North => b'U',
        Dir::South => b'D',
        Dir::West => b'L',
    }
}

fn clone_and_append(path: &Vec<Dir>, new: Dir) -> Vec<Dir> {
    let mut path: Vec<Dir> = path.to_owned();
    path.push(new);
    path
}

fn part1(input: &str) -> String {
    pathfinding::directed::astar::astar(
        &State {
            p: point2(0, 0),
            path: vec![],
        },
        |state| {
            let mut ctx = Context::new();
            ctx.consume(input);
            ctx.consume(state.path.iter().copied().map(dir_char).collect_string());
            let hash = ctx.compute();
            Dir::ORTHO
                .into_iter()
                .filter(|dir| match dir {
                    Dir::North => ((hash[0] & 0xf0) >> 4) >= 0xb,
                    Dir::South => (hash[0] & 0x0f) >= 0xb,
                    Dir::West => ((hash[1] & 0xf0) >> 4) >= 0xb,
                    Dir::East => (hash[1] & 0x0f) >= 0xb,
                })
                .filter_map(|dir| {
                    state
                        .p
                        .apply(dir)
                        .map(|p| new_state(p, clone_and_append(&state.path, dir)))
                })
                .filter(|state| state.p.x <= 3 && state.p.y <= 3)
                .map(|state| (state, 1))
                .collect_vec()
        },
        |_| 0,
        |state| state.p == point2(3, 3),
    )
    .unwrap()
    .0
    .last()
    // AStarSingle::new(
    // vec![State {
    //     p: point2(0, 0),
    //     path: vec![],
    // }],
    //     |state| state.p == point2(3, 3),
    // |state| {
    //     let mut ctx = Context::new();
    //     ctx.consume(input);
    //     ctx.consume(state.path.iter().copied().map(dir_char).collect_string());
    //     let hash = ctx.compute();
    //     Dir::ORTHO
    //         .into_iter()
    //         .filter(|dir| match dir {
    //             Dir::North => ((hash[0] & 0xf0) >> 4) >= 0xb,
    //             Dir::South => (hash[0] & 0x0f) >= 0xb,
    //             Dir::West => ((hash[1] & 0xf0) >> 4) >= 0xb,
    //             Dir::East => (hash[1] & 0x0f) >= 0xb,
    //         })
    //         .filter_map(|dir| {
    //             state
    //                 .p
    //                 .apply(dir)
    //                 .map(|p| new_state(p, clone_and_append(&state.path, dir)))
    //         })
    //         .filter(|state| state.p.x <= 3 && state.p.y <= 3)
    //         .map(|state| (state, 1))
    //         .collect_vec()
    //     },
    //     |_| 0,
    // )
    // .next()
    // .unwrap()
    // .path()
    // .0
    // .last()
    .unwrap()
    .path
    .iter()
    .copied()
    .map(dir_char)
    .collect_string()
}

// fn part2(input: &str) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        // assert_eq!(part1(input), todo!());
        // assert_eq!(part2(input), todo!());
    }
}
