use core::{
    marker::PhantomData,
    ops::{Shl, Shr},
};

use crate::t_funk::{Copointed, Pointed, Tagged};

use super::{
    action::Action,
    input_mode::{InputGet, InputMode},
    output_mode::OutputMode,
};

/// An input in a do block
pub struct Input<T>(PhantomData<T>);

impl<T> Default for Input<T> {
    fn default() -> Self {
        Input(PhantomData)
    }
}

impl<T, Rhs> Shr<Action<Rhs>> for Input<T> {
    type Output = Tagged<Input<T>, Action<Rhs>>;

    fn shr(self, rhs: Action<Rhs>) -> Self::Output {
        Tagged::point(rhs)
    }
}

impl<T, Rhs> Shl<Input<Rhs>> for Input<T> {
    type Output = Tagged<Input<T>, Input<Rhs>>;

    fn shl(self, rhs: Input<Rhs>) -> Self::Output {
        Tagged::point(rhs)
    }
}

impl<T, Rhs> Shl<Action<Rhs>> for Input<T> {
    type Output = Tagged<Input<T>, Action<Rhs>>;

    fn shl(self, rhs: Action<Rhs>) -> Self::Output {
        Tagged::point(rhs)
    }
}

impl<P, T, Rhs> Shr<Input<Rhs>> for Tagged<P, T> {
    type Output = Tagged<Input<Rhs>, Self>;

    fn shr(self, _: Input<Rhs>) -> Self::Output {
        Tagged::point(self)
    }
}

impl<P, T, Rhs> Shl<Input<Rhs>> for Tagged<P, T> {
    type Output = Tagged<P, Tagged<Input<Rhs>, T>>;

    fn shl(self, _: Input<Rhs>) -> Self::Output {
        Tagged::point(Tagged::point(self.copoint()))
    }
}

pub trait InputOf: Sized {
    fn input() -> Input<Self> {
        Default::default()
    }
}

impl<T> InputOf for T {}

pub trait GetOf: Sized {
    fn get() -> Input<InputGet<Self>> {
        Default::default()
    }
}

impl<T> GetOf for T {}

impl<M, T, C, I, P> InputMode<C, I, P> for Tagged<Input<M>, T>
where
    M: InputMode<C, I, P>,
{
    fn fetch(context: C) -> I {
        M::fetch(context)
    }
}

impl<M, T, C, O, P> OutputMode<C, O, P> for Tagged<Input<M>, T>
where
    T: OutputMode<C, O, P>,
{
    type Output = T::Output;

    fn apply(context: C, output: O) -> Self::Output {
        T::apply(context, output)
    }
}
