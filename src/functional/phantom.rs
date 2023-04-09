use core::marker::PhantomData;

use super::{Applicative, Copointed, Functor, Monad, Pointed};

/// Phantom monad, used to lift values into a monadic context
/// alongside some additional type parameter to make them
/// distinct from other instances of the same type.
pub struct Phantom<P, T>(T, PhantomData<P>);

impl<P, T> Default for Phantom<P, T>
where
    T: Default,
{
    fn default() -> Self {
        Phantom::<P, T>::of(Default::default())
    }
}

impl<P, T> Clone for Phantom<P, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Phantom::<P, T>::of(self.0.clone())
    }
}

impl<P, T> Copy for Phantom<P, T> where T: Copy {}

impl<P, T> PartialEq for Phantom<P, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<P, T> Eq for Phantom<P, T> where T: Eq {}

impl<P, T> PartialOrd for Phantom<P, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<P, T> Ord for Phantom<P, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<P, T> core::hash::Hash for Phantom<P, T>
where
    T: core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<P, T> Pointed for Phantom<P, T> {
    type Pointed = T;

    fn of(unit: Self::Pointed) -> Self {
        Phantom(unit, PhantomData)
    }
}

impl<P, T> Copointed for Phantom<P, T> {
    type Copointed = T;

    fn unwrap(self) -> Self::Copointed {
        self.0
    }
}

impl<P, A> Functor<A> for Phantom<P, A> {
    type Mapped<B> = Phantom<P, B>;

    fn map<B, F>(self, f: F) -> Self::Mapped<B>
    where
        F: Fn(A) -> B,
    {
        Phantom::<P, B>::of(f(self.unwrap()))
    }
}

impl<P, T> Applicative<T> for Phantom<P, T> {
    fn apply<B, A>(self, a: A) -> B
    where
        T: Fn(A) -> B,
    {
        self.unwrap()(a)
    }
}

impl<P, T> Monad<T> for Phantom<P, T> {
    fn chain<M, F>(self, f: F) -> M
    where
        F: Fn(T) -> M,
    {
        f(self.unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{
        phantom::Phantom, Applicative, Copointed, Functor, Monad, Pointed,
    };

    #[test]
    fn test_phantom() {
        enum Tag {}

        let id1 = Phantom::<Tag, _>::of(5);
        let id2: Phantom<Tag, _> = id1.map(|x| x * 3);
        let id3: Phantom<Tag, _> =
            Phantom::<Tag, _>::of(|x: Phantom<Tag, _>| x.map(|y| y - 3)).apply(id2);
        let id4 = id3.chain(|x| Phantom::<Tag, _>::of(x / 3));
        assert_eq!(id1.unwrap(), 5);
        assert_eq!(id2.unwrap(), 15);
        assert_eq!(id3.unwrap(), 12);
        assert_eq!(id4.unwrap(), 4);
    }
}

