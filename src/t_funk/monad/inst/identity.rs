use crate::macros::{
    applicative::Applicative, foldable::Foldable, functor::Functor, monad::Monad, monoid::Monoid,
    semigroup::Semigroup, Copointed, Pointed,
};

/// Identity monad, used to lift values into a monadic context.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Functor,
    Applicative,
    Monad,
    Semigroup,
    Monoid,
    Foldable,
)]
pub struct Identity<T>(pub T);

#[cfg(test)]
mod test {
    use crate::t_funk::{
        closure::Compose, monad::Chain, test_functor_laws, tlist::ToHList, Add, Apply, Closure,
        Curry2, Curry2B, CurryN, Div, Flip, Fmap, FoldMap, Identity, Mul, PointF, Sub, Sum, Then,
    };

    #[test]
    fn test_identity() {
        let id1 = Identity(5);
        assert_eq!(id1, Identity(5));

        let id2: Identity<i32> = id1.fmap(Mul.curry_n().call(3));
        assert_eq!(id2, Identity(15));

        let id3: Identity<Curry2B<Sub, i32>> = Identity(Sub.suffix2(3));
        let id3: Identity<i32> = id3.apply(id2);
        assert_eq!(id3, Identity(12));

        let id4: Identity<i32> = id3.chain(
            PointF::<Identity<_>>::default()
                .compose(Div.flip())
                .prefix2(3),
        );
        assert_eq!(id4, Identity(4));

        let id5: Identity<i32> = id4.then(Identity(1234));
        assert_eq!(id5, Identity(1234));

        let id6: Sum<i32> = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
            .to_hlist()
            .fold_map(PointF::<Sum<i32>>::default());
        assert_eq!(id6, Sum(55));
    }

    #[test]
    fn test_functor_laws_identity() {
        test_functor_laws(Identity(4), Add.prefix2(2), Mul.prefix2(2))
    }
}
