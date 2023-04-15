use crate::functional::{
    Applicative, Closure, Curried, Curry, Flip, Flipped, Functor, Id, SequenceA, Traversable,
};

use super::PushFront;

impl<Head, Tail, F> Traversable<F> for (Head, Tail)
where
    F: Clone + Closure<Head>,
    F::Output: Functor<Curried<Flipped<PushFront>>>,
    <F::Output as Functor<Curried<Flipped<PushFront>>>>::Mapped: Applicative<Tail::Traversed>,
    Tail: Traversable<F>,
{
    type Traversed = <<F::Output as Functor<Curried<Flipped<PushFront>>>>::Mapped as Applicative<
        Tail::Traversed,
    >>::Applied;

    fn traverse(self, f: F) -> Self::Traversed {
        f.clone()
            .call(self.0)
            .fmap(PushFront.flip().curry())
            .apply(self.1.traverse(f))
    }
}

impl<F> Traversable<F> for () {
    type Traversed = ((), ());

    fn traverse(self, _: F) -> Self::Traversed {
        ((), ())
    }
}

impl<T> SequenceA for T
where
    Self: Traversable<Id>,
{
    type Sequenced = <Self as Traversable<Id>>::Traversed;

    fn sequence_a(self) -> Self::Sequenced {
        self.traverse(Id)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        functional::{Id, SequenceA, Traversable},
        hlist::tuple::Cons,
    };

    #[test]
    fn test_sequence_a() {
        let list = ((0,).cons(), (0, 1).cons(), (0, 1, 2).cons()).cons();
        let decafisbad = list.sequence_a();
        assert_eq!(
            decafisbad,
            (
                (0, 0, 0).cons(),
                (0, 0, 1).cons(),
                (0, 0, 2).cons(),
                (0, 1, 0).cons(),
                (0, 1, 1).cons(),
                (0, 1, 2).cons()
            )
                .cons()
        );
    }

    #[test]
    fn test_traverse() {
        let list = ((0,).cons(), (0, 1).cons(), (0, 1, 2).cons()).cons();
        let decafisbad = list.traverse(Id);
        assert_eq!(
            decafisbad,
            (
                (0, 0, 0).cons(),
                (0, 0, 1).cons(),
                (0, 0, 2).cons(),
                (0, 1, 0).cons(),
                (0, 1, 1).cons(),
                (0, 1, 2).cons()
            )
                .cons()
        );
    }
}
