use core::ops::{Deref, DerefMut};

use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::{closure::OutputT, Closure, Fmap, Function, Replace},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<T> Deref for Either<T, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => r,
        }
    }
}

impl<T> DerefMut for Either<T, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => r,
        }
    }
}

impl<L, R> Either<L, R> {
    pub fn try_left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub fn try_right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    pub fn left(self) -> L {
        self.try_left().expect("Called left() on a Right value")
    }

    pub fn right(self) -> R {
        self.try_right().expect("Called right() on a Left value")
    }
}

impl<T> Either<T, T> {
    pub fn unwrap(self) -> T {
        match self {
            Either::Left(t) | Either::Right(t) => t,
        }
    }
}

impl<L, R, F> Fmap<F> for Either<L, R>
where
    F: Closure<L> + Closure<R>,
{
    type Fmap = Either<OutputT<F, L>, OutputT<F, R>>;

    fn fmap(self, f: F) -> Self::Fmap {
        match self {
            Either::Left(l) => Either::Left(f.call(l)),
            Either::Right(r) => Either::Right(f.call(r)),
        }
    }
}

impl<L, R, T> Replace<T> for Either<L, R> {
    type Replace = Either<T, T>;

    fn replace(self, t: T) -> Self::Replace {
        match self {
            Either::Left(_) => Either::Left(t),
            Either::Right(_) => Either::Right(t),
        }
    }
}

impl<L, R, I> Closure<I> for Either<L, R>
where
    L: Closure<I>,
    R: Closure<I, Output = OutputT<L, I>>,
{
    type Output = OutputT<L, I>;

    fn call(self, i: I) -> Self::Output {
        match self {
            Either::Left(f) => f.call(i),
            Either::Right(f) => f.call(i),
        }
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct EitherUnwrap;

impl<T> Function<Either<T, T>> for EitherUnwrap {
    type Output = T;

    fn call(input: Either<T, T>) -> Self::Output {
        input.unwrap()
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct EitherLeft;

impl<L, R> Function<Either<L, R>> for EitherLeft {
    type Output = L;

    fn call(input: Either<L, R>) -> Self::Output {
        input.left()
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct EitherRight;

impl<L, R> Function<Either<L, R>> for EitherRight {
    type Output = R;

    fn call(input: Either<L, R>) -> Self::Output {
        input.right()
    }
}
