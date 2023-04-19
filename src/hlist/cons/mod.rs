//! Traits implemented over cons list types.

mod cons_append;
mod cons_applicative;
mod cons_as_mut;
mod cons_as_ref;
mod cons_clone;
mod cons_fold_left;
mod cons_fold_right;
mod cons_functor;
mod cons_get;
mod cons_gets;
mod cons_length;
mod cons_list;
mod cons_list_mut;
mod cons_list_ref;
mod cons_monad;
mod cons_monoid;
mod cons_pop_back;
mod cons_pop_front;
mod cons_push_back;
mod cons_push_front;
mod cons_remove;
mod cons_semigroup;
mod cons_set;
mod cons_sets;
mod cons_traversable;
mod uncons;

pub use cons_append::*;
pub use cons_applicative::*;
pub use cons_as_mut::*;
pub use cons_as_ref::*;
pub use cons_clone::*;
pub use cons_fold_left::*;
pub use cons_fold_right::*;
pub use cons_functor::*;
pub use cons_get::*;
pub use cons_gets::*;
pub use cons_length::*;
pub use cons_list::*;
pub use cons_list_mut::*;
pub use cons_list_ref::*;
pub use cons_pop_back::*;
pub use cons_pop_front::*;
pub use cons_push_back::*;
pub use cons_push_front::*;
pub use cons_remove::*;
pub use cons_semigroup::*;
pub use cons_set::*;
pub use cons_sets::*;
pub use cons_traversable::*;
pub use uncons::*;

#[cfg(test)]
mod do_notation_monadic {
    use core::{
        marker::PhantomData,
        ops::{BitAnd, Shr},
    };

    use crate::{
        derive_closure,
        functional::{
            Closure, Compose, Composed, Copointed, CurriedA, Curry, FirstF, Flip, Flipped,
            Function, Chain, Pointed, Pure, Spread, Spreaded, State, Tagged, Then, Tuple,
        },
        hlist::{
            cons::{ConsGet, ConsSet, Uncons},
            tuple::Cons,
        },
    };

    #[test]
    fn test_monadic_do() {
        struct StateGet<M, T, P>(PhantomData<(M, T, P)>);

        impl<M, T, P> Default for StateGet<M, T, P> {
            fn default() -> Self {
                StateGet(PhantomData)
            }
        }

        impl<M, T, P> Clone for StateGet<M, T, P> {
            fn clone(&self) -> Self {
                StateGet(PhantomData)
            }
        }

        impl<M, T, P> Copy for StateGet<M, T, P> {}

        impl<C, M, T, P> Function<C> for StateGet<M, T, P>
        where
            C: ConsGet<Tagged<M, T>, P>,
        {
            type Output = T;

            fn call(ctx: C) -> Self::Output {
                ctx.cons_get().copoint()
            }
        }

        derive_closure!(StateGet<M, T, P>);

        struct StateSet<U, P>(PhantomData<(U, P)>);

        impl<U, P> Default for StateSet<U, P> {
            fn default() -> Self {
                StateSet(PhantomData)
            }
        }

        impl<U, P> Clone for StateSet<U, P> {
            fn clone(&self) -> Self {
                StateSet(PhantomData)
            }
        }

        impl<U, P> Copy for StateSet<U, P> {}

        impl<U, P, C, T> Function<(T, C)> for StateSet<U, P>
        where
            C: ConsSet<Tagged<U, T>, P>,
        {
            type Output = ((), C);

            fn call((t, ctx): (T, C)) -> Self::Output {
                ((), ctx.cons_set(Tagged::point(t)))
            }
        }

        derive_closure!(StateSet<U, P>);

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Test;

        impl Function<(i32, f32, &str)> for Test {
            type Output = f32;

            fn call((a, b, _): (i32, f32, &str)) -> Self::Output {
                a as f32 + b
            }
        }

        derive_closure!(Test);

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Push;

        impl<F, I> Function<(F, I)> for Push {
            type Output = State<
                Composed<
                    Spreaded<Flipped<Tuple>>,
                    CurriedA<Flipped<FirstF>, Composed<F, CurriedA<Flipped<Tuple>, I>>>,
                >,
            >;

            fn call((f, i): (F, I)) -> Self::Output {
                State::point(
                    Tuple
                        .flip()
                        .spread()
                        .compose(FirstF.flip().curry_a(f.compose(Tuple.flip().curry_a(i)))),
                )
            }
        }

        derive_closure!(Push);

        struct Run;

        impl<F, I> Function<(F, I)> for Run
        where
            F: Closure<I::Uncons>,
            I: Uncons,
        {
            type Output = State<CurriedA<Tuple, F::Output>>;

            fn call((f, i): (F, I)) -> Self::Output {
                State::point(Tuple.curry_a(f.call(i.uncons())))
            }
        }

        derive_closure!(Run);

        struct Set<M, P>(PhantomData<(M, P)>);

        impl<M, P> Default for Set<M, P> {
            fn default() -> Self {
                Set(PhantomData)
            }
        }

        impl<M, P> Clone for Set<M, P> {
            fn clone(&self) -> Self {
                Set(PhantomData)
            }
        }

        impl<M, P, I> Function<I> for Set<M, P> {
            type Output = State<CurriedA<StateSet<Float, P>, I>>;

            fn call(input: I) -> Self::Output {
                State::point(StateSet::<Float, P>::default().curry_a(input))
            }
        }

        derive_closure!(Set<M, P>);

        struct Take<M, T, P>(PhantomData<(M, T, P)>);

        impl<M, T, P> Default for Take<M, T, P> {
            fn default() -> Self {
                Take(PhantomData)
            }
        }

        impl<M, T, P> Clone for Take<M, T, P> {
            fn clone(&self) -> Self {
                Take(PhantomData)
            }
        }

        derive_closure!(Take<M, T, P>);

        impl<M, T, P, I> Function<I> for Take<M, T, P> {
            type Output = State<CurriedA<StateGet<M, T, P>, I>>;

            fn call(input: I) -> Self::Output {
                State::point(StateGet::<M, T, P>::default().curry_a(input))
            }
        }

        impl<F, Rhs> Shr<Rhs> for State<F>
        where
            State<F>: Chain<Rhs>,
        {
            type Output = <State<F> as Chain<Rhs>>::Chain;

            fn shr(self, rhs: Rhs) -> Self::Output {
                self.chain(rhs)
            }
        }

        impl<F, Rhs> BitAnd<Rhs> for State<F>
        where
            State<F>: Then<Rhs>,
        {
            type Output = <State<F> as Then<Rhs>>::Then;

            fn bitand(self, rhs: Rhs) -> Self::Output {
                self.then(rhs)
            }
        }

        trait ContextPush<T, P> {
            type StateGet<M>
            where
                Self: ConsGet<Tagged<M, T>, P>;

            fn get<M>(&self) -> Self::StateGet<M>
            where
                Self: ConsGet<Tagged<M, T>, P>;
        }

        impl<T, U, P> ContextPush<U, P> for T {
            type StateGet<M> = CurriedA<Push, StateGet<M, U, P>>

            where
                T: ConsGet<Tagged<M, U>, P>;

            fn get<M>(&self) -> Self::StateGet<M>
            where
                T: ConsGet<Tagged<M, U>, P>,
            {
                Push.curry_a(StateGet::<M, U, P>::default())
            }
        }

        trait ContextRun<F> {
            fn run(&self, f: F) -> CurriedA<Run, F> {
                Run.curry_a(f)
            }
        }

        impl<T, F> ContextRun<F> for T {}

        trait ContextSet<P> {
            fn set<M>(&self) -> Set<M, P> {
                Default::default()
            }
        }

        impl<T, P> ContextSet<P> for T {}

        trait ContextTake<I, P> {
            fn take<M>(&self) -> Take<M, I, P> {
                Default::default()
            }
        }

        impl<T, I, P> ContextTake<I, P> for T {}

        enum Int {}
        enum Float {}
        enum Str {}

        let ctx = (
            Tagged::<Int, i32>::point(1),
            Tagged::<Float, f32>::point(2.0),
            Tagged::<Str, &str>::point("three"),
        )
            .cons();

        let chained = State::<()>::pure(())
            >> ctx.get::<Str>()
            >> ctx.get::<Float>()
            >> ctx.get::<Int>()
            >> ctx.run(Test)
            >> ctx.set::<Float>()
            >> ctx.take::<Float>();

        let chained = chained.copoint().call(ctx);
        assert_eq!(chained, 3.0);
    }
}
