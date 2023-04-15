use core::marker::PhantomData;

use crate::{
    functional::Pointed,
    hlist::{
        cons::{ConsList, ConsPushBack, Uncons},
        tuple::Cons,
    },
};

use super::Closure;

pub trait CurryN<I> {
    type Curried;

    fn curry_n(self) -> Self::Curried;
}

impl<T, I> CurryN<I> for T
where
    T: Closure<I>,
    I: Cons,
{
    type Curried = CurriedN<T, (), I::Cons>;

    fn curry_n(self) -> Self::Curried {
        CurriedN(self, (), PhantomData)
    }
}

pub struct CurriedN<F, AO, AI>(F, AO, PhantomData<AI>);

impl<F, AI> Default for CurriedN<F, (), AI>
where
    F: Default,
{
    fn default() -> Self {
        CurriedN(Default::default(), (), PhantomData)
    }
}

impl<F, AO, AI> Clone for CurriedN<F, AO, AI>
where
    F: Clone,
    AO: Clone,
{
    fn clone(&self) -> Self {
        CurriedN(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<F, AO, AI> Copy for CurriedN<F, AO, AI>
where
    F: Copy,
    AO: Copy,
{
}

impl<F, AI> Pointed for CurriedN<F, (), AI>
where
    F: Closure<AI::Uncons>,
    AI: ConsList,
{
    type Pointed = F;

    fn point(unit: Self::Pointed) -> Self {
        CurriedN(unit, (), PhantomData)
    }
}

impl<F, AO, I> Closure<I> for CurriedN<F, AO, (I, ())>
where
    AO: ConsPushBack<I>,
    F: Closure<<AO::ConsPushBack as Uncons>::Uncons>,
{
    type Output = F::Output;

    fn call(self, input: I) -> Self::Output {
        self.0.call(self.1.cons_push_back(input).uncons())
    }
}

impl<F, AO, Tail, Tail2, I> Closure<I> for CurriedN<F, AO, (I, (Tail, Tail2))>
where
    AO: ConsPushBack<I>,
{
    type Output = CurriedN<F, AO::ConsPushBack, (Tail, Tail2)>;

    fn call(self, input: I) -> Self::Output {
        CurriedN(self.0, self.1.cons_push_back(input), PhantomData)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        derive_closure,
        functional::{function::curry_n::CurryN, Closure, Function},
    };

    #[test]
    fn test_curry_n() {
        struct ManyArgs;

        impl<A, B, C, D, E, F, G> Function<(A, B, C, D, E, F, G)> for ManyArgs {
            type Output = (A, B, C, D, E, F, G);

            fn call(input: (A, B, C, D, E, F, G)) -> Self::Output {
                input
            }
        }

        derive_closure!(ManyArgs);

        let curried = ManyArgs.curry_n();
        let curried = curried.call(1);
        let curried = curried.call(2.0);
        let curried = curried.call("three");
        let curried = curried.call(4);
        let curried = curried.call(5.0);
        let curried = curried.call("six");
        let curried = curried.call(7);
        assert_eq!(curried, (1, 2.0, "three", 4, 5.0, "six", 7));
    }
}
