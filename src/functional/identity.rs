use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_monoid,
    derive_pointed, derive_semigroup,
};

/// Identity monad, used to lift values into a monadic context.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identity<T>(T);

derive_pointed!(Identity<T>);
derive_copointed!(Identity<T>);
derive_functor!(Identity<T>);
derive_applicative!(Identity<T>);
derive_monad!(Identity<T>);
derive_monoid!(Identity<T>);
derive_semigroup!(Identity<T>);

#[cfg(test)]
mod test {
    use crate::{
        functional::{
            Applicative, Composed, Const, Copointed, Curried, Div, Flipped, Fmap, Foldable,
            Function, Functor, Identity, Monad, Mul, Point, Pointed, Sub, Sum, Then,
        },
        hlist::tuple::Cons,
    };

    #[test]
    fn test_identity() {
        let id1 = Identity::point(5);
        assert_eq!(id1.copoint(), 5);

        let id2: Identity<i32> = id1.fmap(Curried::point(Mul).call(3));
        assert_eq!(id2.copoint(), 15);

        let id3: Identity<i32> = Identity::point(
            Curried::point(Flipped::point(Fmap)).call(Curried::point(Flipped::point(Sub)).call(3)),
        )
        .apply(id2);
        assert_eq!(id3.copoint(), 12);

        let id4: Identity<i32> = id3.chain(
            Curried::point(Composed::point((
                Flipped::point(Div),
                Point::<Identity<_>>::default(),
            )))
            .call(3),
        );
        assert_eq!(id4.copoint(), 4);

        let id5: Identity<i32> = id4.then(Curried::point(Const).call(Identity::point(1234)));
        assert_eq!(id5.copoint(), 1234);

        let id6: Sum<i32> = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
            .cons()
            .fold_map(Point::<Sum<i32>>::default());
        assert_eq!(id6.copoint(), 55);
    }
}
