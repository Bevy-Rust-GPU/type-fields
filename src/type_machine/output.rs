use core::{
    marker::PhantomData,
    ops::{Shl, Shr},
};

use crate::t_funk::{Pointed, Tagged};

use super::{
    action::Action,
    input_mode::InputMode,
    instruction::Instruction,
    output_mode::{OutputMode, OutputPushBack, OutputSet},
};

/// An output in a do block
pub struct Output<T>(PhantomData<T>);

impl<T> Default for Output<T> {
    fn default() -> Self {
        Output(PhantomData)
    }
}

impl<T, Rhs> Shl<Rhs> for Output<T>
where
    Rhs: Instruction,
{
    type Output = Tagged<Output<T>, Action<Rhs>>;

    fn shl(self, rhs: Rhs) -> Self::Output {
        Tagged::point(Action(rhs))
    }
}

impl<P, T, Rhs> Shr<Output<Rhs>> for Tagged<P, T> {
    type Output = Tagged<Output<Rhs>, Self>;

    fn shr(self, _: Output<Rhs>) -> Self::Output {
        Tagged::point(self)
    }
}

impl<P, T, Rhs> Shl<Output<Rhs>> for Tagged<P, T> {
    type Output = Tagged<Output<Rhs>, Self>;

    fn shl(self, _: Output<Rhs>) -> Self::Output {
        Tagged::point(self)
    }
}

pub trait OutputOf: Sized {
    fn output() -> Output<Self> {
        Default::default()
    }
}

impl<T> OutputOf for T {}

pub trait SetOf: Sized {
    fn set() -> Output<OutputSet<Self>> {
        Default::default()
    }
}

impl<T> SetOf for T {}

pub trait DefOf: Sized {
    fn def() -> Output<OutputPushBack<Self>> {
        Default::default()
    }
}

impl<T> DefOf for T {}

impl<M, T, C, I, P> InputMode<C, I, P> for Tagged<Output<M>, T>
where
    T: InputMode<C, I, P>,
{
    fn fetch(context: C) -> I {
        T::fetch(context)
    }
}

impl<M, T, C, O, P> OutputMode<C, O, P> for Tagged<Output<M>, T>
where
    M: OutputMode<C, O, P>,
{
    type Output = M::Output;

    fn apply(context: C, output: O) -> Self::Output {
        M::apply(context, output)
    }
}
