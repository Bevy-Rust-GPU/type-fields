use super::{Applicative, Copointed, Functor, Monad, Pointed};

/// Identity monad, used to lift values into a monadic context.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identity<T>(T);

impl<T> Pointed for Identity<T> {
    type Pointed = T;

    fn of(unit: Self::Pointed) -> Self {
        Identity(unit)
    }
}

impl<T> Copointed for Identity<T> {
    type Copointed = T;

    fn unwrap(self) -> Self::Copointed {
        self.0
    }
}

impl<A> Functor<A> for Identity<A> {
    type Mapped<B> = Identity<B>;

    fn map<B, F>(self, f: F) -> Self::Mapped<B>
    where
        F: Fn(A) -> B,
    {
        Identity::of(f(self.unwrap()))
    }
}

impl<T> Applicative<T> for Identity<T> {
    fn apply<B, A>(self, a: A) -> B
    where
        T: Fn(A) -> B,
    {
        self.unwrap()(a)
    }
}

impl<T> Monad<T> for Identity<T> {
    fn chain<M, F>(self, f: F) -> M
    where
        F: Fn(T) -> M,
    {
        f(self.unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{Applicative, Copointed, Functor, Identity, Monad, Pointed};

    #[test]
    fn test_identity() {
        let id1 = Identity::of(5);
        let id2: Identity<i32> = id1.map(|x| x * 3);
        let id3: Identity<i32> = Identity::of(|x: Identity<i32>| x.map(|y| y - 3)).apply(id2);
        let id4 = id3.chain(|x| Identity::of(x / 3));
        assert_eq!(id1.unwrap(), 5);
        assert_eq!(id2.unwrap(), 15);
        assert_eq!(id3.unwrap(), 12);
        assert_eq!(id4.unwrap(), 4);
    }
}
