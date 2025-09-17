use lib::{a_star_single, md5::{Digest, Md5}, point2, Dir, IteratorExt, Point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    p: Point2<usize>,
    path: Vec<Dir>,
}

impl From<State> for Point2<usize> {
    fn from(val: State) -> Self {
        val.p
    }
}

const fn dir_char(d: Dir) -> u8 {
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

fn neighbors(input: &str, state: State) -> impl Iterator<Item = State> {
    let mut ctx = Md5::new();
    ctx.update(input);
    ctx.update(state.path.iter().copied().map(dir_char).collect_string());
    let hash = ctx.finalize();
    Dir::ORTHO
        .into_iter()
        .filter(move |dir| match dir {
            Dir::North => ((hash[0] & 0xf0) >> 4) >= 0xb,
            Dir::South => (hash[0] & 0x0f) >= 0xb,
            Dir::West => ((hash[1] & 0xf0) >> 4) >= 0xb,
            Dir::East => (hash[1] & 0x0f) >= 0xb,
        })
        .filter_map(move |dir| {
            state.p.apply(dir).map(|p| State {
                p,
                path: clone_and_append(&state.path, dir),
            })
        })
        .filter(|state| state.p.x <= 3 && state.p.y <= 3)
}

const fn start_state() -> State {
    State {
        p: point2(0, 0),
        path: vec![],
    }
}

fn part1(input: &str) -> String {
    println!("{}", size_of::<usize>());
    a_star_single(
        vec![start_state()],
        |state| state.p == point2(3, 3),
        |state| neighbors(input, state.clone()).map(|state| (state, 1)),
        |_| 0,
    )
    .unwrap()
    .end()
    .path
    .iter()
    .copied()
    .map(dir_char)
    .collect_string()
}

fn part2(input: &str) -> u32 {
    let mut stack = vec![start_state()];

    let mut longest = 0;

    while let Some(state) = stack.pop() {
        if state.p == point2(3, 3) {
            longest = longest.max(state.path.len());
            continue;
        }

        for n in neighbors(input, state) {
            stack.push(n);
        }
    }

    longest as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "DURLDRRDRD");
        assert_eq!(part2(input), 650);
    }
}
