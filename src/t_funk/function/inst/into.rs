use core::marker::PhantomData;

use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};

#[derive(Closure, Category, Arrow)]
pub struct IntoF<T>(PhantomData<T>);

impl<T> Default for IntoF<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for IntoF<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for IntoF<T> {}

impl<T, U> Function<U> for IntoF<T>
where
    U: Into<T>,
{
    type Output = T;

    fn call(input: U) -> Self::Output {
        input.into()
    }
}

