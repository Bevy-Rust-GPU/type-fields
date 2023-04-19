mod first;
mod second;

pub use first::*;
pub use second::*;

pub trait Bifunctor {
    type Firsted<F>
    where
        Self: First<F>;
    type Seconded<F>
    where
        Self: Second<F>;

    fn first<F>(self, f: F) -> Self::Firsted<F>
    where
        Self: First<F>;

    fn second<F>(self, f: F) -> Self::Seconded<F>
    where
        Self: Second<F>;
}

impl<T> Bifunctor for T {
    type Firsted<F> = T::First
    where
        T: First<F>;

    type Seconded<F> = T::Second
    where
        T: Second<F>;

    fn first<F>(self, f: F) -> Self::Firsted<F>
    where
        T: First<F>,
    {
        First::<F>::first(self, f)
    }

    fn second<F>(self, f: F) -> Self::Seconded<F>
    where
        T: Second<F>,
    {
        Second::<F>::second(self, f)
    }
}
