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
pub use uncons::*;

#[cfg(test)]
mod test {
    use core::marker::PhantomData;

    use crate::{
        functional::{Applicative, Copointed, Function, Pointed, Tagged},
        hlist::{
            cons::{ConsFoldRight, Uncons},
            tuple::{Cons, TupleGetImpl},
        },
    };

    #[test]
    fn test_const_functor() {
        struct Get<T, P>(PhantomData<(T, P)>);

        impl<T, P> Default for Get<T, P> {
            fn default() -> Self {
                Get(PhantomData)
            }
        }

        impl<T, P> Clone for Get<T, P> {
            fn clone(&self) -> Self {
                Get(PhantomData)
            }
        }

        impl<T, P, I> Function<I> for Get<T, P>
        where
            I: TupleGetImpl<T, P>,
        {
            type Output = T;

            fn call(self, input: I) -> Self::Output {
                input.get_impl()
            }
        }

        #[derive(Clone)]
        struct Sub;

        impl Function<(i32, i32)> for Sub {
            type Output = i32;

            fn call(self, (a, b): (i32, i32)) -> Self::Output {
                a - b
            }
        }

        impl Function<(i32, f32)> for Sub {
            type Output = f32;

            fn call(self, (a, b): (i32, f32)) -> Self::Output {
                a as f32 - b
            }
        }

        impl Function<(f32, i32)> for Sub {
            type Output = f32;

            fn call(self, (a, b): (f32, i32)) -> Self::Output {
                a - b as f32
            }
        }

        impl Function<(f32, f32)> for Sub {
            type Output = f32;

            fn call(self, (a, b): (f32, f32)) -> Self::Output {
                a - b
            }
        }

        #[derive(Clone)]
        struct Panic;

        impl<T> Function<(T,)> for Panic
        where
            T: core::fmt::Display,
        {
            type Output = ();

            fn call(self, input: (T,)) -> Self::Output {
                panic!("{}", input.0);
            }
        }

        let ctx = ("three", 2.0, 1);

        let list = (
            Action::point(Panic),
            Action::point(Sub),
            Context::point(Get::<i32, _>::default()),
            Action::point(Sub),
            Context::point(Get::<i32, _>::default()),
            Context::point(Get::<f32, _>::default()),
        )
            .cons();

        /// Wrapper denoting an action that should collapse an existing call stack
        type Action<T> = Tagged<TagAction, T>;
        enum TagAction {}

        /// Wrapper denoting an action that modifies context
        enum TagContext {}
        type Context<T> = Tagged<TagContext, T>;

        trait DoTrait {
            type Output;

            fn r#do(self) -> Self::Output;
        }

        impl<C, A, T> DoTrait for ((C, A), Action<T>)
        where
            C: Clone,
            T: Function<A::Uncons>,
            A: Uncons,
        {
            type Output = (C, (<T as Function<A::Uncons>>::Output, ()));

            fn r#do(self) -> Self::Output {
                let ((c, a), n) = self;
                (c, (n.apply(a.uncons()), ()))
            }
        }

        impl<C, A, T> DoTrait for ((C, A), Context<T>)
        where
            C: Clone,
            T: Function<C>,
        {
            type Output = (C, (<T as Function<C>>::Output, A));

            fn r#do(self) -> Self::Output {
                let ((c, a), n) = self;
                (c.clone(), (n.copoint().call(c), a))
            }
        }

        /// DoTrait::do
        #[derive(Clone)]
        struct Do;

        impl<T> Function<T> for Do
        where
            T: DoTrait,
        {
            type Output = T::Output;

            fn call(self, input: T) -> Self::Output {
                input.r#do()
            }
        }

        let (c, res) = list.cons_fold_right((ctx, ()), Do);
        assert_eq!(c, ctx);
        assert_eq!(res, ((), ()));
    }
}
