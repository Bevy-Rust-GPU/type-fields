use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_monoid,
    derive_pointed, derive_semigroup,
    functional::{Pointed, Pure, Semigroup},
};

/// Identity monad, used to lift values into a monadic context.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identity<T>(T);

impl<T> Pure for Identity<T> {
    type Pure<U> = Identity<U>;

    fn pure<U>(t: U) -> Self::Pure<U> {
        Identity::point(t)
    }
}

derive_pointed!(Identity<T>);
derive_copointed!(Identity<T>);
derive_functor!(Identity<T>);
derive_applicative!(Identity<T>);
derive_monad!(Identity<T>);
derive_monoid!(Identity<T>);
derive_semigroup!(Identity<T>);

impl<T> Semigroup<()> for Identity<T> {
    type Appended = Identity<T>;

    fn mappend(self, _: ()) -> Self::Appended {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{
        functional::{
            test_functor_laws, Add, Applicative, Closure, Composed, Copointed, CurriedA, Curry,
            CurryN, Div, Flip, Flipped, Foldable, Functor, Identity, Monad, Mul, Point, Pointed,
            Sub, Sum, Then,
        },
        hlist::tuple::Cons,
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
            Composed::point((Div.flip(), Point::<Identity<_>>::default()))
                .curry_n()
                .call(3),
        );
        assert_eq!(id4.copoint(), 4);

        let id5: Identity<i32> = id4.then(Identity::point(1234));
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
