use lib::{Dir, Grid, IteratorExt, a_star_score, itertools::Itertools, point2};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone)]
struct Node {
    x: u32,
    y: u32,
    used: u32,
    avail: u32,
}

fn parse_line(l: &str) -> Node {
    let (path, _, used, avail, _) = l.split_ascii_whitespace().collect_tuple().unwrap();
    let (_, x_str, y_str) = path.split('-').collect_tuple().unwrap();
    Node {
        x: x_str.strip_prefix('x').unwrap().parse().unwrap(),
        y: y_str.strip_prefix('y').unwrap().parse().unwrap(),
        used: used.strip_suffix('T').unwrap().parse().unwrap(),
        avail: avail.strip_suffix('T').unwrap().parse().unwrap(),
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .skip(2)
        .map(parse_line)
        .permutations(2)
        .map(|p| p.into_iter().collect_tuple().unwrap())
        .count_where(|(a, b)| a.used != 0 && !(a.x == b.x && a.y == b.y) && a.used <= b.avail)
        as u32
}

fn part2(input: &str) -> u32 {
    let nodes = input.lines().skip(2).map(parse_line).collect_vec();

    let max_x = nodes.iter().map(|n| n.x).max().unwrap() as usize;
    let max_y = nodes.iter().map(|n| n.y).max().unwrap() as usize;

    let mut grid = Grid::new_filled((0, 0), max_x + 1, max_y + 1);

    for node in nodes {
        grid.set((node.x as usize, node.y as usize), (node.used, node.avail));
    }

    let empty = grid.enumerate().find(|n| n.1.0 == 0).unwrap().0;
    let goal = point2(max_x, 0);

    a_star_score(
        vec![empty],
        |p| *p == goal,
        |p| {
            Dir::ORTHO
                .into_iter()
                .filter_map(|d| p.apply(d))
                .filter(|p| grid.get(*p).is_some_and(|c| c.0 < 100))
                .map(|p| (p, 1))
                .collect_vec()
        },
        |p| p.manhattan_dist(goal) as u32,
    )
    .unwrap()
        + ((max_x as u32 - 1) * 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 892);
        assert_eq!(part2(input), 227);
    }
}
