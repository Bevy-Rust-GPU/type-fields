use super::Functor;

/// A `Functor` type that can take a wrapped function,
/// map it over a provided value, and wrap the result
pub trait Applicative<F>: Functor<F> {
    fn apply<B, A>(self, a: A) -> B
    where
        F: Fn(A) -> B;
}
