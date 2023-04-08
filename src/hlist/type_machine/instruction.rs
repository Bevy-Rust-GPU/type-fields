extern crate alloc;

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

