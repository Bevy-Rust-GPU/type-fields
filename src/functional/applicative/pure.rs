use core::marker::PhantomData;

use crate::{derive_closure, functional::Function};

pub trait Pure {
    type Pure<T>;
    fn pure<T>(t: T) -> Self::Pure<T>;
}

pub struct PureF<T>(PhantomData<T>);

impl<T> Default for PureF<T> {
    fn default() -> Self {
        PureF(PhantomData)
    }
}

derive_closure!(PureF<T>);

impl<T> Clone for PureF<T> {
    fn clone(&self) -> Self {
        PureF(PhantomData)
    }
}

impl<T, U> Function<U> for PureF<T>
where
    T: Pure,
{
    type Output = T::Pure<U>;

    fn call(input: U) -> Self::Output {
        T::pure(input)
    }
}

