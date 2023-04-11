use core::marker::PhantomData;

use super::{Applicative, Copointed, Function, Functor, Monad, Pointed};

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

impl<P, T> Default for Tagged<P, T>
where
    T: Default,
{
    fn default() -> Self {
        Tagged::<P, T>::point(Default::default())
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

impl<P, A> Functor<A> for Tagged<P, A> {
    type Mapped<B> = Tagged<P, B>;

    fn fmap<B, F>(self, f: F) -> Self::Mapped<B>
    where
        F: Function<A, Output = B>,
    {
        Tagged::<P, B>::point(f.call(self.copoint()))
    }
}

impl<P, T> Applicative<T> for Tagged<P, T> {
    fn apply<B, A>(self, a: A) -> B
    where
        T: Function<A, Output = B>,
    {
        self.copoint().call(a)
    }
}

impl<P, T> Monad<T> for Tagged<P, T> {
    fn chain<M, F>(self, f: F) -> M
    where
        F: Function<T, Output = M>,
    {
        f.call(self.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{Applicative, Copointed, FunctionFn, Functor, Monad, Pointed, Tagged};

    #[test]
    fn test_phantom() {
        enum Tag {}

        let id1 = Tagged::<Tag, _>::point(5);
        let id2: Tagged<Tag, _> = id1.fmap(FunctionFn::point(|x| x * 3));
        let id3: Tagged<Tag, _> =
            Tagged::<Tag, _>::point(FunctionFn::point(|x: Tagged<Tag, _>| {
                x.fmap(FunctionFn::point(|y| y - 3))
            }))
            .apply(id2);
        let id4 = id3.chain(FunctionFn::point(|x| Tagged::<Tag, _>::point(x / 3)));
        let id5 = id4.then(FunctionFn::point(|_| Tagged::<Tag, _>::point(1234)));
        assert_eq!(id1.copoint(), 5);
        assert_eq!(id2.copoint(), 15);
        assert_eq!(id3.copoint(), 12);
        assert_eq!(id4.copoint(), 4);
        assert_eq!(id5.copoint(), 1234);
    }
}
