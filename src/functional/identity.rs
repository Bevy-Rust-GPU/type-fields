use super::{Applicative, Copointed, Function, Functor, Monad, Pointed};

/// Identity monad, used to lift values into a monadic context.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<T> Function<T> for Identity<T> {
    type Output = Self;

    fn call(self, input: T) -> Self::Output {
        Identity(input)
    }
}

impl<T, F> Functor<F> for Identity<T>
where
    F: Function<T>,
{
    type Mapped = Identity<F::Output>;

    fn fmap(self, f: F) -> Self::Mapped {
        Pointed::point(f.call(self.copoint()))
    }
}

impl<T, U> Applicative<U> for Identity<T>
where
    T: Function<U>,
{
    type Applied = T::Output;

    fn apply(self, a: U) -> Self::Applied
    where
        T: Function<U>,
    {
        self.copoint().call(a)
    }
}

impl<T, F> Monad<F> for Identity<T>
where
    F: Function<T>,
{
    type Chained = F::Output;

    fn chain(self, f: F) -> Self::Chained {
        f.call(self.copoint())
    }
}

/*
impl<T> Monoid for Identity<T>
where
    T: Monoid,
{
    type Identity = Identity<T::Identity>;

    fn mempty() -> Self::Identity {
        Pointed::point(T::mempty())
    }
}

impl<T, U> Semigroup<Identity<U>> for Identity<T>
where
    T: Semigroup<U>,
{
    type Appended = Identity<T::Appended>;

    fn mappend(self, t: Identity<U>) -> Self::Appended {
        Pointed::point(self.0.mappend(t.copoint()))
    }
}
*/

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
