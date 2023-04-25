use crate::macros::{
    applicative::{Apply, Pure},
    foldable::{FoldMap, Foldl, Foldr},
    functor::{Fmap, Replace},
    monad::{Chain, Then},
    monoid::{Mconcat, Mempty},
    semigroup::Mappend,
    Copointed, Pointed,
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
pub struct Left<T>(pub T);

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
pub struct Right<T>(pub T);

pub trait Either<T>:
    Pointed<Pointed = T> + Copointed<Copointed = T> + Functor + Applicative + Monad
{
}

impl<T> Either<T> for Left<T> {}
impl<T> Either<T> for Right<T> {}

#[cfg(test)]
mod test {
    use crate::t_funk::{test_functor_laws, Add, Curry, Fmap, Mul};

    use super::{Left, Right};

    #[test]
    fn test_either() {
        let left = Left(2);
        let mapped = left.fmap(Add.prefix(1));
        assert_eq!(mapped, Left(3));

        let left = Right(2);
        let mapped = left.fmap(Add.prefix(1));
        assert_eq!(mapped, Right(3));
    }

    #[test]
    fn test_functor_laws_maybe() {
        test_functor_laws(Left(1), Add.prefix(2), Mul.prefix(2));
        test_functor_laws(Right(1), Add.prefix(2), Mul.prefix(2));
    }
}
