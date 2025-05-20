use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<C, Cost> {
    pub data: C,
    pub cost: Cost,
}

impl<C: Eq, Cost: Ord> Ord for Node<C, Cost> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse for min-heap behavior
    }
}

impl<C: Eq, Cost: Ord> PartialOrd for Node<C, Cost> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
