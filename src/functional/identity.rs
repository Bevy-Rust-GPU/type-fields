use crate::{derive_pointed, derive_copointed, derive_functor, derive_applicative, derive_monad, derive_monoid, derive_semigroup};


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
            Applicative, Copointed, Foldable, FunctionFn, Functor, Identity, Monad, Point, Pointed,
            Sum, Then,
        },
        hlist::tuple::Cons,
    };

    #[test]
    fn test_identity() {
        let id1 = Identity::point(5);
        let id2: Identity<i32> = id1.fmap(FunctionFn::point(|x| x * 3));
        let id3: Identity<i32> = Identity::point(FunctionFn::point(|x: Identity<i32>| {
            x.fmap(FunctionFn::point(|y| y - 3))
        }))
        .apply(id2);
        let id4 = id3.chain(FunctionFn::point(|x| Identity::point(x / 3)));
        let id5 = id4.then(FunctionFn::point(|_| Identity::point(1234)));
        let id6 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
            .cons()
            .fold_map(Point::<Sum<i32>>::default());

        assert_eq!(id1.copoint(), 5);
        assert_eq!(id2.copoint(), 15);
        assert_eq!(id3.copoint(), 12);
        assert_eq!(id4.copoint(), 4);
        assert_eq!(id5.copoint(), 1234);
        assert_eq!(id6.copoint(), 55);
    }
}
