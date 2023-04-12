use core::marker::PhantomData;

use super::Function;

/// A type that can wrap a value
pub trait Pointed
where
    Self: Sized,
{
    /// The wrapped value
    type Pointed;

    /// Wrap `Pointed` into `Self`
    fn point(unit: Self::Pointed) -> Self;
}

/// Pointed::point
pub struct Point<I>(PhantomData<I>);

impl<T> Default for Point<T> {
    fn default() -> Self {
        Point(PhantomData)
    }
}

impl<T> Clone for Point<T> {
    fn clone(&self) -> Self {
        Point(PhantomData)
    }
}

impl<I> Function<I::Pointed> for Point<I>
where
    I: Pointed,
{
    type Output = I;

    fn call(self, input: I::Pointed) -> I {
        I::point(input)
    }
}
