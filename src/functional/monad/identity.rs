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
            test_functor_laws, Add, Applicative, Closure, Composed, Const, Copointed, Curry,
            CurryN, Div, Flip, Fmap, Foldable, Functor, Identity, Monad, Mul, Point, Pointed, Sub,
            Sum, Then,
        },
        hlist::tuple::Cons,
    };

    #[test]
    fn test_identity() {
        let id1 = Identity::point(5);
        assert_eq!(id1.copoint(), 5);

        let id2: Identity<i32> = id1.fmap(Mul.curry_n().call(3));
        assert_eq!(id2.copoint(), 15);

        let id3: Identity<i32> =
            Identity::point(Fmap.flip().curry_n().call(Sub.flip().curry_n().call(3))).apply(id2);
        assert_eq!(id3.copoint(), 12);

        let id4: Identity<i32> = id3.chain(
            Composed::point((Div.flip(), Point::<Identity<_>>::default()))
                .curry_n()
                .call(3),
        );
        assert_eq!(id4.copoint(), 4);

        let id5: Identity<i32> = id4.then(Const.curry_n().call(Identity::point(1234)));
        assert_eq!(id5.copoint(), 1234);

        let id6: Sum<i32> = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
            .cons()
            .fold_map(Point::<Sum<i32>>::default());
        assert_eq!(id6.copoint(), 55);
    }

    #[test]
    fn test_functor_laws_identity() {
        test_functor_laws(Identity::point(4), Add.curry_a(2), Mul.curry_a(2))
    }
}
