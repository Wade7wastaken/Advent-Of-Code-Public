use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use num::Zero;

use crate::{Grid, Point2};

use super::{node::Node, path::Path};

pub struct AStarMulti<C, Cost, EndCond, Neighbors, H> {
    // constants
    end_cond: EndCond,
    neighbors: Neighbors,
    h: H,

    open_set: BinaryHeap<Node<C, Cost>>, // the set of cells we need to look at, ordered by node.cost
    g_score: HashMap<C, Cost>,           // score for traveling to a specific node
    came_from: HashMap<C, Vec<C>>,
}

impl<
    C: Clone + PartialEq + Eq + Hash,
    Cost: Clone + Ord + Zero,
    EndCond: FnMut(&C) -> bool,
    I: IntoIterator<Item = (C, Cost)>,
    Neighbors: FnMut(&C) -> I,
    H: FnMut(&C) -> Cost,
> AStarMulti<C, Cost, EndCond, Neighbors, H>
{
    pub fn new(starts: Vec<C>, end_cond: EndCond, neighbors: Neighbors, mut h: H) -> Self {
        let mut open_set = BinaryHeap::with_capacity(starts.len());
        let mut g_score = HashMap::with_capacity(starts.len());

        for start in starts {
            open_set.push(Node {
                data: start.clone(),
                cost: h(&start),
            });
            g_score.insert(start, Cost::zero());
        }

        Self {
            end_cond,
            neighbors,
            h,
            open_set,
            g_score,
            came_from: HashMap::new(),
        }
    }

    pub const fn reconstruct(&self, end: C) -> MultiPathResult<'_, C, Cost> {
        MultiPathResult {
            ends: end,
            scores: &self.g_score,
            came_from: &self.came_from,
        }
    }
}

impl<
    C: Clone + PartialEq + Eq + Hash,
    Cost: Clone + Ord + Zero,
    EndCond: FnMut(&C) -> bool,
    I: IntoIterator<Item = (C, Cost)>,
    Neighbors: FnMut(&C) -> I,
    H: FnMut(&C) -> Cost,
> Iterator for AStarMulti<C, Cost, EndCond, Neighbors, H>
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Node { data: node, .. }) = self.open_set.pop() {
            if (self.end_cond)(&node) {
                return Some(node);
            }

            for (neighbor, move_cost) in (self.neighbors)(&node) {
                // g_score[node] will never be none because everything in open_set will be in in g_score
                let tent_g_score = self.g_score.get(&node).unwrap().clone() + move_cost;
                let actual_g_score = self.g_score.get(&neighbor);
                let cmp = actual_g_score.map(|actual| tent_g_score.cmp(actual));

                match cmp {
                    None | Some(Ordering::Less) => {
                        self.g_score.insert(neighbor.clone(), tent_g_score.clone());
                        self.came_from.insert(neighbor.clone(), vec![node.clone()]);
                        let new_cost = tent_g_score + (self.h)(&neighbor);
                        self.open_set.push(Node {
                            data: neighbor,
                            cost: new_cost,
                        });
                    }
                    Some(Ordering::Equal) => {
                        self.came_from
                            .get_mut(&neighbor)
                            .unwrap()
                            .push(node.clone());
                    }
                    _ => {}
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct MultiPathResult<'a, C, Cost> {
    ends: C,
    scores: &'a HashMap<C, Cost>,
    came_from: &'a HashMap<C, Vec<C>>,
}

impl<C: Clone + PartialEq + Eq + Hash, Cost: Clone + Ord + Zero> MultiPathResult<'_, C, Cost> {
    pub fn apply<T: Clone>(&self, grid: &mut Grid<T>, path: &T) -> Option<()>
    where
        C: Into<Point2<usize>>,
    {
        fn a<C: Clone + PartialEq + Eq + Hash + Into<Point2<usize>>, T: Clone>(
            c: &C,
            came_from: &HashMap<C, Vec<C>>,
            grid: &mut Grid<T>,
            path: &T,
        ) -> Option<()> {
            grid.set(c.clone(), path.clone())?;
            for next in came_from.get(c).into_iter().flatten() {
                a(next, came_from, grid, path);
            }
            Some(())
        }

        a(&self.ends, self.came_from, grid, path)
    }

    pub fn reconstruct_paths(&self) -> Vec<Path<C>> {
        fn a<C: Eq + Hash + Clone>(c: &C, came_from: &HashMap<C, Vec<C>>) -> Vec<Path<C>> {
            let mut paths = vec![];
            match came_from.get(c) {
                None => paths.push(Path(vec![c.clone()])),
                Some(next_list) => {
                    for child in next_list {
                        let ans = a(child, came_from);
                        for mut recieved in ans {
                            recieved.0.push(c.clone());
                            paths.push(recieved);
                        }
                    }
                }
            }
            paths
        }
        a(&self.ends, self.came_from)
    }

    pub fn end_score(&self) -> &Cost {
        self.scores.get(&self.ends).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{Dir, Entity};

    use super::*;

    #[test]
    fn test() {
        let grid = Grid::from_chars_transpose(
            "
##############
#...........E#
#.########.#.#
#.#........#.#
#.#........#.#
#.#........#.#
#.#........#.#
#.#.....####.#
#.#.....#....#
#.#######.##.#
#S...........#
##############
        "
            .trim(),
        )
        .unwrap();

        let start = grid.find(&'S').unwrap();
        let end = grid.find(&'E').unwrap();

        let mut finder = AStarMulti::new(
            vec![Entity::new_on_grid(start, Dir::East, &grid).unwrap()],
            |c| c.pos() == end,
            |en| {
                Dir::ORTHO
                    .into_iter()
                    .filter_map(|dir| en.set_dir(dir).step_bounded())
                    .filter(|en| *grid.get(*en).unwrap() != '#')
                    .map(|en| (en, 1))
                    .collect_vec()
            },
            |_| 0,
        );
        while let Some(next) = finder.next() {
            let result = finder.reconstruct(next);
        }
        // finder.by_ref().map(|x| finder.reconstruct(x));
        while let Some(end) = finder.next() {
            let result = finder.reconstruct(end);
            println!("{}", result.end_score());
            for path in result.reconstruct_paths() {
                let mut grid = grid.clone();
                grid.apply(path.0.clone(), &'O');
                println!("{grid}");
                // result.reset_path(path);
            }
        }
        // res1.apply(&mut grid, &'O');

        // println!("{}", res1.end_score());
    }
}
