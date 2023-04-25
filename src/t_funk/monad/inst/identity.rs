use type_fields_macros::{
    Apply, Chain, Copointed, Fmap, Fold, FoldMap, Foldl, Foldr, Mappend, Mconcat, Mempty, Pointed,
    Pure, Replace, Then,
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
    Fold,
)]
pub struct Identity<T>(T);

#[cfg(test)]
mod test {
    use crate::t_funk::{
        closure::Compose, test_functor_laws, tlist::ToHList, Add, Apply, Chain, Closure, Copointed,
        CurriedA, Curry, CurryN, Div, Flip, Flipped, Fmap, FoldMap, Identity, Mul, PointF, Pointed,
        Sub, Sum, Then,
    };

    #[test]
    fn test_identity() {
        let id1 = Identity::point(5);
        assert_eq!(id1.copoint(), 5);

        let id2: Identity<i32> = id1.fmap(Mul.curry_n().call(3));
        assert_eq!(id2.copoint(), 15);

        let id3: Identity<CurriedA<Flipped<Sub>, i32>> =
            Identity::point(Sub.flip().curry().call(3));
        let id3: Identity<i32> = id3.apply(id2);
        assert_eq!(id3.copoint(), 12);

        let id4: Identity<i32> = id3.chain(
            PointF::<Identity<_>>::default()
                .compose(Div.flip())
                .curry_n()
                .call(3),
        );
        assert_eq!(id4.copoint(), 4);

        let id5: Identity<i32> = id4.then(Identity::point(1234));
        assert_eq!(id5.copoint(), 1234);

        let id6: Sum<i32> = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
            .to_hlist()
            .fold_map(PointF::<Sum<i32>>::default());
        assert_eq!(id6.copoint(), 55);
    }

    #[test]
    fn test_functor_laws_identity() {
        test_functor_laws(Identity::point(4), Add.curry_a(2), Mul.curry_a(2))
    }
}
