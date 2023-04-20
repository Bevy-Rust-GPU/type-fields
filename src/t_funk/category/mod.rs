use crate::t_funk::{Compose, Composed, Function, Id};

/// A set of types with an identity function and composition operator.
pub trait Category<Of> {
    type Composed<F>
    where
        Self: Compose<F>;

    fn id(self) -> Self;

    fn compose<F>(self, f: F) -> <Self as Category<Of>>::Composed<F>
    where
        Self: Compose<F>;
}

/// Closure category
pub enum ClosureC {}

impl<T> Category<ClosureC> for T {
    type Composed<F> = Composed<Self, F> where Self: Compose<F>;

    fn id(self) -> T {
        Id::call(self)
    }

    fn compose<F>(self, f: F) -> <Self as Category<ClosureC>>::Composed<F>
    where
        Self: Compose<F>,
    {
        Compose::compose(self, f)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{Closure, Id};

    use super::Category;

    #[test]
    fn test_category() {
        let foo = Id;
        let bar = foo.id();
        let baz = foo.compose(bar);
        let res = baz.call(1234);
        assert_eq!(res, 1234)
    }
}
