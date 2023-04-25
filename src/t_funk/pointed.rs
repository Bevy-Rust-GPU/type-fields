use core::marker::PhantomData;

use crate::macros::{
    arrow::{Arr, First, Second},
    category::{Compose, Id},
    Closure,
};

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
#[derive(Closure, Id, Compose, Arr, First, Second)]
pub struct PointF<I>(PhantomData<I>);

impl<T> Default for PointF<T> {
    fn default() -> Self {
        PointF(PhantomData)
    }
}

impl<T> Clone for PointF<T> {
    fn clone(&self) -> Self {
        PointF(PhantomData)
    }
}

impl<I> Function<I::Pointed> for PointF<I>
where
    I: Pointed,
{
    type Output = I;

    fn call(input: I::Pointed) -> I {
        I::point(input)
    }
}
