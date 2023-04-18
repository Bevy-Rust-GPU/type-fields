pub trait Traversable<F, P> {
    type Traversed;

    fn traverse(self, f: F) -> Self::Traversed;
}

pub trait SequenceA<P> {
    type Sequenced;

    fn sequence_a(self) -> Self::Sequenced;
}
