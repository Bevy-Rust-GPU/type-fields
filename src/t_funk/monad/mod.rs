mod chain;
mod inst;
mod then;

pub use chain::*;
pub use inst::*;
pub use then::*;

use super::Applicative;

pub trait Monad: Applicative {
    type Chained<F>
    where
        Self: Chain<F>;
    type Then<F>
    where
        Self: Then<F>;

    fn chain<F>(self, f: F) -> Self::Chained<F>
    where
        Self: Chain<F>;

    fn then<F>(self, f: F) -> <Self as Monad>::Then<F>
    where
        Self: Then<F>;
}

impl<T> Monad for T {
    type Chained<F> = T::Chain where T: Chain<F>;

    type Then<F> = T::Then where T: Then<F>;

    fn chain<F>(self, f: F) -> Self::Chained<F>
    where
        Self: Chain<F>,
    {
        Chain::chain(self, f)
    }

    fn then<F>(self, f: F) -> <Self as Monad>::Then<F>
    where
        Self: Then<F>,
    {
        Then::then(self, f)
    }
}
