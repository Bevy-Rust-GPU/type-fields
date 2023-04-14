pub trait Traversable<F> {
    type Traversed;

    fn traverse(self, f: F) -> Self::Traversed;
}

pub trait SequenceA {
    type Sequenced;

    fn sequence_a(self) -> Self::Sequenced;
}
