mod sequence_a;
mod traverse;

pub use sequence_a::*;
pub use traverse::*;

use super::{Foldable, Functor};

pub trait Traversable: Functor + Foldable {
    type Traversed<F, P>
    where
        Self: Traverse<F, P>;
    type Sequenced<P>
    where
        Self: SequenceA<P>;

    fn traverse<F, P>(self, f: F) -> Self::Traversed<F, P>
    where
        Self: Traverse<F, P>;
    fn sequence_a<F, P>(self) -> Self::Sequenced<P>
    where
        Self: SequenceA<P>;
}

impl<T> Traversable for T {
    type Traversed<F, P> = T::Traverse
    where
        T: Traverse<F, P>;

    type Sequenced<P> = T::SequenceA
    where
        T: SequenceA<P>;

    fn traverse<F, P>(self, f: F) -> Self::Traversed<F, P>
    where
        Self: Traverse<F, P>,
    {
        Traverse::<F, P>::traverse(self, f)
    }

    fn sequence_a<F, P>(self) -> Self::Sequenced<P>
    where
        Self: SequenceA<P>,
    {
        SequenceA::<P>::sequence_a(self)
    }
}
