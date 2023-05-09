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

    type Composed<F> = T::Compose where T: Compose<F>;

    fn id(self) -> Self::Identity
    where
        T: Id,
    {
        <T as Id>::id()
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
    use crate::t_funk::{category::Compose, category::Id, Add, Closure, Curry2};

    #[test]
    fn test_category() {
        let foo = Add.suffix(3);
        let bar = Add::id();
        let baz = foo.compose(bar);
        let res = baz.call(1234);
        assert_eq!(res, 1237)
    }
}
