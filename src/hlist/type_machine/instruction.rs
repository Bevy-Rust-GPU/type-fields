use crate::hlist::tuple::TupleList;

/// A single operation that takes input and produces output
pub trait Instruction {
    type Input<'a>: TupleList
    where
        Self: 'a;
    type Output: TupleList;

    fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output;
}

