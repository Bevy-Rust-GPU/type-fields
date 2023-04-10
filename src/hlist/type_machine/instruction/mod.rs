mod pop_back;
mod pop_front;
mod push_back;
mod push_front;
mod remove;
mod set;
mod sets;

pub use pop_back::*;
pub use pop_front::*;
pub use push_back::*;
pub use push_front::*;
pub use remove::*;
pub use set::*;
pub use sets::*;

/// A single operation that takes input and produces output
pub trait Instruction {
    type Input<'a>
    where
        Self: 'a;
    type InputMode;

    type Output;
    type OutputMode;

    fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output;
}

pub trait InstructionInput {
    type InputMode;
}

pub trait InstructionOutput {
    type OutputMode;
}
