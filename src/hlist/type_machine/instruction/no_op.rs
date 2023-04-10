use crate::hlist::type_machine::{output_mode::OutputMode, input_mode::InputMode};

use super::Instruction;

/// Unit instruction with no input or output
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoOp;

impl Instruction for NoOp {
    type Input<'a> = ();
    type Output = ();

    fn exec<'a>(self, _: Self::Input<'a>) -> Self::Output {}
}

impl<C> InputMode<C, (), ()> for NoOp {
    fn fetch(_: C) -> () {
        ()
    }
}

impl<C> OutputMode<C, (), ()> for NoOp {
    type Output = C;

    fn apply(context: C, _: ()) -> Self::Output {
        context
    }
}

