use crate::hlist::type_machine::{input_mode::InputNone, output_mode::OutputPopBack};

use super::Instruction;

/// Instruction for pushing a value to the back of the context
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PopBack;

impl Instruction for PopBack {
    type Input<'a> = ()
    where
        Self: 'a;

    type InputMode = InputNone;

    type Output = ();

    type OutputMode = OutputPopBack;

    fn exec<'a>(self, _: Self::Input<'a>) -> Self::Output {}
}
