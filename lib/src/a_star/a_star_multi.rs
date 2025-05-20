use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
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
    best_found: Option<(Vec<C>, Cost)>,
}

impl<
        C: Into<Point2<usize>> + PartialEq + Eq + Hash + Copy,
        Cost: Copy + Ord + Zero + Add,
        EndCond: Fn(&C) -> bool,
        I: IntoIterator<Item = (C, Cost)>,
        Neighbors: Fn(&C) -> I,
        H: Fn(&C) -> Cost,
    > AStarMulti<C, Cost, EndCond, Neighbors, H>
{
    pub fn new(starts: Vec<C>, end_cond: EndCond, neighbors: Neighbors, h: H) -> Self {
        let mut open_set = BinaryHeap::with_capacity(starts.len());
        let mut g_score = HashMap::with_capacity(starts.len());

        for start in starts {
            open_set.push(Node {
                data: start,
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
            best_found: None,
        }
    }

    pub fn run(&mut self) -> Option<MultiPathResult<C, Cost>> {
        let mut counter = 0;
        while let Some(Node { data: node, .. }) = self.open_set.pop() {
            if (self.end_cond)(&node) {
                println!("traversed {counter} nodes");
                let cost = *self.g_score.get(&node).unwrap();

                match &mut self.best_found {
                    None => {
                        self.best_found = Some((vec![node], cost));
                    }
                    Some(best_found) => {
                        if best_found.1 == cost {
                            best_found.0.push(node);
                        } else {
                            debug_assert!(best_found.1 < cost);
                            return Some(MultiPathResult {
                                ends: best_found.0.clone(),
                                final_score: best_found.1,
                                // scores: self.g_score.clone(),
                                came_from: self.came_from.clone(),
                            });
                        }
                    }
                }
            }

            for (neighbor, move_cost) in (self.neighbors)(&node) {
                // g_score[node] will never be none because everything in open_set will be in in g_score
                let tentative_g_score = *self.g_score.get(&node).unwrap() + move_cost;
                let actual_g_score = self.g_score.get(&neighbor);
                let cmp = actual_g_score.map(|actual| tentative_g_score.cmp(actual));

                match cmp {
                    None | Some(Ordering::Less) => {
                        self.g_score.insert(neighbor, tentative_g_score);
                        self.came_from.insert(neighbor, vec![node]);
                        self.open_set.push(Node {
                            data: neighbor,
                            cost: tentative_g_score + (self.h)(&neighbor),
                        });
                    }
                    Some(Ordering::Equal) => {
                        self.came_from.get_mut(&neighbor).unwrap().push(node);
                    }
                    _ => {}
                }
            }
            counter += 1;
        }
        let best_found = self.best_found.as_ref()?;
        Some(MultiPathResult {
            ends: best_found.0.clone(),
            final_score: best_found.1,
            // scores: self.g_score.clone(),
            came_from: self.came_from.clone(),
        })
    }
}

#[derive(Debug)]
pub struct MultiPathResult<C, Cost> {
    ends: Vec<C>,
    final_score: Cost,
    // scores: HashMap<C, Cost>,
    came_from: HashMap<C, Vec<C>>,
}

impl<C: Into<Point2<usize>> + PartialEq + Eq + Copy + Hash, Cost: Copy> MultiPathResult<C, Cost> {
    pub fn apply<T: Clone>(&self, grid: &mut Grid<T>, path: &T) -> Option<()> {
        fn a<C: Into<Point2<usize>> + Eq + Hash + Copy, T: Clone>(
            next: &Vec<C>,
            came_from: &HashMap<C, Vec<C>>,
            grid: &mut Grid<T>,
            path: &T,
        ) -> Option<()> {
            for n in next {
                grid.set(*n, path.clone())?;
                let empty = vec![];
                let next_list = came_from.get(n).unwrap_or(&empty);
                a(next_list, came_from, grid, path)?;
            }
            Some(())
        }
        for end in &self.ends {
            grid.set(*end, path.clone());
        }
        a(&self.ends, &self.came_from, grid, path)
    }

    pub fn reconstruct_paths(&self) -> Vec<Path<C>> {
        fn a<C: Into<Point2<usize>> + Eq + Hash + Copy>(
            next: &Vec<C>,
            came_from: &HashMap<C, Vec<C>>,
        ) -> Vec<Path<C>> {
            let mut paths = vec![];
            for n in next {
                match came_from.get(n) {
                    None => paths.push(Path(vec![*n])),
                    Some(next_list) => {
                        let ans = a(next_list, came_from);
                        for mut recieved in ans {
                            recieved.0.push(*n);
                            paths.push(recieved);
                        }
                    }
                }
            }
            paths
        }
        a(&self.ends, &self.came_from)
    }

    pub fn end_score(&self) -> Cost {
        self.final_score
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{Dir, Entity};

    use super::*;

    #[test]
    fn test() {
        let mut grid = Grid::from_chars(
            "
##############
#...........E#
#.########.#.#
#.#........#.#
#.#........#.#
#.#........#.#
#.#........#.#
#.#####.##.#.#
#.#...#.#....#
#.#.#.###.##.#
#S...........#
##############
"
            .trim(),
        )
        .unwrap();

        let start = grid.find(&'S').unwrap();
        let end = grid.find(&'E').unwrap();

        let mut finder = AStarMulti::new(
            vec![Entity::new_on_grid(start, Dir::EAST, &grid).unwrap()],
            |c| c.pos() == end,
            |en| {
                Dir::ORTHO
                    .into_iter()
                    .filter_map(|dir| en.set_dir(dir).step_bounded())
                    .filter(|en| *grid.get(*en).unwrap() != '#')
                    .map(|en| (en, 1))
                    .collect_vec()
            },
            |en| en.pos().manhattan_dist(end) as u32,
        );
        let res1 = finder.run().unwrap();
        res1.apply(&mut grid, &'O');

        println!("{grid}");
        println!("{}", res1.end_score());
    }
}
