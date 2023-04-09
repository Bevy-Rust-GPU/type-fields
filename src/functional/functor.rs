/// A `Pointed` type that can map a function over its wrapped value
pub trait Functor<A> {
    type Mapped<B>: Functor<B>;

    fn map<B, F>(self, f: F) -> <Self as Functor<A>>::Mapped<B>
    where
        F: Fn(A) -> B;
}
