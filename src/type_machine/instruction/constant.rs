use crate::t_funk::{Copointed, Pointed};

use super::Instruction;

/// Instruction for pushing a value to the back of the context
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Const<T>(pub T);

impl<T> Pointed for Const<T> {
    type Pointed = T;

    fn point(unit: Self::Pointed) -> Self {
        Const(unit)
    }
}

impl<T> Copointed for Const<T> {
    type Copointed = T;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<T> Instruction for Const<T>
where
    for<'a> T: 'a,
{
    type Input<'a> = ()
    where
        Self: 'a;

    type Output = T;

    fn exec<'a>(self, _: Self::Input<'a>) -> Self::Output {
        self.0
    }
}
