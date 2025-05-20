use crate::Point2;

#[derive(Debug)]
pub struct Path<C>(pub Vec<C>);

impl<C: Into<Point2<usize>> + PartialEq + Eq + Copy> IntoIterator for Path<C> {
    type Item = C;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
