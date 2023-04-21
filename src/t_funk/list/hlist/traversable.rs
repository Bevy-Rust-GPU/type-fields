use crate::t_funk::{
    Apply, Closure, Curried, Curry, Flip, Flipped, Fmap, Id, Pure, SequenceA, Traverse,
};

use super::PushFrontF;

impl<Head, Tail, F, P> Traverse<F, P> for (Head, Tail)
where
    F: Clone + Closure<Head>,
    F::Output: Fmap<Curried<Flipped<PushFrontF>>>,
    <F::Output as Fmap<Curried<Flipped<PushFrontF>>>>::Fmap: Apply<Tail::Traverse>,
    Tail: Traverse<F, P>,
{
    type Traverse =
        <<F::Output as Fmap<Curried<Flipped<PushFrontF>>>>::Fmap as Apply<Tail::Traverse>>::Apply;

    fn traverse(self, f: F) -> Self::Traverse {
        f.clone()
            .call(self.0)
            .fmap(PushFrontF.flip().curry())
            .apply(self.1.traverse(f))
    }
}

impl<F, P> Traverse<F, P> for ()
where
    P: Pure,
{
    type Traverse = P::Pure<Self>;

    fn traverse(self, _: F) -> Self::Traverse {
        P::pure::<()>(self)
    }
}

impl<Head, Tail, P> SequenceA<P> for (Head, Tail)
where
    Self: Traverse<Id, P>,
{
    type SequenceA = <Self as Traverse<Id, P>>::Traverse;

    fn sequence_a(self) -> Self::SequenceA {
        self.traverse(Id)
    }
}

impl<P> SequenceA<P> for ()
where
    Self: Traverse<Id, P>,
{
    type SequenceA = <Self as Traverse<Id, P>>::Traverse;

    fn sequence_a(self) -> Self::SequenceA {
        self.traverse(Id)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{tlist::ToHList, Id, SequenceA, Traverse};

    #[test]
    fn test_sequence_a() {
        let list = ((0,).to_hlist(), (0, 1).to_hlist(), (0, 1, 2).to_hlist()).to_hlist();
        let decafisbad = SequenceA::<()>::sequence_a(list);
        assert_eq!(
            decafisbad,
            (
                (0, 0, 0).to_hlist(),
                (0, 0, 1).to_hlist(),
                (0, 0, 2).to_hlist(),
                (0, 1, 0).to_hlist(),
                (0, 1, 1).to_hlist(),
                (0, 1, 2).to_hlist()
            )
                .to_hlist()
        );
    }

    #[test]
    fn test_traverse() {
        let list = ((0,).to_hlist(), (0, 1).to_hlist(), (0, 1, 2).to_hlist()).to_hlist();
        let decafisbad = Traverse::<Id, ()>::traverse(list, Id);
        assert_eq!(
            decafisbad,
            (
                (0, 0, 0).to_hlist(),
                (0, 0, 1).to_hlist(),
                (0, 0, 2).to_hlist(),
                (0, 1, 0).to_hlist(),
                (0, 1, 1).to_hlist(),
                (0, 1, 2).to_hlist()
            )
                .to_hlist()
        );
    }
}
