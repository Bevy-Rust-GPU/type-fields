use crate::hlist::type_machine::{input_mode::InputNone, output_mode::OutputSets};

use super::Instruction;

/// Instruction for pushing a value to the back of the context
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sets<T>(pub T);

impl<T> Instruction for Sets<T>
where
    for<'a> T: 'a,
{
    type Input<'a> = ()
    where
        Self: 'a;

    type InputMode = InputNone;

    type Output = T;

    type OutputMode = OutputSets;

    fn exec<'a>(self, _: Self::Input<'a>) -> Self::Output {
        self.0
    }
}

