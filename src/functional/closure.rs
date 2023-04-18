use crate::{derive_closure, functional::Function};

/// A [`Function`] that closes over some external scope via `self` parameter.
pub trait Closure<Input>: Sized {
    type Output;

    fn call(self, input: Input) -> Self::Output;
}

/// Closure::call
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Call;

impl<F, I> Function<(F, I)> for Call
where
    F: Closure<I>,
{
    type Output = F::Output;

    fn call((f, i): (F, I)) -> Self::Output {
        f.call(i)
    }
}

derive_closure!(Call);

