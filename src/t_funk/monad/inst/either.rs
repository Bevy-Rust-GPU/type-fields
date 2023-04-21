use type_fields_macros::{
    Apply, Chain, Copointed, Fmap, FoldMap, Foldl, Foldr, Mappend, Mconcat, Mempty, Pointed, Pure,
    Replace, Then,
};

use crate::t_funk::{Applicative, Copointed, Functor, Monad, Pointed};

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Fmap,
    Replace,
    Pure,
    Apply,
    Chain,
    Then,
    Mempty,
    Mappend,
    Mconcat,
    FoldMap,
    Foldr,
    Foldl,
)]
pub struct Left<T>(T);

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Fmap,
    Replace,
    Pure,
    Apply,
    Chain,
    Then,
    Mempty,
    Mappend,
    Mconcat,
    FoldMap,
    Foldr,
    Foldl,
)]
pub struct Right<T>(T);

pub trait Either<T>:
    Pointed<Pointed = T> + Copointed<Copointed = T> + Functor + Applicative + Monad
{
}

impl<T> Either<T> for Left<T> {}
impl<T> Either<T> for Right<T> {}

#[cfg(test)]
mod test {
    use crate::t_funk::{test_functor_laws, Add, Closure, Curry, CurryN, Fmap, Mul, Pointed};

    use super::{Left, Right};

    #[test]
    fn test_either() {
        let left = Left::point(2);
        let mapped = left.fmap(CurryN::<(i32, i32)>::curry_n(Add).call(1));
        assert_eq!(mapped, Left::point(3));

        let left = Right::point(2);
        let mapped = left.fmap(CurryN::<(i32, i32)>::curry_n(Add).call(1));
        assert_eq!(mapped, Right::point(3));
    }

    #[test]
    fn test_functor_laws_maybe() {
        test_functor_laws(Left::point(1), Add.curry_a(2), Mul.curry_a(2));
        test_functor_laws(Right::point(1), Add.curry_a(2), Mul.curry_a(2));
    }
}
