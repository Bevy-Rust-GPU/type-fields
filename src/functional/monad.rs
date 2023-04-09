use super::Applicative;

/// An `Applicative` type that can flat-map a function over its wrapped value
pub trait Monad<T>: Applicative<T> {
    fn chain<M, F>(self, f: F) -> M
    where
        F: Fn(T) -> M;
}
