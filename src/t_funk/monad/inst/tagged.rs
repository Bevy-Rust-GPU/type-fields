use core::marker::PhantomData;

use crate::t_funk::{
    Apply, Chain, Closure, Copointed, Fmap, Fold, Foldr, Id, Mappend, Mconcat, Mempty, Pointed,
    Replace, Then,
};

/// Phantom monad, used to lift values into a monadic context
/// alongside some additional type parameter to make them
/// distinct from other instances of the same type.
pub struct Tagged<M, T>(pub PhantomData<M>, pub T);

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

impl<P, A, F> Fmap<F> for Tagged<P, A>
where
    F: Closure<A>,
{
    type Fmap = Tagged<P, F::Output>;

    fn fmap(self, f: F) -> Self::Fmap {
        Tagged::point(f.call(self.copoint()))
    }
}

impl<P, A, B> Replace<B> for Tagged<P, A> {
    type Replace = Tagged<P, B>;

    fn replace(self, t: B) -> Self::Replace {
        Tagged::point(t)
    }
}

impl<P, T, U> Apply<U> for Tagged<P, T>
where
    T: Closure<U>,
{
    type Apply = T::Output;

    fn apply(self, a: U) -> Self::Apply
    where
        T: Closure<U>,
    {
        self.copoint().call(a)
    }
}

impl<P, T, F> Chain<F> for Tagged<P, T>
where
    F: Closure<T>,
{
    type Chain = F::Output;

    fn chain(self, f: F) -> Self::Chain {
        f.call(self.copoint())
    }
}

impl<P, T, F> Then<F> for Tagged<P, T> {
    type Then = F;

    fn then(self, f: F) -> Self::Then {
        self.replace(Id).apply(f)
    }
}

impl<P, T> Mempty for Tagged<P, T>
where
    T: Mempty,
{
    type Mempty = Tagged<P, T::Mempty>;

    fn mempty() -> Self::Mempty {
        Tagged::point(T::mempty())
    }
}

impl<P, T, U> Mappend<Tagged<P, U>> for Tagged<P, T>
where
    T: Mappend<U>,
{
    type Mappend = Tagged<P, T::Mappend>;

    fn mappend(self, t: Tagged<P, U>) -> Self::Mappend {
        Pointed::point(self.1.mappend(t.copoint()))
    }
}

impl<P, T> Mconcat for Tagged<P, T>
where
    T: Mconcat,
{
    type Mconcat = Tagged<P, T::Mconcat>;

    fn mconcat(self) -> Self::Mconcat {
        Tagged::point(self.copoint().mconcat())
    }
}

impl<P, T, F, Z> Foldr<F, Z> for Tagged<P, T>
where
    T: Foldr<F, Z>,
{
    type Foldr = Tagged<P, T::Foldr>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        Tagged::point(self.copoint().foldr(f, z))
    }
}

impl<P, T> Fold for Tagged<P, T>
where
    T: Fold,
{
    type Fold = Tagged<P, T::Fold>;

    fn fold(self) -> Self::Fold {
        Pointed::point(self.copoint().fold())
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        test_functor_laws, Add, Apply, Chain, Closure, Composed, Copointed, Curry, CurryN, Div,
        Flip, Fmap, FmapF, Mul, PointF, Pointed, Sub, Tagged, Then,
    };

    #[test]
    fn test_tagged() {
        enum Tag {}

        let id1: Tagged<Tag, i32> = Tagged::<Tag, _>::point(5);
        assert_eq!(id1.copoint(), 5);

        let id2: Tagged<Tag, i32> = id1.fmap(Mul.curry_n().call(3));
        assert_eq!(id2.copoint(), 15);

        let id3: Tagged<Tag, i32> = Tagged::<Tag, _>::point(
            FmapF::default()
                .flip()
                .curry_n()
                .call(Sub.flip().curry_n().call(3)),
        )
        .apply(id2);
        assert_eq!(id3.copoint(), 12);

        let id4: Tagged<Tag, i32> = id3.chain(
            Composed::point((Div.flip(), PointF::<Tagged<Tag, _>>::default()))
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
