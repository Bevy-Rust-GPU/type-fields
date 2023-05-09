use core::marker::PhantomData;

use crate::{
    macros::{
        arrow::{Arr, Fanout, First, Second, Split},
        category::{Compose, Id},
    },
    t_funk::{Apply, Curried2, Curry2, hlist::Nil},
};

use crate::t_funk::{
    applicative::Pure, hlist::PushFrontF, Closure, Flip, Flipped, Fmap, Function, LiftA2, Then,
};

#[derive(Id, Compose, Arr, First, Second, Split, Fanout)]
pub struct ReplicateM<C, P>(PhantomData<(C, P)>);

impl<F, Next, P> Function<F> for ReplicateM<(Next,), P>
where
    ReplicateM<Next, P>: Function<F>,
    F: Clone + Fmap<Curried2<Flipped<PushFrontF>>>,
    F::Fmap: Apply<<ReplicateM<Next, P> as Function<F>>::Output>,
{
    type Output = <<F as Fmap<Curried2<Flipped<PushFrontF>>>>::Fmap as Apply<
        <ReplicateM<Next, P> as Function<F>>::Output,
    >>::Apply;

    fn call(f: F) -> Self::Output {
        LiftA2.call((
            PushFrontF.flip().curry(),
            f.clone(),
            ReplicateM::<Next, P>::call(f),
        ))
    }
}

impl<F, P> Function<F> for ReplicateM<(), P>
where
    P: Pure,
{
    type Output = P::Pure<Nil>;

    fn call(_: F) -> Self::Output {
        P::pure(Nil)
    }
}

pub struct ReplicateMDiscard<C>(PhantomData<C>);

impl<F, Next> Function<F> for ReplicateMDiscard<(Next,)>
where
    F: Clone + Then<<ReplicateMDiscard<Next> as Function<F>>::Output>,
    ReplicateMDiscard<Next>: Function<F>,
{
    type Output = F::Then;

    fn call(f: F) -> Self::Output {
        f.clone().then(ReplicateMDiscard::<Next>::call(f))
    }
}

impl<F> Function<F> for ReplicateMDiscard<()> {
    type Output = F;

    fn call(f: F) -> Self::Output {
        f
    }
}
