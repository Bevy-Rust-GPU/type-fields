mod add;
mod compose;
mod constant;
mod id;
mod mul;
mod curry;

use core::marker::PhantomData;

pub use add::*;
pub use curry::*;
pub use compose::*;
pub use constant::*;
pub use id::*;
pub use mul::*;

use super::{Copointed, Pointed};

/// Abstract function trait.
///
/// Allows many signatures to be implemented on a single type,
/// unlike Fn* whose present implementation encodes a 1:1 coupling.
///
/// This allowing mapping over heterogenous lists.
pub trait Function<Inputs> {
    type Output;

    fn call(self, input: Inputs) -> Self::Output;
}

pub struct FunctionFn<F, O>(F, PhantomData<O>);

impl<F, O> Clone for FunctionFn<F, O>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        FunctionFn(self.0.clone(), PhantomData)
    }
}

impl<F, O> Pointed for FunctionFn<F, O> {
    type Pointed = F;

    fn point(unit: Self::Pointed) -> Self {
        FunctionFn(unit, PhantomData)
    }
}

impl<F, O> Copointed for FunctionFn<F, O> {
    type Copointed = F;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

/// Blanket FnOnce convenience impl.
///
/// Still subject to 1:1 impl coupling,
/// so cannot be mapped over heterogeneous structures.
impl<F, I, O> Function<I> for FunctionFn<F, O>
where
    F: FnOnce(I) -> O,
{
    type Output = O;

    fn call(self, input: I) -> Self::Output {
        self.0(input)
    }
}
