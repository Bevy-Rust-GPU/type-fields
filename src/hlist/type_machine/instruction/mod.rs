mod constant;
mod no_op;

pub use constant::*;
pub use no_op::*;

use crate::functional::{Copointed, Phantom};

/// A single operation that takes input and produces output
pub trait Instruction {
    type Input<'a>
    where
        Self: 'a;

    type Output;

    fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output;
}

impl<P, T> Instruction for Phantom<P, T>
where
    Self: Copointed<Copointed = T>,
    for<'a> P: 'a,
    T: Instruction,
{
    type Input<'a> = T::Input<'a>
    where
        Self: 'a;

    type Output = T::Output;

    fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
        let this = self.unwrap();
        this.exec(input)
    }
}
