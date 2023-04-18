use core::marker::PhantomData;

use crate::functional::{
    Applicative, Closure, Copointed, Functor, Monad, Monoid, Pointed, Semigroup,
};

/// Phantom monad, used to lift values into a monadic context
/// alongside some additional type parameter to make them
/// distinct from other instances of the same type.
pub struct Tagged<M, T>(PhantomData<M>, T);

impl<P, T> core::fmt::Debug for Tagged<P, T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.1.fmt(f)
    }
}

impl<P, T> Clone for Tagged<P, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Tagged::<P, T>::point(self.1.clone())
    }
}

impl<P, T> Copy for Tagged<P, T> where T: Copy {}

impl<P, T> PartialEq for Tagged<P, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<P, T> Eq for Tagged<P, T> where T: Eq {}

impl<P, T> PartialOrd for Tagged<P, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<P, T> Ord for Tagged<P, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl<P, T> core::hash::Hash for Tagged<P, T>
where
    T: core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state)
    }
}

impl<P, T> Pointed for Tagged<P, T> {
    type Pointed = T;

    fn point(unit: Self::Pointed) -> Self {
        Tagged(PhantomData, unit)
    }
}

impl<P, T> Copointed for Tagged<P, T> {
    type Copointed = T;

    fn copoint(self) -> Self::Copointed {
        self.1
    }
}

impl<P, A, F> Functor<F> for Tagged<P, A>
where
    F: Closure<A>,
{
    type Mapped = Tagged<P, F::Output>;

    fn fmap(self, f: F) -> Self::Mapped {
        Tagged::point(f.call(self.copoint()))
    }
}

impl<P, T, U> Applicative<U> for Tagged<P, T>
where
    T: Closure<U>,
{
    type Applied = T::Output;

    fn apply(self, a: U) -> Self::Applied
    where
        T: Closure<U>,
    {
        self.copoint().call(a)
    }
}

impl<P, T, F> Monad<F> for Tagged<P, T>
where
    F: Closure<T>,
{
    type Chained = F::Output;

    fn chain(self, f: F) -> Self::Chained {
        f.call(self.copoint())
    }
}

impl<P, T> Monoid for Tagged<P, T>
where
    T: Monoid,
{
    type Identity = T::Identity;

    fn mempty() -> Self::Identity {
        T::mempty()
    }
}

impl<P, T, U> Semigroup<Tagged<P, U>> for Tagged<P, T>
where
    T: Semigroup<U>,
{
    type Appended = Tagged<P, T::Appended>;

    fn mappend(self, t: Tagged<P, U>) -> Self::Appended {
        Pointed::point(self.1.mappend(t.copoint()))
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{
        test_functor_laws, Add, Applicative, Closure, Composed, Copointed, Curry, CurryN, Div,
        Flip, Fmap, Functor, Monad, Mul, Point, Pointed, Sub, Tagged, Then,
    };

    #[test]
    fn test_tagged() {
        enum Tag {}

        let id1: Tagged<Tag, i32> = Tagged::<Tag, _>::point(5);
        assert_eq!(id1.copoint(), 5);

        let id2: Tagged<Tag, i32> = id1.fmap(Mul.curry_n().call(3));
        assert_eq!(id2.copoint(), 15);

        let id3: Tagged<Tag, i32> =
            Tagged::<Tag, _>::point(Fmap.flip().curry_n().call(Sub.flip().curry_n().call(3)))
                .apply(id2);
        assert_eq!(id3.copoint(), 12);

        let id4: Tagged<Tag, i32> = id3.chain(
            Composed::point((Div.flip(), Point::<Tagged<Tag, _>>::default()))
                .curry_n()
                .call(3),
        );
        assert_eq!(id4.copoint(), 4);

        let id5: Tagged<Tag, i32> = id4.then(Tagged::<Tag, _>::point(1234));
        assert_eq!(id5.copoint(), 1234);
    }

    #[test]
    fn test_functor_laws_tagged() {
        enum Tag {}
        test_functor_laws(Tagged::<Tag, _>::point(1), Add.curry_a(2), Mul.curry_a(2));
    }
}
