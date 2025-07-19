use itertools::Itertools;

use crate::Point2;

// width is the width of one child (row)
fn transpose<T>(original: Vec<Vec<T>>, width: usize) -> Vec<Vec<T>> {
    let mut transposed = (0..width)
        .map(|_| Vec::with_capacity(original.len()))
        .collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip_eq(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PoisonState {
    Rows, // Mut ref to cols, cols is canonical, rows is poisoned, copy cols into rows
    Cols, // Mut ref to rows, rows is canonical, cols is poisoned, copy rows into cols
    None, // both are the same
}

// defaults to rows if poison is None
pub enum RowsOrCols<T> {
    Rows(T),
    Cols(T),
}

impl<T> RowsOrCols<T> {
    pub fn either(self) -> T {
        match self {
            RowsOrCols::Rows(rows) => rows,
            RowsOrCols::Cols(cols) => cols,
        }
    }
}

// maybe update regular to upright
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerGrid<T> {
    rows: Vec<Vec<T>>, // width x height
    cols: Vec<Vec<T>>, // height x width
    width: usize,
    height: usize,
    poison: PoisonState,
}

impl<T> InnerGrid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Option<Self> {
        let height = grid.len();
        let width = grid.first().map_or(0, Vec::len);

        if !grid.iter().all(|r| r.len() == width) {
            return None;
        }

        Some(Self {
            rows: grid,
            cols: vec![],
            width,
            height,
            poison: PoisonState::Cols,
        })
    }

    pub unsafe fn new_unchecked(grid: Vec<Vec<T>>) -> Self {
        let height = grid.len();
        let width = grid.first().map_or(0, Vec::len);

        Self {
            rows: grid,
            cols: vec![],
            width,
            height,
            poison: PoisonState::Cols,
        }
    }

    pub fn new_transpose(rows: Vec<Vec<T>>) -> Option<Self>
    where
        T: Clone,
    {
        let height = rows.len();
        let width = rows.first().map_or(0, Vec::len);

        if !rows.iter().all(|r| r.len() == width) {
            return None;
        }

        let cols = transpose(rows.clone(), width);

        Some(Self {
            rows,
            cols,
            width,
            height,
            poison: PoisonState::None,
        })
    }

    pub unsafe fn new_transpose_unchecked(rows: Vec<Vec<T>>) -> Self
    where
        T: Clone,
    {
        let height = rows.len();
        let width = rows.first().map_or(0, Vec::len);

        let cols = transpose(rows.clone(), width);

        Self {
            rows,
            cols,
            width,
            height,
            poison: PoisonState::None,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn assert_rows(&self) {
        assert!(
            self.poison != PoisonState::Rows,
            "Tried to access normal grid when it was poisoned"
        );
    }

    fn assert_cols(&self) {
        assert!(
            self.poison != PoisonState::Cols,
            "Tried to get transposed grid when it was poisoned"
        );
    }

    pub fn get_rows(&self) -> &Vec<Vec<T>> {
        self.assert_rows();
        &self.rows
    }

    pub fn get_cols(&self) -> &Vec<Vec<T>> {
        self.assert_cols();
        &self.cols
    }

    pub fn get_both(&self) -> RowsOrCols<&Vec<Vec<T>>> {
        match self.poison {
            PoisonState::None => RowsOrCols::Rows(&self.rows),
            PoisonState::Rows => RowsOrCols::Cols(&self.cols),
            PoisonState::Cols => RowsOrCols::Rows(&self.rows),
        }
    }

    pub fn into_rows(self) -> Vec<Vec<T>> {
        self.assert_rows();
        self.rows
    }

    pub fn into_cols(self) -> Vec<Vec<T>> {
        self.assert_cols();
        self.cols
    }

    // pub fn into_both(self) -> RowsOrCols<Vec<Vec<T>>> {
    //     match self.poison {
    //         PoisonState::None => RowsOrCols::Rows(self.rows),
    //         PoisonState::Rows => RowsOrCols::Cols(self.cols),
    //         PoisonState::Cols => RowsOrCols::Rows(self.rows),
    //     }
    // }

    pub fn get_rows_mut(&mut self) -> &mut Vec<Vec<T>> {
        self.assert_rows();
        self.poison = PoisonState::Cols; // transposed is poisoned because we gave a mut ref to normal
        &mut self.rows
    }

    pub fn get_cols_mut(&mut self) -> &mut Vec<Vec<T>> {
        self.assert_cols();
        self.poison = PoisonState::Rows; // regular is poisoned because we gave a mut ref to transposed
        &mut self.cols
    }

    pub fn get_both_mut(&mut self) -> RowsOrCols<&mut Vec<Vec<T>>> {
        match self.poison {
            PoisonState::None => RowsOrCols::Rows(&mut self.rows),
            PoisonState::Rows => RowsOrCols::Cols(&mut self.cols),
            PoisonState::Cols => RowsOrCols::Rows(&mut self.rows),
        }
    }

    // returns poison state to None
    pub fn retranspose(&mut self)
    where
        T: Clone,
    {
        match self.poison {
            PoisonState::Rows => {
                // move transposed into regular
                self.rows = transpose(self.cols.clone(), self.height);
            }
            PoisonState::Cols => {
                // move regular into transposed
                self.cols = transpose(self.rows.clone(), self.width);
            }
            PoisonState::None => {}
        }
        self.poison = PoisonState::None;
    }

    // de-poisons rows
    pub fn retranspose_rows(&mut self)
    where
        T: Clone,
    {
        if self.poison == PoisonState::Rows {
            // move transposed into regular
            self.rows = transpose(self.cols.clone(), self.height);
            self.poison = PoisonState::None;
        }
    }

    // de-poisons cols
    pub fn retranspose_cols(&mut self)
    where
        T: Clone,
    {
        if self.poison == PoisonState::Cols {
            // move regular into transposed
            self.cols = transpose(self.rows.clone(), self.width);
            self.poison = PoisonState::None;
        }
    }

    pub fn transpose(&mut self) {
        std::mem::swap(&mut self.rows, &mut self.cols);
        self.poison = match self.poison {
            PoisonState::Cols => PoisonState::Rows,
            PoisonState::Rows => PoisonState::Cols,
            PoisonState::None => PoisonState::None,
        };
    }

    pub fn set(&mut self, p: impl Into<Point2<usize>>, v: T) -> Option<()>
    where
        T: Clone,
    {
        let Point2 { x, y } = p.into();
        match self.poison {
            PoisonState::Rows => {
                *self.cols.get_mut(x)?.get_mut(y)? = v;
            }
            PoisonState::Cols => {
                *self.rows.get_mut(y)?.get_mut(x)? = v;
            }
            PoisonState::None => {
                *self.rows.get_mut(y)?.get_mut(x)? = v.clone();
                *self.cols.get_mut(x)?.get_mut(y)? = v;
            }
        }
        Some(())
    }

    #[must_use]
    // todo: rewrite to not use new
    pub fn map<F: Clone>(self, f: impl Fn(T) -> F) -> InnerGrid<F> {
        match self.poison {
            PoisonState::Rows => {
                let mapped = self
                    .cols
                    .into_iter()
                    .map(|row| row.into_iter().map(&f).collect())
                    .collect();
                InnerGrid {
                    rows: vec![],
                    cols: mapped,
                    width: self.width,
                    height: self.height,
                    poison: self.poison,
                }
            }
            PoisonState::Cols => {
                let mapped = self
                    .rows
                    .into_iter()
                    .map(|row| row.into_iter().map(&f).collect())
                    .collect();
                InnerGrid {
                    rows: mapped,
                    cols: vec![],
                    width: self.width,
                    height: self.height,
                    poison: self.poison,
                }
            }
            PoisonState::None => {
                let mapped_rows: Vec<Vec<_>> = self
                    .rows
                    .into_iter()
                    .map(|row| row.into_iter().map(&f).collect())
                    .collect();
                let mapped_cols = transpose(mapped_rows.clone(), self.width);
                InnerGrid {
                    rows: mapped_rows,
                    cols: mapped_cols,
                    width: self.width,
                    height: self.height,
                    poison: self.poison,
                }
            }
        }
        // Grid::new(
        //     self.0
        //         .into_rows()
        //         .into_iter()
        //         .map(|row| row.into_iter().map(&f).collect())
        //         .collect(),
        // )
        // .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_empty() {
        let rows: Vec<Vec<i32>> = vec![];
        let width = 0;
        let cols = transpose(rows, width);
        let cmp: Vec<Vec<i32>> = vec![];
        assert_eq!(cols, cmp);
    }

    #[test]
    fn transpose_square() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let width = rows[0].len();
        let cols = transpose(rows, width);
        assert_eq!(cols, vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9],]);
    }

    #[test]
    fn transpose_wide() {
        let rows = vec![vec![1, 2, 3, 4], vec![4, 5, 6, 5], vec![7, 8, 9, 6]];
        let width = rows[0].len();
        let cols = transpose(rows, width);
        assert_eq!(
            cols,
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9], vec![4, 5, 6],]
        );
    }

    #[test]
    fn transpose_long() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![8, 7, 6]];
        let width = rows[0].len();
        let cols = transpose(rows, width);
        assert_eq!(
            cols,
            vec![vec![1, 4, 7, 8], vec![2, 5, 8, 7], vec![3, 6, 9, 6],]
        );
    }
}
