pub trait SliceTools<T> {
    fn at(&self, i: usize) -> &T;

    fn at_mut(&mut self, i: usize) -> &mut T;
}

impl<T> SliceTools<T> for [T] {
    /// Returns a reference to the element at index i modulo the slice's length.
    fn at(&self, i: usize) -> &T {
        let el = self.get(i % self.len());
        unsafe { el.unwrap_unchecked() }
    }

    /// Returns a mutable reference to the element at index i modulo the slice's
    /// length.
    fn at_mut(&mut self, i: usize) -> &mut T {
        let len = self.len();
        let el = self.get_mut(i % len);
        unsafe { el.unwrap_unchecked() }
    }
}
