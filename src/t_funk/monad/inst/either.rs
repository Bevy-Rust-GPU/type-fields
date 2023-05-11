use crate::macros::{
    applicative::applicative, foldable::foldable, functor::functor, monad::monad, monoid::monoid,
    semigroup::semigroup, Copointed, Pointed,
};

#[functor]
#[applicative]
#[monad]
#[semigroup]
#[monoid]
#[foldable]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Left<T>(pub T);

#[functor]
#[applicative]
#[monad]
#[semigroup]
#[monoid]
#[foldable]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Right<T>(pub T);

#[cfg(test)]
mod test {
    use crate::t_funk::{test_functor_laws, Add, Curry2, Fmap, Mul};

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
