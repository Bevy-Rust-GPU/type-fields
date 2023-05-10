use crate::t_funk::{
    applicative::Pure, function::Id, Apply, Closure, Curried2, Curry2, Flip, Flipped, Fmap,
    SequenceA, Traverse,
};

use super::{Cons, Nil, PushFrontF};

impl<T, N, F, P> Traverse<F, P> for Cons<T, N>
where
    F: Clone + Closure<T>,
    F::Output: Fmap<Curried2<Flipped<PushFrontF>>>,
    <F::Output as Fmap<Curried2<Flipped<PushFrontF>>>>::Fmap: Apply<N::Traverse>,
    N: Traverse<F, P>,
{
    type Traverse =
        <<F::Output as Fmap<Curried2<Flipped<PushFrontF>>>>::Fmap as Apply<N::Traverse>>::Apply;

    fn traverse(self, f: F) -> Self::Traverse {
        f.clone()
            .call(self.0)
            .fmap(PushFrontF.flip().curry())
            .apply(self.1.traverse(f))
    }
}

impl<F, P> Traverse<F, P> for Nil
where
    P: Pure,
{
    type Traverse = P::Pure<Self>;

    fn traverse(self, _: F) -> Self::Traverse {
        P::pure::<Self>(self)
    }
}

impl<T, N, P> SequenceA<P> for Cons<T, N>
where
    Self: Traverse<Id, P>,
{
    type SequenceA = <Self as Traverse<Id, P>>::Traverse;

    fn sequence_a(self) -> Self::SequenceA {
        self.traverse(Id)
    }
}

impl<P> SequenceA<P> for Nil
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
    use crate::t_funk::{function::Id, hlist::Nil, tlist::ToHList, SequenceA, Traverse};

    #[test]
    fn test_traverse() {
        let list = ((0,).to_hlist(), (0, 1).to_hlist(), (0, 1, 2).to_hlist()).to_hlist();
        let decafisbad = Traverse::<Id, Nil>::traverse(list, Id);
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
    fn test_sequence_a() {
        let list = ((0,).to_hlist(), (0, 1).to_hlist(), (0, 1, 2).to_hlist()).to_hlist();
        let decafisbad = SequenceA::<Nil>::sequence_a(list);
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
