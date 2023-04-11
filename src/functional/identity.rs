use super::{Applicative, Copointed, Function, Functor, Monad, Pointed};

/// Identity monad, used to lift values into a monadic context.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identity<T>(T);

impl<T> Pointed for Identity<T> {
    type Pointed = T;

    fn point(unit: Self::Pointed) -> Self {
        Identity(unit)
    }
}

impl<T> Copointed for Identity<T> {
    type Copointed = T;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<A> Functor<A> for Identity<A> {
    type Mapped<B> = Identity<B>;

    fn fmap<B, F>(self, f: F) -> Self::Mapped<B>
    where
        F: Function<A, Output = B>,
    {
        Identity::point(f.call(self.copoint()))
    }
}

impl<T> Applicative<T> for Identity<T> {
    fn apply<B, A>(self, a: A) -> B
    where
        T: Function<A, Output = B>,
    {
        self.copoint().call(a)
    }
}

impl<T> Monad<T> for Identity<T> {
    fn chain<M, F>(self, f: F) -> M
    where
        F: Function<T, Output = M>,
    {
        f.call(self.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{
        Applicative, Copointed, FunctionFn, Functor, Identity, Monad, Pointed,
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
        assert_eq!(id1.copoint(), 5);
        assert_eq!(id2.copoint(), 15);
        assert_eq!(id3.copoint(), 12);
        assert_eq!(id4.copoint(), 4);
        assert_eq!(id5.copoint(), 1234);
    }
}
