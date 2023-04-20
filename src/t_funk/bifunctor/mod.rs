mod both;
mod first;
mod second;

pub use both::*;
pub use first::*;
pub use second::*;

pub trait Bifunctor {
    type Firsted<F>
    where
        Self: First<F>;
    type Seconded<F>
    where
        Self: Second<F>;
    type Bothed<F1, F2>
    where
        Self: Both<F1, F2>;

    fn first<F>(self, f: F) -> Self::Firsted<F>
    where
        Self: First<F>;

    fn second<F>(self, f: F) -> Self::Seconded<F>
    where
        Self: Second<F>;

    fn both<F1, F2>(self, f1: F1, f2: F2) -> Self::Bothed<F1, F2>
    where
        Self: Both<F1, F2>;
}

impl<T> Bifunctor for T {
    type Firsted<F> = T::First
    where
        T: First<F>;

    type Seconded<F> = T::Second
    where
        T: Second<F>;

    type Bothed<F1, F2> = T::Both
    where
        T:Both<F1, F2>;

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

    fn both<F1, F2>(self, f1: F1, f2: F2) -> Self::Bothed<F1, F2>
    where
        T: Both<F1, F2>,
    {
        Both::<F1, F2>::both(self, f1, f2)
    }
}
