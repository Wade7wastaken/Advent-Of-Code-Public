use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use num::Zero;

use crate::{Grid, Point2};

use super::{node::Node, path::Path};

pub fn a_star_single<
    C: Clone + PartialEq + Eq + Hash,
    Cost: Clone + Ord + Zero,
    EndCond: FnMut(&C) -> bool,
    I: IntoIterator<Item = (C, Cost)>,
    Neighbors: FnMut(&C) -> I,
    H: FnMut(&C) -> Cost,
>(
    starts: Vec<C>,
    mut end_cond: EndCond,
    mut neighbors: Neighbors,
    mut h: H,
) -> Option<SinglePathResult<C, Cost>> {
    let mut open_set = BinaryHeap::with_capacity(starts.len());
    let mut g_score = HashMap::with_capacity(starts.len());

    for start in starts {
        open_set.push(Node {
            data: start.clone(),
            cost: h(&start),
        });
        g_score.insert(start, Cost::zero());
    }

    let mut came_from = HashMap::new();

    while let Some(Node { data: node, .. }) = open_set.pop() {
        if (end_cond)(&node) {
            return Some(SinglePathResult {
                end: node,
                scores: g_score,
                came_from,
            });
        }

        for (neighbor, move_cost) in (neighbors)(&node) {
            // g_score[node] will never be none because everything in open_set will be in in g_score
            let tent_g_score = g_score.get(&node).unwrap().clone() + move_cost;
            let actual_g_score = g_score.get(&neighbor);
            let is_better = actual_g_score
                .map(|actual| tent_g_score.cmp(actual))
                .is_none_or(Ordering::is_lt);

            if is_better {
                g_score.insert(neighbor.clone(), tent_g_score.clone());
                came_from.insert(neighbor.clone(), node.clone());
                let new_cost = tent_g_score + (h)(&neighbor);
                open_set.push(Node {
                    data: neighbor,
                    cost: new_cost,
                });
            }
        }
    }
    None
}

pub struct SinglePathResult<C, Cost> {
    end: C,
    scores: HashMap<C, Cost>,
    came_from: HashMap<C, C>,
}

impl<C: Clone + PartialEq + Eq + Hash, Cost: Clone + Ord + Zero> SinglePathResult<C, Cost> {
    pub fn apply<T: Clone>(&self, grid: &mut Grid<T>, path: &T) -> Option<()>
    where
        C: Into<Point2<usize>>,
    {
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

    pub const fn end(&self) -> &C {
        &self.end
    }

    pub fn end_score(&self) -> &Cost {
        self.scores.get(&self.end).unwrap()
    }
}
