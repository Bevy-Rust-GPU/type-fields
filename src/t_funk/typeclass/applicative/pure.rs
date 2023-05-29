use core::marker::PhantomData;

use crate::macros::Closure;

use crate::t_funk::Function;

pub trait Pure {
    type Pure<T>;
    fn pure<T>(t: T) -> Self::Pure<T>;
}

#[derive(Closure)]
pub struct PureF<T>(PhantomData<T>);

impl<T> Default for PureF<T> {
    fn default() -> Self {
        PureF(PhantomData)
    }
}

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
