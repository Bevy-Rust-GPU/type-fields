use crate::functional::{Copointed, Pointed};

use super::Function;

pub struct Const<T>(T);

impl<T> Pointed for Const<T> {
    type Pointed = T;

    fn point(unit: Self::Pointed) -> Self {
        Const(unit)
    }
}

impl<T> Copointed for Const<T> {
    type Copointed = T;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<T> Clone for Const<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Pointed::point(self.0.clone())
    }
}

impl<A, T> Function<A> for Const<T> {
    type Output = T;

    fn call(self, _: A) -> Self::Output {
        self.0
    }
}
