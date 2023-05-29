mod inst;

pub use inst::*;

use crate::macros::functions;

/// A [`Function`] that closes over some external scope via `self` parameter.
#[functions]
pub trait Closure<Input>: Sized {
    type Output;

    fn call(self, input: Input) -> Self::Output;
}

pub type OutputT<T, U> = <T as Closure<U>>::Output;

/// Blanket impl for value-level functions.
///
/// API-friendly simplification for non-generic functions.
impl<F, I, O> Closure<I> for F
where
    F: FnOnce(I) -> O,
{
    type Output = O;

    fn call(self, input: I) -> Self::Output {
        self(input)
    }
}

impl Closure<()> for () {
    type Output = ();

    fn call(self, (): ()) -> Self::Output {
        ()
    }
}
