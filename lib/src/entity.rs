use std::{
    fmt::{self, Display},
    hash::Hash,
};

use num::{CheckedAdd, CheckedSub, Num};

use crate::{Dir, Grid, Offset, Point2, Vec2};

/// [lower, upper), None is assumed to be unbounded
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds<T: Num + Copy>(Point2<T>, Point2<T>);

impl<T: Num + Copy + Display> Display for Bounds<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.0, self.1)
    }
}

/// Represents an object with both a position and a direction. Has optional
/// bounds that restrict its positional movement.
#[derive(Debug, Clone, Copy)]
pub struct Entity<T: Num + Copy, D: Offset = Dir> {
    pos: Point2<T>,
    dir: D,
    bounds: Option<Bounds<T>>,
}

impl<T: Num + Copy, D: Offset> Entity<T, D> {
    /// Gets the entity's position.
    pub const fn pos(self) -> Point2<T> {
        self.pos
    }

    /// Gets the entity's direction.
    pub const fn dir(self) -> D {
        self.dir
    }

    /// Gets the entity's position and direction.
    pub const fn tuple(self) -> (Point2<T>, D) {
        (self.pos, self.dir)
    }

    /// Gets the entity's bounds if they exist.
    pub const fn bounds(self) -> Option<Bounds<T>> {
        self.bounds
    }

    /// Provides the ability to change the bounds after the entity was created.
    /// Unsafe because invalid bounds might cause an indexing panic when used
    /// with grid.
    ///
    /// # Safety
    /// Caller is responsible for making sure the new bounds are safe in the
    /// given situation
    pub const unsafe fn update_bounds(&mut self, new_bounds: Option<Bounds<T>>) {
        self.bounds = new_bounds;
    }

    /// Returns whether the entity has bounds.
    pub const fn has_bounds(self) -> bool {
        self.bounds.is_some()
    }

    /// Checks if the entity's current position is within the entity's bounds.
    fn is_bounded(&self) -> bool
    where
        T: PartialOrd,
    {
        self.bounds
            .is_some_and(|bounds| self.pos.within(bounds.0, bounds.1))
    }

    /// Creates a new entity with a position and direction.
    pub fn new(pos: impl Into<Point2<T>>, dir: D) -> Self {
        Self {
            pos: pos.into(),
            dir,
            bounds: None,
        }
    }

    /// Creates a new entity with bounds. If the position is out of bounds, it
    /// returns None.
    pub fn new_bounded(
        pos: impl Into<Point2<T>>,
        dir: D,
        lower: impl Into<Point2<T>>,
        upper: impl Into<Point2<T>>,
    ) -> Option<Self>
    where
        T: PartialOrd,
    {
        let lower = lower.into();
        let upper = upper.into();
        let pos = pos.into();

        pos.within(lower, upper).then_some(Self {
            pos,
            dir,
            bounds: Some(Bounds(lower, upper)),
        })
    }

    /// Creates a new entity with bounds from a grid. Returns None if the given
    /// point is not bounded by the grid.
    pub fn new_on_grid<C: Clone>(pos: impl Into<Point2<T>>, dir: D, grid: &Grid<C>) -> Option<Self>
    where
        T: From<usize> + PartialOrd,
    {
        Self::new_bounded(
            pos,
            dir,
            (T::zero(), T::zero()),
            (grid.width().into(), grid.height().into()),
        )
    }

    fn set_internal(self, pos: impl Into<Point2<T>>, dir: D) -> Self {
        Self {
            pos: pos.into(),
            dir,
            ..self
        }
    }

    /// Returns a new entity with the new position and direction. This is useful
    /// when you need to create a new entity with the same bounds. This function
    /// doesn't check the bounds and will panic if the entity has bounds as a
    /// failsafe.
    #[must_use]
    pub fn set(self, pos: impl Into<Point2<T>>, dir: D) -> Self {
        assert!(
            self.bounds.is_none(),
            "Called set with an entity that has bounds. Use set_bounded instead."
        );
        self.set_internal(pos, dir)
    }

    /// Returns a new entity with the new position and direction. This is useful
    /// when you need to create a new entity with the same bounds. If the new
    /// position would cause the entity to go out of bounds, None is returned
    pub fn set_bounded(self, pos: impl Into<Point2<T>>, dir: D) -> Option<Self>
    where
        T: PartialOrd,
    {
        Some(self.set_internal(pos, dir)).filter(Self::is_bounded)
    }

    fn set_pos_internal(self, pos: impl Into<Point2<T>>) -> Self {
        Self {
            pos: pos.into(),
            ..self
        }
    }

    /// Returns a new entity with a new position and the same dir. This function
    /// doesn't check the bounds and will panic if the entity has bounds as a
    /// failsafe.
    #[must_use]
    pub fn set_pos(self, pos: impl Into<Point2<T>>) -> Self {
        assert!(
            self.bounds.is_none(),
            "Called set_pos with an entity that has bounds. Use set_pos_bounded instead."
        );
        self.set_pos_internal(pos)
    }

    /// Returns a new entity with a new position and the same dir. If the new
    /// position would cause the entity to go out of bounds, None is returned
    pub fn set_pos_bounded(self, pos: impl Into<Point2<T>>) -> Option<Self>
    where
        T: PartialOrd,
    {
        Some(self.set_pos_internal(pos)).filter(Self::is_bounded)
    }

    /// Returns a new entity with a new direction
    #[must_use]
    pub const fn set_dir(self, dir: D) -> Self {
        Self { dir, ..self }
    }

    /// Turns the entity left
    #[must_use]
    pub fn turn_left(self) -> Self {
        self.set_dir(self.dir.turn_left())
    }

    /// Turns the entity right
    #[must_use]
    pub fn turn_right(self) -> Self {
        self.set_dir(self.dir.turn_right())
    }

    /// Reverses the entity
    #[must_use]
    pub fn reverse(self) -> Self {
        self.set_dir(self.dir.reverse())
    }

    fn step_internal(self) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + TryFrom<isize>,
    {
        self.pos
            .apply(self.dir)
            .map(|pos| self.set_pos_internal(pos))
    }

    /// Moves the entity's position by its direction. This function doesn't
    /// check the bounds and will panic if the entity has bounds as a failsafe.
    pub fn step(self) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + TryFrom<isize>,
    {
        assert!(
            self.bounds.is_none(),
            "Called step with an entity that has bounds. Use set_pos_bounded instead."
        );
        
        self.step_internal()
    }

    /// Moves the entity's position by its direction. If the new position would
    /// cause the entity to go out of bounds, None is returned
    pub fn step_bounded(self) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + PartialOrd + TryFrom<isize>,
    {
        self.step_internal().filter(Self::is_bounded)
    }

    fn step_n_internal(self, n: isize) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + TryFrom<isize>,
    {
        let a: Vec2 = self.dir.into();
        self.pos.apply(a * n).map(|pos| self.set_pos_internal(pos))
    }

    /// Moves the entity's position by its direction n steps. This function doesn't
    /// check the bounds and will panic if the entity has bounds as a failsafe.
    pub fn step_n(self, n: isize) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + TryFrom<isize>,
    {
        assert!(
            self.bounds.is_none(),
            "Called step with an entity that has bounds. Use set_pos_bounded instead."
        );
        self.step_n_internal(n)
    }

    /// Moves the entity's position by its direction n steps. If the new position would
    /// cause the entity to go out of bounds, None is returned
    pub fn step_n_bounded(self, n: isize) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + PartialOrd + TryFrom<isize>,
    {
        self.step_n_internal(n).filter(Self::is_bounded)
    }

    fn slide_internal(self, dir: Dir) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + TryFrom<isize>,
    {
        self.pos.apply(dir).map(|pos| self.set_pos_internal(pos))
    }

    /// Applies a direction to the entity without updating the entity's actual
    /// direction. This function doesn't check the bounds and will panic if the
    /// entity has bounds as a failsafe.
    pub fn slide(self, dir: Dir) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + TryFrom<isize>,
    {
        assert!(
            self.bounds.is_none(),
            "Called slide with an entity that has bounds. Use slide_bounded instead."
        );
        self.slide_internal(dir)
    }

    /// Applies a direction to the entity without updating the entity's actual
    /// direction. If the new position would cause the entity to go out of
    /// bounds, None is returned
    pub fn slide_bounded(self, dir: Dir) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + PartialOrd + TryFrom<isize>,
    {
        self.slide_internal(dir).filter(Self::is_bounded)
    }
}

impl<T: Num + Copy, D: Offset> PartialEq for Entity<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.dir
    }
}

impl<T: Num + Copy, D: Offset> Eq for Entity<T, D> {}

impl<T: Num + Copy + Hash, D: Offset> Hash for Entity<T, D> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.dir.hash(state);
    }
}

impl<T: Num + Copy + Display, D: Offset> Display for Entity<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.bounds {
            Some(bounds) => write!(
                f,
                "At {} heading {} with bounds {}",
                self.pos, self.dir, bounds
            ),
            None => write!(f, "At {} heading {}", self.pos, self.dir),
        }
    }
}

impl<T: Num + Copy, D: Offset> From<Entity<T, D>> for Point2<T> {
    fn from(value: Entity<T, D>) -> Self {
        value.pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            Entity::new((1, 2), Dir::East).to_string(),
            "At (1, 2) heading East"
        );
    }

    #[test]
    fn getters() {
        let en = Entity::new_bounded((1, 2), Dir::North, (0, 0), (5, 5)).unwrap();
        assert_eq!(en.pos(), (1, 2).into());
        assert_eq!(en.dir(), Dir::North);
        assert_eq!(en.tuple(), ((1, 2).into(), Dir::North));
        // unsafe {en.update_bounds(Some(((-1, -1).into(), (5, 5).into())));}
    }

    #[test]
    fn new() {
        let en = Entity::new((5, 6), Dir::East);
        assert_eq!(en.pos(), (5, 6).into());
        assert_eq!(en.dir(), Dir::East);
        assert_eq!(en.tuple(), ((5, 6).into(), Dir::East));
        assert_eq!(en.bounds(), None);

        let en = Entity::new_bounded((1, 2), Dir::North, (0, 0), (5, 5)).unwrap();
        assert_eq!(en.pos(), (1, 2).into());
        assert_eq!(en.dir(), Dir::North);
        assert_eq!(en.bounds(), Some(Bounds((0, 0).into(), (5, 5).into())));

        let en = Entity::new_bounded((-1, 0), Dir::North, (0, 0), (5, 5));
        assert_eq!(en, None);
        let en = Entity::new_bounded((0, -1), Dir::North, (0, 0), (5, 5));
        assert_eq!(en, None);
        let en = Entity::new_bounded((5, 0), Dir::North, (0, 0), (5, 5));
        assert_eq!(en, None);
        let en = Entity::new_bounded((0, 5), Dir::North, (0, 0), (5, 5));
        assert_eq!(en, None);

        let grid = Grid::from_chars_transpose("abc").unwrap();
        let en = Entity::new_on_grid((0, 0), Dir::North, &grid).unwrap();
        assert_eq!(
            en.bounds,
            Some(Bounds(Point2::new(0, 0), Point2::new(3, 1)))
        );
        assert_eq!(en.set_pos_bounded((3, 1)), None);
        let en = Entity::new_on_grid((3, 1), Dir::North, &grid);
        assert_eq!(en, None);
    }

    #[test]
    fn set() {
        let en = Entity::new((4, 4), Dir::East);
        assert_eq!(en.set_dir(Dir::West), Entity::new((4, 4), Dir::West));
        assert_eq!(en.set_pos((5, 5)), Entity::new((5, 5), Dir::East));
        assert_eq!(en.set((5, 5), Dir::West), Entity::new((5, 5), Dir::West));
    }

    #[test]
    fn set_bounded() {
        let en = Entity::new_bounded((4, 4), Dir::East, (0, 0), (10, 10)).unwrap();
        assert_eq!(
            en.set_pos_bounded((5, 5)).map(Entity::tuple),
            Some(((5, 5).into(), Dir::East))
        );
        assert_eq!(
            en.set_bounded((5, 5), Dir::West).map(Entity::tuple),
            Some(((5, 5).into(), Dir::West))
        );

        assert_eq!(en.set_pos_bounded((-1, -1)), None);
        assert_eq!(en.set_pos_bounded((10, 10)), None);

        assert_eq!(en.set_bounded((-1, -1), Dir::West), None);
        assert_eq!(en.set_bounded((10, 10), Dir::West), None);
    }

    #[test]
    fn turn() {
        let en = Entity::new((0, 0), Dir::East);
        assert_eq!(en.turn_left(), Entity::new((0, 0), Dir::North));
        assert_eq!(en.turn_right(), Entity::new((0, 0), Dir::South));
        assert_eq!(en.reverse(), Entity::new((0, 0), Dir::West));
    }

    #[test]
    fn step() {
        let en = Entity::new((4, 0), Dir::East);
        assert_eq!(en.step(), Some(Entity::new((5, 0), Dir::East)));
        let en = Entity::new((4, 0), Dir::West);
        assert_eq!(en.step(), Some(Entity::new((3, 0), Dir::West)));
        let en = Entity::new((4, 0), Dir::South);
        assert_eq!(en.step(), Some(Entity::new((4, 1), Dir::South)));
        let en = Entity::new((4u32, 0u32), Dir::North);
        assert_eq!(en.step(), None);
    }

    #[test]
    fn step_bounded() {
        let en = Entity::new_bounded((4, 0), Dir::East, (0, 0), (6, 6)).unwrap();
        assert_eq!(
            en.step_bounded().map(Entity::tuple),
            Some(((5, 0).into(), Dir::East))
        );
        let en = Entity::new_bounded((4, 0), Dir::West, (0, 0), (6, 6)).unwrap();
        assert_eq!(
            en.step_bounded().map(Entity::tuple),
            Some(((3, 0).into(), Dir::West))
        );

        let en = Entity::new_bounded((4, 0), Dir::North, (0, 0), (6, 6)).unwrap();
        assert_eq!(en.step_bounded(), None);
        let en = Entity::new_bounded((5, 5), Dir::South, (0, 0), (6, 6)).unwrap();
        assert_eq!(en.step_bounded(), None);
    }

    #[test]
    fn slide() {
        let en = Entity::new((4u32, 0u32), Dir::North);
        assert_eq!(en.slide(Dir::East), Some(Entity::new((5, 0), Dir::North)));
        assert_eq!(en.slide(Dir::West), Some(Entity::new((3, 0), Dir::North)));
        assert_eq!(en.slide(Dir::South), Some(Entity::new((4, 1), Dir::North)));
        assert_eq!(en.slide(Dir::North), None);
    }

    #[test]
    fn slide_bounded() {
        let en = Entity::new_bounded((4, 0), Dir::North, (0, 0), (6, 6)).unwrap();
        assert_eq!(
            en.slide_bounded(Dir::East).map(Entity::tuple),
            Some(((5, 0).into(), Dir::North))
        );
        // let en = Entity::new_bounded((4, 0), Dir::WEST, (0, 0), (6, 6)).unwrap();
        assert_eq!(
            en.slide_bounded(Dir::West).map(Entity::tuple),
            Some(((3, 0).into(), Dir::North))
        );

        // let en = Entity::new_bounded((4, 0), Dir::NORTH, (0, 0), (6, 6)).unwrap();
        assert_eq!(en.slide_bounded(Dir::North), None);
        let en = Entity::new_bounded((5, 5), Dir::East, (0, 0), (6, 6)).unwrap();
        assert_eq!(en.slide_bounded(Dir::South), None);
    }
}
