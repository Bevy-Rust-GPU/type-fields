use core::marker::PhantomData;

use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};

#[derive(Closure, Category, Arrow)]
pub struct FromF<T>(PhantomData<T>);

impl<T> Default for FromF<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for FromF<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for FromF<T> {}

impl<T, U> Function<U> for FromF<T>
where
    T: From<U>,
{
    type Output = T;

    fn call(input: U) -> Self::Output {
        T::from(input)
    }
}

