use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    vec,
};

use itertools::Itertools;

use crate::{tern, ConditionalRev, CountWhere, Dir, Entity, Point2};

mod inner {
    use crate::Point2;

    // width is the width of one child (row)
    fn transpose<T: Clone>(grid: &[Vec<T>], width: usize) -> Vec<Vec<T>> {
        (0..width)
            .map(|i| grid.iter().map(|row| row[i].clone()).collect())
            .collect()
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum PoisonState {
        Rows, // Mut ref to cols, cols is canonical, rows is poisoned, copy cols into rows
        Cols, // Mut ref to rows, rows is canonical, cols is poisoned, copy rows into cols
        None, // both are the same
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

    impl<T: Clone> InnerGrid<T> {
        pub fn new_transpose(grid: Vec<Vec<T>>) -> Option<Self> {
            let height = grid.len();
            let width = grid.first().map_or(0, Vec::len);

            if !grid.iter().all(|r| r.len() == width) {
                return None;
            }

            let cols = transpose(&grid, width);

            Some(Self {
                rows: grid,
                cols,
                width,
                height,
                poison: PoisonState::None,
            })
        }

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

        pub fn into_rows(self) -> Vec<Vec<T>> {
            self.assert_rows();
            self.rows
        }

        pub fn into_cols(self) -> Vec<Vec<T>> {
            self.assert_cols();
            self.cols
        }

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

        // returns poison state to None
        pub fn retranspose(&mut self) {
            match self.poison {
                PoisonState::Rows => {
                    // move transposed into regular
                    self.rows = transpose(&self.cols, self.height);
                }
                PoisonState::Cols => {
                    // move regular into transposed
                    self.cols = transpose(&self.rows, self.width);
                }
                PoisonState::None => {}
            }
            self.poison = PoisonState::None;
        }

        // de-poisons rows
        pub fn retranspose_rows(&mut self) {
            if self.poison == PoisonState::Rows {
                // move transposed into regular
                self.rows = transpose(&self.cols, self.height);
                self.poison = PoisonState::None;
            }
        }

        // de-poisons cols
        pub fn retranspose_cols(&mut self) {
            if self.poison == PoisonState::Cols {
                // move regular into transposed
                self.cols = transpose(&self.rows, self.width);
                self.poison = PoisonState::None;
            }
        }

        pub fn set(&mut self, p: impl Into<Point2<usize>>, v: T) -> Option<()> {
            let Point2 { x, y } = p.into();
            match self.poison {
                PoisonState::Rows => {
                    *self.cols.get_mut(x)?.get_mut(y)? = v;
                }
                PoisonState::Cols => {
                    *self.rows.get_mut(y)?.get_mut(x)? = v.clone();
                }
                PoisonState::None => {
                    *self.rows.get_mut(y)?.get_mut(x)? = v.clone();
                    *self.cols.get_mut(x)?.get_mut(y)? = v;
                }
            }
            Some(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T: Clone>(inner::InnerGrid<T>);

impl<T: Clone> Grid<T> {
    #[must_use]
    pub fn new_transpose(grid: Vec<Vec<T>>) -> Option<Self> {
        inner::InnerGrid::new_transpose(grid).map(|g| Self(g))
    }

    #[must_use]
    pub fn new(grid: Vec<Vec<T>>) -> Option<Self> {
        inner::InnerGrid::new(grid).map(|g| Self(g))
    }

    pub fn from_double_iter_transpose(
        it: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
    ) -> Option<Self> {
        Self::new_transpose(it.into_iter().map(|f| f.into_iter().collect()).collect())
    }

    pub fn from_double_iter(
        it: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
    ) -> Option<Self> {
        Self::new(it.into_iter().map(|f| f.into_iter().collect()).collect())
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.0.width()
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.0.height()
    }

    pub fn map<F: Clone>(self, f: impl Fn(T) -> F) -> Grid<F> {
        Grid::new_transpose(
            self.0
                .into_rows()
                .into_iter()
                .map(|row| row.into_iter().map(&f).collect())
                .collect(),
        )
        .unwrap()
    }

    pub fn get(&self, p: impl Into<Point2<usize>>) -> Option<&T> {
        let Point2 { x, y } = p.into();
        self.0.get_rows().get(y)?.get(x)
    }

    pub fn get_mut(&mut self, p: impl Into<Point2<usize>>) -> Option<&mut T> {
        let Point2 { x, y } = p.into();
        self.0.get_rows_mut().get_mut(y)?.get_mut(x)
    }

    pub fn get_offset(&self, p: impl Into<Point2<usize>>, offset: Dir) -> Option<&T> {
        let p: Point2<_> = p.into();
        p.apply(offset).and_then(|p| self.get(p))
    }

    pub fn set(&mut self, p: impl Into<Point2<usize>>, v: T) -> Option<()> {
        self.0.set(p, v)
    }

    pub fn update(&mut self, p: impl Into<Point2<usize>>, f: impl Fn(&T) -> T) -> Option<()> {
        let p: Point2<_> = p.into();
        let cur = self.get(p)?;
        let updated = f(cur);
        self.set(p, updated);
        Some(())
    }

    pub fn apply(
        &mut self,
        cells: impl IntoIterator<Item = impl Into<Point2<usize>>>,
        path: &T,
    ) -> &mut Self {
        for p in cells {
            self.set(p.into(), path.clone());
        }
        self
    }

    #[must_use]
    pub fn applied(
        mut self,
        cells: impl IntoIterator<Item = impl Into<Point2<usize>>>,
        path: &T,
    ) -> Self {
        self.apply(cells, path);
        self
    }

    #[must_use]
    pub fn rows(&self) -> &Vec<Vec<T>> {
        self.0.get_rows()
    }

    #[must_use]
    pub fn rows_mut(&mut self) -> &mut Vec<Vec<T>> {
        self.0.get_rows_mut()
    }

    #[must_use]
    pub fn into_rows(self) -> Vec<Vec<T>> {
        self.0.into_rows()
    }

    #[must_use]
    pub fn into_rows_iter(self) -> vec::IntoIter<Vec<T>> {
        self.0.into_rows().into_iter()
    }

    #[must_use]
    pub fn row(&self, row: usize) -> Option<&Vec<T>> {
        self.0.get_rows().get(row)
    }

    #[must_use]
    pub fn row_mut(&mut self, row: usize) -> Option<&mut Vec<T>> {
        self.0.get_rows_mut().get_mut(row)
    }

    #[must_use]
    pub fn cols(&self) -> &Vec<Vec<T>> {
        self.0.get_cols()
    }

    #[must_use]
    pub fn cols_mut(&mut self) -> &mut Vec<Vec<T>> {
        self.0.get_cols_mut()
    }

    #[must_use]
    pub fn into_cols(self) -> Vec<Vec<T>> {
        self.0.into_cols()
    }

    #[must_use]
    pub fn into_cols_iter(self) -> vec::IntoIter<Vec<T>> {
        self.0.into_cols().into_iter()
    }

    #[must_use]
    pub fn col(&self, col: usize) -> Option<&Vec<T>> {
        self.0.get_cols().get(col)
    }

    #[must_use]
    pub fn col_mut(&mut self, col: usize) -> Option<&mut Vec<T>> {
        self.0.get_cols_mut().get_mut(col)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Point2<usize>, &T)> + Clone {
        self.0.get_rows().iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| (Point2::new(x, y), c))
        })
    }

    pub fn into_enumerate(self) -> impl Iterator<Item = (Point2<usize>, T)> {
        self.0
            .into_rows()
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(x, c)| (Point2::new(x, y), c))
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.get_rows().iter().flat_map(|row| row.iter())
    }

    pub fn retranspose(&mut self) {
        self.0.retranspose();
    }

    // de-poisons rows
    pub fn retranspose_rows(&mut self) {
        self.0.retranspose_rows();
    }

    // de-poisons cols
    pub fn retranspose_cols(&mut self) {
        self.0.retranspose_cols();
    }

    // pub fn transpose(&mut self) {
    //     std::mem::swap(&mut self.grid, &mut self.transposed);
    // }

    // #[must_use]
    // pub fn into_transposed(mut self) -> Self {
    //     self.transpose();
    //     self
    // }

    pub fn count_where(&self, cmp: impl Fn(&T) -> bool) -> usize {
        self.0
            .get_rows()
            .iter()
            .map(|row| row.iter().count_where(&cmp))
            .sum()
    }

    pub fn count(&self, cmp: &T) -> usize
    where
        T: PartialEq,
    {
        self.count_where(|c| c == cmp)
    }

    pub fn find(&self, cell: &T) -> Option<Point2<usize>>
    where
        T: PartialEq,
    {
        self.enumerate().find_map(|(p, c)| (c == cell).then_some(p))
    }

    pub fn find_all<'a>(
        &'a self,
        cell: &'a T,
    ) -> impl Iterator<Item = Point2<usize>> + use<'a, T> + Clone
    where
        T: PartialEq,
    {
        self.enumerate()
            .filter_map(move |(p, c)| (c == cell).then_some(p))
    }

    pub fn apply_gravity(&mut self, dir: Dir, air_cell: &T, stationary_cells: &[T]) -> &mut Self
    where
        T: PartialEq,
    {
        let is_reversed = dir == Dir::NORTH || dir == Dir::WEST;
        let is_sideways = dir == Dir::EAST || dir == Dir::WEST;

        let (perp_len, par_len) = Point2::new(self.0.width(), self.0.height())
            .swap_if(is_sideways)
            .tuple();

        // converts par, perp to (x, y)
        let to_point = |par, perp| Point2::new(perp, par).swap_if(is_sideways);

        // applies an offset in the forward direction
        let offset = |i| if is_reversed { i - 1 } else { i + 1 };
        // applies an offset in the backward direction
        let offset_reversed = |i| if is_reversed { i + 1 } else { i - 1 };

        for par in (0..par_len).rev_if(!is_reversed).skip(1) {
            for perp in (0..perp_len).clone() {
                let p = to_point(par, perp);
                let c = self.get(p).unwrap();
                if c == air_cell || stationary_cells.contains(c) {
                    continue;
                }

                let new_par_i = tern!(is_reversed, 0..par, offset(par)..par_len)
                    .rev_if(is_reversed)
                    .find(|i| self.get(to_point(*i, perp)).unwrap() != air_cell)
                    .map_or(tern!(is_reversed, 0, par_len - 1), offset_reversed);

                let c = c.clone();

                self.set(p, air_cell.clone()).unwrap();
                self.set(to_point(new_par_i, perp), c).unwrap();
            }
        }
        self
    }

    #[must_use]
    pub fn applied_gravity(mut self, dir: Dir, air_cell: &T, stationary_cells: &[T]) -> Self
    where
        T: PartialEq,
    {
        self.apply_gravity(dir, air_cell, stationary_cells);
        self
    }

    pub fn fill(&mut self, start: impl Into<Point2<usize>>, new: &T) -> &mut Self
    where
        T: PartialEq,
    {
        let p = start.into();
        let target = self.get(p).unwrap().clone();
        let mut queue = vec![Entity::new_on_grid(p, Dir::NORTH, self).unwrap()];

        while let Some(en) = queue.pop() {
            let c = self.get_mut(en.pos()).unwrap();
            if *c != target {
                continue;
            }
            *c = new.clone();
            for dir in Dir::ORTHO {
                if let Some(new) = en.set_dir(dir).step_bounded() {
                    queue.push(new);
                }
            }
        }

        self
    }

    #[must_use]
    pub fn filled(mut self, start: impl Into<Point2<usize>>, new: &T) -> Self
    where
        T: PartialEq,
    {
        self.fill(start, new);
        self
    }

    pub fn surrounding(&self, p: impl Into<Point2<usize>>, option: Surround) -> Surrounding<'_, T> {
        let p: Point2<_> = p.into();

        let offsets: &[Dir] = match option {
            Surround::All => &Dir::SURROUNDING,
            Surround::Corners => &Dir::CORNERS,
            Surround::Ortho => &Dir::ORTHO_SNAKE,
        };
        let a = offsets
            .iter()
            .copied()
            .filter_map(move |offset| {
                p.apply(offset)
                    .and_then(|q| Some((q, offset, self.get(q)?)))
            })
            .collect();

        Surrounding(a)
    }
}

impl Grid<char> {
    #[must_use]
    pub fn from_chars(chars: &str) -> Option<Self> {
        Self::from_double_iter_transpose(chars.split_whitespace().map(|row| row.trim().chars()))
    }

    #[must_use]
    pub fn from_chars_gaps(chars: &str, x: usize, y: usize, dx: usize, dy: usize) -> Option<Self> {
        Self::from_double_iter_transpose(
            chars
                .lines()
                .skip(y)
                .step_by(dy)
                .map(|row| row.chars().skip(x).step_by(dx)),
        )
    }
}

impl Grid<u8> {
    #[must_use]
    pub fn from_bytes(chars: &str) -> Option<Self> {
        Self::from_double_iter_transpose(chars.split_whitespace().map(|row| row.bytes()))
    }
}

impl<'a> Grid<&'a str> {
    #[must_use]
    pub fn from_str(str: &'a str, row_sep: &str, cell_sep: &str) -> Option<Self> {
        Self::from_double_iter_transpose(str.split(row_sep).map(|row| row.split(cell_sep)))
    }
}

impl<T: Clone> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::iter::FlatMap<
        vec::IntoIter<Vec<T>>,
        vec::IntoIter<T>,
        fn(Vec<T>) -> <Vec<T> as IntoIterator>::IntoIter,
    >;
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_rows()
            .into_iter()
            .flat_map(IntoIterator::into_iter)
    }
}

impl<T: Clone + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_stringed = self
            .0
            .get_rows()
            .iter()
            .map(|row| row.iter().map(ToString::to_string).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let max_len = to_stringed
            .iter()
            .flat_map(|row| row.iter().map(String::len))
            .max()
            .unwrap_or(0);

        let result = to_stringed
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| format!("{cell:^max_len$}"))
                    .join(if f.alternate() { " " } else { "" })
            })
            .join("\n");

        write!(f, "{result}")
    }
}

impl<T: Clone + Hash> Hash for Grid<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get_rows().hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Surround {
    All,
    Ortho,
    Corners,
}

pub struct Surrounding<'a, T>(Vec<(Point2<usize>, Dir, &'a T)>);

impl<'a, T> IntoIterator for Surrounding<'a, T> {
    type Item = (Point2<usize>, Dir, &'a T);
    type IntoIter = vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> Surrounding<'a, T> {
    pub fn values(self) -> impl Iterator<Item = &'a T> {
        self.0.into_iter().map(|(_, _, c)| c)
    }

    pub fn points(self) -> impl Iterator<Item = Point2<usize>> + use<'a, T> {
        self.0.into_iter().map(|(p, _, _)| p)
    }

    pub fn dirs(self) -> impl Iterator<Item = Dir> + use<'a, T> {
        self.0.into_iter().map(|(_, d, _)| d)
    }

    pub fn enumerate(self) -> impl Iterator<Item = (Point2<usize>, &'a T)> {
        self.0.into_iter().map(|(p, _, c)| (p, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_grid() {
        let grid: Grid<i32> = Grid::new_transpose(vec![]).unwrap();
        assert_eq!(grid.0.width(), 0);
        assert_eq!(grid.0.height(), 0);
        assert_eq!(grid.get((0, 0)), None);
        assert_eq!(grid.get((1, 1)), None);
    }

    #[test]
    fn uneven_grid() {
        let grid = Grid::new_transpose(vec![vec![1, 2], vec![1, 2, 3]]);
        assert!(grid.is_none());
    }

    #[test]
    fn from_iter() {
        let grid = Grid::from_double_iter_transpose(["abc".chars(), "def".chars()]).unwrap();
        assert_eq!(grid.get((0, 0)), Some(&'a'));
        assert_eq!(grid.get((2, 1)), Some(&'f'));
    }

    #[test]
    fn from_strs() {
        let grid = Grid::from_chars(
            "abc
def
ghi",
        )
        .unwrap();
        assert_eq!(grid.get((0, 0)), Some(&'a'));
        assert_eq!(grid.get((2, 2)), Some(&'i'));

        let grid = Grid::from_str("a b c,d e f,g h i", ",", " ").unwrap();
        assert_eq!(grid.get((0, 0)), Some(&"a"));
        assert_eq!(grid.get((2, 2)), Some(&"i"));

        let grid = Grid::from_bytes(
            "abc
def
ghi",
        )
        .unwrap();
        assert_eq!(grid.get((0, 0)), Some(&b'a'));
        assert_eq!(grid.get((2, 2)), Some(&b'i'));
    }

    #[test]
    fn find() {
        let grid = Grid::from_double_iter_transpose(["abc".chars(), "def".chars(), "ghi".chars()])
            .unwrap();
        assert_eq!(grid.find(&'i'), Some((2, 2).into()));
        assert_eq!(grid.find(&'j'), None);
    }

    #[test]
    fn gravity() {
        let grid = Grid::from_chars(
            "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
..........
",
        )
        .unwrap();

        // North
        let answer = "
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
..........
";
        assert_eq!(
            grid.clone().apply_gravity(Dir::NORTH, &'.', &['#']).rows(),
            &answer
                .trim()
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );

        // South
        let answer = "
.....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
O.....OO..
#O...###..
#OO..#....
.OO.O....O
";
        assert_eq!(
            grid.clone().apply_gravity(Dir::SOUTH, &'.', &['#']).rows(),
            &answer
                .trim()
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );

        // East
        let answer = "
....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....
..........
";
        assert_eq!(
            grid.clone().apply_gravity(Dir::EAST, &'.', &['#']).rows(),
            &answer
                .trim()
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );

        // West
        let answer = "
O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....
..........
";
        assert_eq!(
            grid.clone().apply_gravity(Dir::WEST, &'.', &['#']).rows(),
            &answer
                .trim()
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );
    }

    macro_rules! enumerate_format {
        ( $( ($x:expr, $y:expr, $dir:expr, $ch:expr) ),* ) => {
            vec![
                $(
                    ((($x, $y).into()), $dir, &$ch)
                ),*
            ]
        };
    }

    #[test]
    fn surrounding_all() {
        let grid = Grid::from_double_iter_transpose(["abc".chars(), "def".chars(), "ghi".chars()])
            .unwrap();

        assert_eq!(
            grid.surrounding((1, 1), Surround::All).0,
            enumerate_format![
                (0, 0, Dir::new(-1, -1), 'a'),
                (1, 0, Dir::new(0, -1), 'b'),
                (2, 0, Dir::new(1, -1), 'c'),
                (0, 1, Dir::new(-1, 0), 'd'),
                (2, 1, Dir::new(1, 0), 'f'),
                (0, 2, Dir::new(-1, 1), 'g'),
                (1, 2, Dir::new(0, 1), 'h'),
                (2, 2, Dir::new(1, 1), 'i')
            ]
        );
        assert_eq!(
            grid.surrounding((0, 0), Surround::All).0,
            enumerate_format![
                (1, 0, Dir::new(1, 0), 'b'),
                (0, 1, Dir::new(0, 1), 'd'),
                (1, 1, Dir::new(1, 1), 'e')
            ]
        );
        assert_eq!(
            grid.surrounding((2, 2), Surround::All).0,
            enumerate_format![
                (1, 1, Dir::new(-1, -1), 'e'),
                (2, 1, Dir::new(0, -1), 'f'),
                (1, 2, Dir::new(-1, 0), 'h')
            ]
        );
    }

    #[test]
    fn surrounding_ortho() {
        let grid = Grid::from_double_iter_transpose(["abc".chars(), "def".chars(), "ghi".chars()])
            .unwrap();

        assert_eq!(
            grid.surrounding((1, 1), Surround::Ortho).0,
            enumerate_format![
                (1, 0, Dir::NORTH, 'b'),
                (0, 1, Dir::WEST, 'd'),
                (2, 1, Dir::EAST, 'f'),
                (1, 2, Dir::SOUTH, 'h')
            ]
        );
        assert_eq!(
            grid.surrounding((0, 0), Surround::Ortho).0,
            enumerate_format![(1, 0, Dir::EAST, 'b'), (0, 1, Dir::SOUTH, 'd')]
        );
        assert_eq!(
            grid.surrounding((2, 2), Surround::Ortho).0,
            enumerate_format![(2, 1, Dir::NORTH, 'f'), (1, 2, Dir::WEST, 'h')]
        );
    }

    #[test]
    fn surrounding_corners() {
        let grid = Grid::from_double_iter_transpose(["abc".chars(), "def".chars(), "ghi".chars()])
            .unwrap();

        assert_eq!(
            grid.surrounding((1, 1), Surround::Corners).0,
            enumerate_format![
                (0, 0, Dir::new(-1, -1), 'a'),
                (2, 0, Dir::new(1, -1), 'c'),
                (0, 2, Dir::new(-1, 1), 'g'),
                (2, 2, Dir::new(1, 1), 'i')
            ]
        );
        assert_eq!(
            grid.surrounding((0, 0), Surround::Corners).0,
            enumerate_format![(1, 1, Dir::new(1, 1), 'e')]
        );
        assert_eq!(
            grid.surrounding((2, 2), Surround::Corners).0,
            enumerate_format![(1, 1, Dir::new(-1, -1), 'e')]
        );
    }

    #[test]
    fn display_even_length() {
        let grid = Grid::from_double_iter_transpose(["abc".chars(), "def".chars(), "ghi".chars()])
            .unwrap();
        assert_eq!(grid.to_string(), "abc\ndef\nghi");
        assert_eq!(format!("{grid:#}"), "a b c\nd e f\ng h i");
    }

    #[test]
    fn display_different_length() {
        let grid = Grid::from_double_iter_transpose([
            ["a", "ab", "abc"],
            ["def", "de", "d"],
            ["gh", "ghi", "g"],
        ])
        .unwrap();
        assert_eq!(grid.to_string(), " a ab abc\ndefde  d \ngh ghi g ");
        assert_eq!(format!("{grid:#}"), " a  ab  abc\ndef de   d \ngh  ghi  g ");
    }
}
