use crate::hlist::type_machine::{input_mode::InputNone, output_mode::OutputPushFront};

use super::Instruction;

/// Instruction for pushing a value to the back of the context
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PushFront<T>(pub T);

impl<T> Instruction for PushFront<T>
where
    for<'a> T: 'a,
{
    type Input<'a> = ()
    where
        Self: 'a;

    type InputMode = InputNone;

    type Output = T;

    type OutputMode = OutputPushFront;

    fn exec<'a>(self, _: Self::Input<'a>) -> Self::Output {
        self.0
    }
}
