use crate::{
    macros::{arrow::arrow, category::category, Closure},
    t_funk::{Closure, Either, Function},
};

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct MakeIf;

impl<T, U> Function<(T, U)> for MakeIf {
    type Output = If<T, U>;

    fn call((t, u): (T, U)) -> Self::Output {
        If(t, u)
    }
}

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct If<T, U>(pub T, pub U);

impl<T, U> Closure<bool> for If<T, U> {
    type Output = Either<T, U>;

    fn call(self, input: bool) -> Self::Output {
        if input {
            Either::Left(self.0)
        } else {
            Either::Right(self.1)
        }
    }
}
