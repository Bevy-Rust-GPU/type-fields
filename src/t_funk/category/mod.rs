mod compose;
mod id;

pub use compose::*;
pub use id::*;

/// A set of types with an identity function and composition operator.
pub trait Category {
    type Identity
    where
        Self: Id;

    type Composed<F>
    where
        Self: Compose<F>;

    fn id(self) -> Self::Id
    where
        Self: Id;

    fn compose<F>(self, f: F) -> <Self as Category>::Composed<F>
    where
        Self: Compose<F>;
}

impl<T> Category for T {
    type Identity = T::Id where T: Id;

    type Composed<F> = Composed<T, F> where T: Compose<F>;

    fn id(self) -> Self::Identity
    where
        T: Id,
    {
        Id::id(self)
    }

    fn compose<F>(self, f: F) -> <Self as Category>::Composed<F>
    where
        Self: Compose<F>,
    {
        Compose::compose(self, f)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{category::Id, Add, Closure, Compose, Curry};

    #[test]
    fn test_category() {
        let foo = Add.curry_b(3);
        let bar = foo.id();
        let baz = foo.compose(bar);
        let res = baz.call(1234);
        assert_eq!(res, 1240)
    }
}
