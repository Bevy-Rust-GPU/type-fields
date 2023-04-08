use core::marker::PhantomData;

use crate::hlist::type_machine::{input_mode::InputNone, output_mode::OutputRemove};

use super::Instruction;

/// Instruction for pushing a value to the back of the context
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Remove<T>(PhantomData<T>);

impl<T> Instruction for Remove<T>
where
    for<'a> T: 'a,
{
    type Input<'a> = ()
    where
        Self: 'a;

    type InputMode = InputNone;

    type Output = ();

    type OutputMode = OutputRemove<T>;

    fn exec<'a>(self, _: Self::Input<'a>) -> Self::Output {
        ()
    }
}

