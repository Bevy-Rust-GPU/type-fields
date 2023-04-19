use core::marker::PhantomData;

use crate::{
    functional::{
        ApplyF, Closure, Curried, CurriedA, Curry, Flip, Flipped, Fmap, Function, LiftA2, Pure,
        Then,
    },
    hlist::cons::PushFront,
};

pub struct ReplicateM<C, P>(PhantomData<(C, P)>);

impl<F, Next, P> Function<F> for ReplicateM<(Next,), P>
where
    ReplicateM<Next, P>: Function<F>,
    F: Clone + Fmap<Curried<Flipped<PushFront>>>,
    CurriedA<ApplyF, <F as Fmap<Curried<Flipped<PushFront>>>>::Fmap>:
        Closure<<ReplicateM<Next, P> as Function<F>>::Output>,
{
    type Output = <CurriedA<ApplyF, <F as Fmap<Curried<Flipped<PushFront>>>>::Fmap> as Closure<
        <ReplicateM<Next, P> as Function<F>>::Output,
    >>::Output;

    fn call(f: F) -> Self::Output {
        LiftA2
            .call((PushFront.flip().curry(), f.clone()))
            .call(ReplicateM::<Next, P>::call(f))
    }
}

impl<F, P> Function<F> for ReplicateM<(), P>
where
    P: Pure,
{
    type Output = P::Pure<()>;

    fn call(_: F) -> Self::Output {
        P::pure(())
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
