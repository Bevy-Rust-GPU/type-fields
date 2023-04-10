use core::ops::{Shl, Shr};

use crate::functional::{Copointed, Phantom, Pointed};

use super::{
    input::Input, input_mode::InputMode, instruction::Instruction, output_mode::OutputMode, output::Output,
};

/// An action in a do block
pub struct Action<T>(T);

impl<T> Pointed for Action<T> {
    type Pointed = T;

    fn of(unit: Self::Pointed) -> Self {
        Action(unit)
    }
}

impl<T> Copointed for Action<T> {
    type Copointed = T;

    fn unwrap(self) -> Self::Copointed {
        self.0
    }
}

impl<T> Instruction for Action<T>
where
    T: Instruction,
{
    type Input<'a> = T::Input<'a>
        where
            Self: 'a;

    type Output = T::Output;

    fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output {
        self.0.exec(input)
    }
}

impl<T, C> InputMode<C, (), ()> for Action<T> {
    fn fetch(_: C) -> () {
        ()
    }
}

impl<T, C> OutputMode<C, (), ()> for Action<T> {
    type Output = C;

    fn apply(context: C, _: ()) -> Self::Output {
        context
    }
}

impl<T, Rhs> Shr<Output<Rhs>> for Action<T> {
    type Output = Phantom<Output<Rhs>, Action<T>>;

    fn shr(self, _: Output<Rhs>) -> Self::Output {
        Pointed::of(self)
    }
}

impl<T, Rhs> Shl<Input<Rhs>> for Action<T> {
    type Output = Phantom<Input<Rhs>, Action<T>>;

    fn shl(self, _: Input<Rhs>) -> Self::Output {
        Pointed::of(self)
    }
}

pub trait ActionOf: Sized {
    fn action(self) -> Action<Self> {
        Action::of(self)
    }
}

impl<T> ActionOf for T {}
