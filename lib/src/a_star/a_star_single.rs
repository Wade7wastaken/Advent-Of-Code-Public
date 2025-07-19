use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
};

use num::Zero;

use crate::{Grid, Point2};

use super::{node::Node, path::Path};

pub struct AStarSingle<C, Cost, EndCond, Neighbors, H> {
    // constants
    end_cond: EndCond,
    neighbors: Neighbors,
    h: H,

    open_set: BinaryHeap<Node<C, Cost>>, // the set of cells we need to look at, ordered by node.cost
    g_score: HashMap<C, Cost>,           // score for traveling to a specific node
    came_from: HashMap<C, C>,
}

impl<
        C: Into<Point2<usize>> + PartialEq + Eq + Hash + Clone,
        Cost: Ord + Zero,
        EndCond: Fn(&C) -> bool,
        I: IntoIterator<Item = (C, Cost)>,
        Neighbors: Fn(&C) -> I,
        H: Fn(&C) -> Cost,
    > AStarSingle<C, Cost, EndCond, Neighbors, H>
{
    pub fn new(starts: Vec<C>, end_cond: EndCond, neighbors: Neighbors, h: H) -> Self {
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
}

impl<
        C: Into<Point2<usize>> + PartialEq + Eq + Hash + Clone,
        Cost: Copy + Ord + Zero + Add,
        EndCond: Fn(&C) -> bool,
        I: IntoIterator<Item = (C, Cost)>,
        Neighbors: Fn(&C) -> I,
        H: Fn(&C) -> Cost,
    > Iterator for AStarSingle<C, Cost, EndCond, Neighbors, H>
{
    type Item = SinglePathResult<C, Cost>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut counter = 0;
        while let Some(Node { data: node, .. }) = self.open_set.pop() {
            if (self.end_cond)(&node) {
                println!("traversed {counter} nodes");
                return Some(SinglePathResult {
                    end: node,
                    scores: self.g_score.clone(),
                    came_from: self.came_from.clone(),
                });
            }

            for (neighbor, move_cost) in (self.neighbors)(&node) {
                // g_score[node] will never be none because everything in open_set will be in in g_score
                let tentative_g_score = *self.g_score.get(&node).unwrap() + move_cost;
                let actual_g_score = self.g_score.get(&neighbor);
                let is_better = actual_g_score
                    .map(|actual| tentative_g_score.cmp(actual))
                    .is_none_or(Ordering::is_lt);

                if is_better {
                    self.g_score.insert(neighbor.clone(), tentative_g_score);
                    self.came_from.insert(neighbor.clone(), node.clone());
                    let new_cost = tentative_g_score + (self.h)(&neighbor);
                    self.open_set.push(Node {
                        data: neighbor,
                        cost: new_cost,
                    });
                }
            }
            counter += 1;
        }
        None
    }
}

pub struct SinglePathResult<C, Cost> {
    end: C,
    scores: HashMap<C, Cost>,
    came_from: HashMap<C, C>,
}

impl<C: Into<Point2<usize>> + PartialEq + Eq + Clone + Hash, Cost: Copy> SinglePathResult<C, Cost> {
    pub fn apply<T: Clone>(&self, grid: &mut Grid<T>, path: &T) -> Option<()> {
        let mut current = self.end.clone();
        grid.set(current.clone(), path.clone())?;
        while let Some(next) = self.came_from.get(&current) {
            grid.set(next.clone(), path.clone())?;
            current = next.clone();
        }

        Some(())
    }

    pub fn path(&self) -> Path<C> {
        let mut path = vec![];
        let mut current = self.end.clone();
        path.push(current.clone());
        while let Some(next) = self.came_from.get(&current) {
            path.push(next.clone());
            current = next.clone();
        }
        path.reverse();
        Path(path)
    }

    pub fn end_score(&self) -> Cost {
        *self.scores.get(&self.end).unwrap()
    }
}
