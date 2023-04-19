//! Heterogeneous List (or 'Cons List')
//! Useful for recursive trait evaluation.

mod macros;

mod append;
mod applicative;
mod as_mut;
mod as_ref;
mod clone;
mod fold_left;
mod fold_right;
mod functor;
mod get;
mod gets;
mod hlist;
mod hlist_mut;
mod hlist_ref;
mod length;
mod monad;
mod monoid;
mod path;
mod pop_back;
mod pop_front;
mod push_back;
mod push_front;
mod remove;
mod semigroup;
mod set;
mod sets;
mod to_tlist;
mod traversable;

pub use append::*;
pub use applicative::*;
pub use as_mut::*;
pub use as_ref::*;
pub use clone::*;
pub use fold_left::*;
pub use fold_right::*;
pub use functor::*;
pub use get::*;
pub use gets::*;
pub use hlist::*;
pub use hlist_mut::*;
pub use hlist_ref::*;
pub use length::*;
pub use path::*;
pub use pop_back::*;
pub use pop_front::*;
pub use push_back::*;
pub use push_front::*;
pub use remove::*;
pub use semigroup::*;
pub use set::*;
pub use sets::*;
pub use to_tlist::*;
pub use traversable::*;

#[cfg(test)]
mod do_notation_monadic {
    use core::{
        marker::PhantomData,
        ops::{BitAnd, Shr},
    };

    use type_fields_macros::Closure;

    use crate::t_funk::{
        hlist::ToTList, tlist::ToHList, Chain, Closure, Compose, Composed, Copointed, CurriedA,
        Curry, FirstF, Flip, Flipped, Function, Pointed, Pure, Spread, Spreaded, State, Tagged,
        Then, Tuple,
    };

    #[test]
    fn test_monadic_do() {
        #[derive(Closure)]
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
            C: crate::t_funk::hlist::Get<Tagged<M, T>, P>,
        {
            type Output = T;

            fn call(ctx: C) -> Self::Output {
                ctx.get().copoint()
            }
        }

        #[derive(Closure)]
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
            C: crate::t_funk::hlist::Set<Tagged<U, T>, P>,
        {
            type Output = ((), C);

            fn call((t, ctx): (T, C)) -> Self::Output {
                ((), ctx.set(Tagged::point(t)))
            }
        }

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
        struct Test;

        impl Function<(i32, f32, &str)> for Test {
            type Output = f32;

            fn call((a, b, _): (i32, f32, &str)) -> Self::Output {
                a as f32 + b
            }
        }

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
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

        #[derive(Closure)]
        struct Run;

        impl<F, I> Function<(F, I)> for Run
        where
            F: Closure<I::TList>,
            I: ToTList,
        {
            type Output = State<CurriedA<Tuple, F::Output>>;

            fn call((f, i): (F, I)) -> Self::Output {
                State::point(Tuple.curry_a(f.call(i.to_tlist())))
            }
        }

        #[derive(Closure)]
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

        #[derive(Closure)]
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
                Self: crate::t_funk::hlist::Get<Tagged<M, T>, P>;

            fn get<M>(&self) -> Self::StateGet<M>
            where
                Self: crate::t_funk::hlist::Get<Tagged<M, T>, P>;
        }

        impl<T, U, P> ContextPush<U, P> for T {
            type StateGet<M> = CurriedA<Push, StateGet<M, U, P>>

            where
                T: crate::t_funk::hlist::Get<Tagged<M, U>, P>;

            fn get<M>(&self) -> Self::StateGet<M>
            where
                T: crate::t_funk::hlist::Get<Tagged<M, U>, P>,
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
            .to_hlist();

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