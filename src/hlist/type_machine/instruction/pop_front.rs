use crate::hlist::type_machine::{input_mode::InputNone, output_mode::OutputPopFront};

use super::Instruction;

/// Instruction for pushing a value to the back of the context
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PopFront;

impl Instruction for PopFront {
    type Input<'a> = ()
    where
        Self: 'a;

    type InputMode = InputNone;

    type Output = ();

    type OutputMode = OutputPopFront;

    fn exec<'a>(self, _: Self::Input<'a>) -> Self::Output {}
}

