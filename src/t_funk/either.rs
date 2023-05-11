use core::ops::{Deref, DerefMut};

use crate::t_funk::Fmap;

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
    L: Fmap<F>,
    R: Fmap<F>,
{
    type Fmap = Either<L::Fmap, R::Fmap>;

    fn fmap(self, f: F) -> Self::Fmap {
        match self {
            Either::Left(l) => Either::Left(l.fmap(f)),
            Either::Right(r) => Either::Right(r.fmap(f)),
        }
    }
}

