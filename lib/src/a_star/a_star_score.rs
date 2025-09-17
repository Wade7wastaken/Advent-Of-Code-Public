use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use num::Zero;

use super::node::Node;

pub fn a_star_score<
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
) -> Option<Cost> {
    let mut open_set = BinaryHeap::with_capacity(starts.len());
    let mut g_score = HashMap::with_capacity(starts.len());

    for start in starts {
        open_set.push(Node {
            data: start.clone(),
            cost: h(&start),
        });
        g_score.insert(start, Cost::zero());
    }

    while let Some(Node { data: node, .. }) = open_set.pop() {
        if (end_cond)(&node) {
            return Some(g_score.get(&node).unwrap().clone());
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
                open_set.push(Node {
                    data: neighbor.clone(),
                    cost: tent_g_score + (h)(&neighbor),
                });
            }
        }
    }
    None
}
