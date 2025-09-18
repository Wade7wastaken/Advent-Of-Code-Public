use std::{fmt::Display, hash::Hash, vec};

use itertools::Itertools;

use crate::{Dir, Entity, IteratorExt, Offset, Point2, point2, tern};

use super::inner;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T>(inner::InnerGrid<T>);

impl<T> Grid<T> {
    #[must_use]
    pub fn new(grid: Vec<Vec<T>>) -> Option<Self> {
        inner::InnerGrid::new(grid).map(|g| Self(g))
    }

    #[must_use]
    pub fn new_transpose(grid: Vec<Vec<T>>) -> Option<Self>
    where
        T: Clone,
    {
        inner::InnerGrid::new_transpose(grid).map(|g| Self(g))
    }

    #[must_use]
    pub fn from_double_iter(
        it: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
    ) -> Option<Self> {
        Self::new(it.into_iter().map(|f| f.into_iter().collect()).collect())
    }

    #[must_use]
    pub fn from_double_iter_transpose(
        it: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
    ) -> Option<Self>
    where
        T: Clone,
    {
        Self::new_transpose(it.into_iter().map(|f| f.into_iter().collect()).collect())
    }

    #[must_use]
    pub fn new_filled(c: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self(unsafe { inner::InnerGrid::new_unchecked(vec![vec![c; width]; height]) })
    }

    #[must_use]
    pub fn new_filled_transpose(c: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self(unsafe { inner::InnerGrid::new_transpose_unchecked(vec![vec![c; width]; height]) })
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.0.width()
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.0.height()
    }

    #[must_use]
    // todo: rewrite to not use new
    pub fn map<F: Clone>(self, f: impl Fn(T) -> F) -> Grid<F> {
        Grid(self.0.map(f))
    }

    #[must_use]
    pub fn get(&self, p: impl Into<Point2<usize>>) -> Option<&T> {
        let Point2 { x, y } = p.into();
        match self.0.get_both() {
            inner::RowsOrCols::Rows(rows) => rows.get(y)?.get(x),
            inner::RowsOrCols::Cols(cols) => cols.get(x)?.get(y),
        }
    }

    #[must_use]
    pub fn get_mut(&mut self, p: impl Into<Point2<usize>>) -> Option<&mut T> {
        let Point2 { x, y } = p.into();
        match self.0.get_both_mut() {
            inner::RowsOrCols::Rows(rows) => rows.get_mut(y)?.get_mut(x),
            inner::RowsOrCols::Cols(cols) => cols.get_mut(x)?.get_mut(y),
        }
    }

    #[must_use]
    pub fn get_offset(&self, p: impl Into<Point2<usize>>, offset: impl Offset) -> Option<&T> {
        let p: Point2<_> = p.into();
        p.apply(offset).and_then(|p| self.get(p))
    }

    #[must_use]
    pub fn get_mut_offset(
        &mut self,
        p: impl Into<Point2<usize>>,
        offset: impl Offset,
    ) -> Option<&mut T> {
        let p: Point2<_> = p.into();
        p.apply(offset).and_then(|p| self.get_mut(p))
    }

    pub fn set(&mut self, p: impl Into<Point2<usize>>, v: T) -> Option<()>
    where
        T: Clone,
    {
        self.0.set(p, v)
    }

    pub fn update(&mut self, p: impl Into<Point2<usize>>, f: impl Fn(&T) -> T) -> Option<()>
    where
        T: Clone,
    {
        let p: Point2<_> = p.into();
        let cur = self.get(p)?;
        let updated = f(cur);
        self.set(p, updated);
        Some(())
    }

    pub fn apply(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point2<usize>>>,
        new_cell: &T,
    ) -> &mut Self
    where
        T: Clone,
    {
        for p in points {
            self.set(p.into(), new_cell.clone());
        }
        self
    }

    #[must_use]
    pub fn applied(
        mut self,
        points: impl IntoIterator<Item = impl Into<Point2<usize>>>,
        new_cell: &T,
    ) -> Self
    where
        T: Clone,
    {
        self.apply(points, new_cell);
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

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Point2<usize>, &mut T)> {
        self.0
            .get_rows_mut()
            .iter_mut()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(x, c)| (Point2::new(x, y), c))
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.get_rows().iter().flat_map(|row| row.iter())
    }

    pub fn retranspose(&mut self)
    where
        T: Clone,
    {
        self.0.retranspose();
    }

    // de-poisons rows
    pub fn retranspose_rows(&mut self)
    where
        T: Clone,
    {
        self.0.retranspose_rows();
    }

    // de-poisons cols
    pub fn retranspose_cols(&mut self)
    where
        T: Clone,
    {
        self.0.retranspose_cols();
    }

    pub const fn transpose(&mut self) {
        self.0.transpose();
    }

    #[must_use]
    pub const fn into_transposed(mut self) -> Self {
        self.transpose();
        self
    }

    #[must_use]
    pub fn subgrid(&self, start_x: usize, start_y: usize, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        let data = (start_y..(start_y + height))
            .map(|y| {
                (start_x..(start_x + width))
                    .map(|x| self.get((x, y)).unwrap())
                    .cloned()
                    .collect_vec()
            })
            .collect_vec();

        Self(unsafe { inner::InnerGrid::new_unchecked(data) })
    }

    pub fn paste(&mut self, other: Self, start_x: usize, start_y: usize) -> Option<()>
    where
        T: Clone,
    {
        let offset = point2(start_x, start_y);
        for (p, c) in other.into_enumerate() {
            self.set(p + offset, c)?;
        }
        Some(())
    }

    pub fn rotate(&mut self) {
        self.transpose();
        self.flip_vertical();
    }

    pub fn flip_vertical(&mut self) {
        for row in self.rows_mut() {
            row.reverse();
        }
    }

    pub fn count_where(&self, cmp: impl Fn(&T) -> bool) -> usize {
        self.0
            .get_both()
            .either()
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
        T: Clone + PartialEq,
    {
        let is_reversed = dir == Dir::North || dir == Dir::West;
        let is_sideways = dir == Dir::East || dir == Dir::West;

        let (perp_len, par_len) = Point2::new(self.0.width(), self.0.height())
            .swap_if(is_sideways)
            .into_tuple();

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
        T: Clone + PartialEq,
    {
        self.apply_gravity(dir, air_cell, stationary_cells);
        self
    }

    pub fn fill(&mut self, start: impl Into<Point2<usize>>, new: &T) -> &mut Self
    where
        T: Clone + PartialEq,
    {
        let p = start.into();
        let target = self.get(p).unwrap().clone();
        let mut queue = vec![Entity::new_on_grid(p, Dir::North, self).unwrap()];

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
        T: Clone + PartialEq,
    {
        self.fill(start, new);
        self
    }

    pub fn with_offsets<const N: usize, D: Offset>(
        &self,
        p: impl Into<Point2<usize>>,
        offsets: [D; N],
    ) -> Surrounding<'_, T, D, N> {
        let p: Point2<_> = p.into();

        Surrounding {
            grid: self,
            offsets,
            p,
        }
    }
}

impl Grid<char> {
    #[must_use]
    pub fn from_chars_transpose(chars: &str) -> Option<Self> {
        Self::from_double_iter_transpose(chars.lines().map(|row| row.chars()))
    }

    #[must_use]
    pub fn from_chars(chars: &str) -> Option<Self> {
        Self::from_double_iter(chars.lines().map(|row| row.chars()))
    }

    #[must_use]
    pub fn from_chars_gaps_transpose(
        chars: &str,
        x: usize,
        y: usize,
        dx: usize,
        dy: usize,
    ) -> Option<Self> {
        Self::from_double_iter_transpose(
            chars
                .lines()
                .skip(y)
                .step_by(dy)
                .map(|row| row.chars().skip(x).step_by(dx)),
        )
    }

    #[must_use]
    pub fn from_chars_gaps(chars: &str, x: usize, y: usize, dx: usize, dy: usize) -> Option<Self> {
        Self::from_double_iter(
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
    pub fn from_bytes_transpose(chars: &str) -> Option<Self> {
        Self::from_double_iter_transpose(chars.split_whitespace().map(|row| row.bytes()))
    }

    #[must_use]
    pub fn from_bytes(chars: &str) -> Option<Self> {
        Self::from_double_iter(chars.split_whitespace().map(|row| row.bytes()))
    }
}

impl<'a> Grid<&'a str> {
    #[must_use]
    pub fn from_str_transpose(str: &'a str, row_sep: &str, cell_sep: &str) -> Option<Self> {
        Self::from_double_iter_transpose(str.split(row_sep).map(|row| row.split(cell_sep)))
    }

    #[must_use]
    pub fn from_str(str: &'a str, row_sep: &str, cell_sep: &str) -> Option<Self> {
        Self::from_double_iter(str.split(row_sep).map(|row| row.split(cell_sep)))
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

pub struct Surrounding<'a, T, D: Offset, const N: usize> {
    grid: &'a Grid<T>,
    p: Point2<usize>,
    offsets: [D; N],
}

impl<'a, T, D: Offset, const N: usize> Surrounding<'a, T, D, N> {
    pub fn values(self) -> impl Iterator<Item = &'a T> {
        self.offsets
            .into_iter()
            .filter_map(move |offset| self.grid.get_offset(self.p, offset))
    }

    pub fn all(self) -> impl Iterator<Item = (Point2<usize>, D, &'a T)> {
        self.offsets.into_iter().filter_map(move |offset| {
            self.p
                .apply(offset)
                .and_then(|p| self.grid.get(p).map(|c| (p, offset, c)))
        })
    }

    pub fn enumerate(self) -> impl Iterator<Item = (Point2<usize>, &'a T)> {
        self.offsets.into_iter().filter_map(move |offset| {
            self.p
                .apply(offset)
                .and_then(|p| self.grid.get(p).map(|c| (p, c)))
        })
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use crate::Vec2;

    use super::*;

    #[test]
    fn empty_grid() {
        let grid: Grid<i32> = Grid::new(vec![]).unwrap();
        assert_eq!(grid.0.width(), 0);
        assert_eq!(grid.0.height(), 0);
        assert_eq!(grid.get((0, 0)), None);
        assert_eq!(grid.get((1, 1)), None);

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
        let grid = Grid::from_chars_transpose(
            "abc
def
ghi",
        )
        .unwrap();
        assert_eq!(grid.get((0, 0)), Some(&'a'));
        assert_eq!(grid.get((2, 2)), Some(&'i'));

        let grid = Grid::from_str_transpose("a b c,d e f,g h i", ",", " ").unwrap();
        assert_eq!(grid.get((0, 0)), Some(&"a"));
        assert_eq!(grid.get((2, 2)), Some(&"i"));

        let grid = Grid::from_bytes_transpose(
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
        let src = "
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
..........";

        let north = "
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
..........";

        let south = "
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
.OO.O....O";

        let east = "
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
..........";

        let west = "
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
..........";

        let grid = Grid::from_chars(src.trim()).unwrap();

        let map = [
            (Dir::North, north),
            (Dir::South, south),
            (Dir::East, east),
            (Dir::West, west),
        ];

        for (dir, expected) in map {
            assert_eq!(
                grid.clone().applied_gravity(dir, &'.', &['#']),
                Grid::from_chars(expected.trim()).unwrap()
            );
        }
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

        assert_equal(
            grid.with_offsets((1, 1), Vec2::SURROUNDING).all(),
            enumerate_format![
                (0, 0, Vec2::new(-1, -1), 'a'),
                (1, 0, Vec2::new(0, -1), 'b'),
                (2, 0, Vec2::new(1, -1), 'c'),
                (0, 1, Vec2::new(-1, 0), 'd'),
                (2, 1, Vec2::new(1, 0), 'f'),
                (0, 2, Vec2::new(-1, 1), 'g'),
                (1, 2, Vec2::new(0, 1), 'h'),
                (2, 2, Vec2::new(1, 1), 'i')
            ],
        );
        assert_equal(
            grid.with_offsets((0, 0), Vec2::SURROUNDING).all(),
            enumerate_format![
                (1, 0, Vec2::new(1, 0), 'b'),
                (0, 1, Vec2::new(0, 1), 'd'),
                (1, 1, Vec2::new(1, 1), 'e')
            ],
        );
        assert_equal(
            grid.with_offsets((2, 2), Vec2::SURROUNDING).all(),
            enumerate_format![
                (1, 1, Vec2::new(-1, -1), 'e'),
                (2, 1, Vec2::new(0, -1), 'f'),
                (1, 2, Vec2::new(-1, 0), 'h')
            ],
        );
    }

    #[test]
    fn surrounding_ortho() {
        let grid = Grid::from_double_iter_transpose(["abc".chars(), "def".chars(), "ghi".chars()])
            .unwrap();

        assert_equal(
            grid.with_offsets((1, 1), Dir::ORTHO_SNAKE).all(),
            enumerate_format![
                (1, 0, Dir::North, 'b'),
                (0, 1, Dir::West, 'd'),
                (2, 1, Dir::East, 'f'),
                (1, 2, Dir::South, 'h')
            ],
        );
        assert_equal(
            grid.with_offsets((0, 0), Dir::ORTHO_SNAKE).all(),
            enumerate_format![(1, 0, Dir::East, 'b'), (0, 1, Dir::South, 'd')],
        );
        assert_equal(
            grid.with_offsets((2, 2), Dir::ORTHO_SNAKE).all(),
            enumerate_format![(2, 1, Dir::North, 'f'), (1, 2, Dir::West, 'h')],
        );
    }

    #[test]
    fn surrounding_corners() {
        let grid = Grid::from_double_iter_transpose(["abc".chars(), "def".chars(), "ghi".chars()])
            .unwrap();

        assert_equal(
            grid.with_offsets((1, 1), Vec2::CORNERS).all(),
            enumerate_format![
                (0, 0, Vec2::new(-1, -1), 'a'),
                (2, 0, Vec2::new(1, -1), 'c'),
                (0, 2, Vec2::new(-1, 1), 'g'),
                (2, 2, Vec2::new(1, 1), 'i')
            ],
        );
        assert_equal(
            grid.with_offsets((0, 0), Vec2::CORNERS).all(),
            enumerate_format![(1, 1, Vec2::new(1, 1), 'e')],
        );
        assert_equal(
            grid.with_offsets((2, 2), Vec2::CORNERS).all(),
            enumerate_format![(1, 1, Vec2::new(-1, -1), 'e')],
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
