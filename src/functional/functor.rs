use crate::functional::fconst;

/// A type that can map a function over its wrapped value
pub trait Functor<A> {
    type Mapped<B>: Functor<B>;

    /// `<$>`
    fn fmap<B, F>(self, f: F) -> <Self as Functor<A>>::Mapped<B>
    where
        F: FnOnce(A) -> B;

    /// `<$`
    fn replace<B>(self, t: B) -> <Self as Functor<A>>::Mapped<B>
    where
        Self: Sized,
    {
        self.fmap(fconst(t))
    }
}

/// A type that can emplace itself within a functor
pub trait FunctorEmplace: Sized {
    /// `$>`
    fn emplace<A, F>(self, f: F) -> <F as Functor<A>>::Mapped<Self>
    where
        F: Functor<A>,
    {
        f.fmap(fconst(self))
    }
}

impl<T> FunctorEmplace for T {}
