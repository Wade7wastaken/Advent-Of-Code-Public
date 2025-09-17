use crate::tern;

pub trait Swap<A, B> {
    fn swap(self) -> (B, A);
}

impl<A, B> Swap<A, B> for (A, B) {
    /// Swaps a 2-tuple.
    fn swap(self) -> (B, A) {
        (self.1, self.0)
    }
}

pub trait SwapIf {
    #[must_use]
    fn swap_if(self, swap: bool) -> Self;
}

impl<T> SwapIf for (T, T) {
    /// Swaps a 2-tuple if a condition is true.
    fn swap_if(self, cond: bool) -> Self {
        tern!(cond, self.swap(), self)
    }
}
