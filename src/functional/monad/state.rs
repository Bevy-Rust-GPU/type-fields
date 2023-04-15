use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_pointed,
    functional::{Applicative, Closure, Const, CurriedA, Curry, Flip, Flipped, Function, Pointed}, derive_closure,
};

use super::Monad;

/// 2-tuple constructor
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tuple;

impl<A, B> Function<(A, B)> for Tuple {
    type Output = (A, B);

    fn call(input: (A, B)) -> Self::Output {
        input
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State<F>(F);

derive_pointed!(State<F>);
derive_copointed!(State<F>);
derive_functor!(State<F>);
derive_applicative!(State<F>);

impl<T, F> Monad<F> for State<T> {
    type Chained = State<CurriedA<Flipped<StateChain<T>>, F>>;

    fn chain(self, f: F) -> Self::Chained {
        State(StateChain(self).flip().curry_a(f))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MakeState;

impl<I> Function<I> for MakeState {
    type Output = State<CurriedA<Tuple, I>>;

    fn call(input: I) -> Self::Output {
        State::point(Tuple.curry_a(input))
    }
}

/// TODO: This should be replaced with three-argument currying
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateChain<T>(State<T>);

impl<T, F, S, A, S2, F2> Closure<(S, F)> for StateChain<T>
where
    T: Closure<S, Output = (A, S2)>,
    F: Closure<A, Output = State<F2>>,
    F2: Closure<S2>,
{
    type Output = F2::Output;

    fn call(self, (s, f): (S, F)) -> Self::Output {
        let (a, _s) = self.0.apply(s);
        f.call(a).apply(_s)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Put;

impl<I> Function<I> for Put {
    type Output = State<CurriedA<Const, ((), I)>>;

    fn call(input: I) -> Self::Output {
        State::point(Const.curry_a(((), input)))
    }
}

derive_closure!(Put);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Get;

impl<I> Function<I> for Get
where
    I: Clone,
{
    type Output = State<CurriedA<Const, (I, I)>>;

    fn call(input: I) -> Self::Output {
        State::point(Const.curry_a((input.clone(), input)))
    }
}

derive_closure!(Get);

#[cfg(test)]
mod test {
    use crate::{
        derive_closure,
        functional::{
            Applicative, Closure, Const, Curry, CurryN, Flip, Function, Functor, Id, Monad,
            Pointed, Put, State,
        },
        hlist::{cons::PushFront, tuple::Cons},
    };

    #[test]
    fn test_state() {
        struct Random;

        impl Function<i32> for Random {
            type Output = (i32, i32);

            fn call(input: i32) -> Self::Output {
                let next_seed =
                    (1839567234_i32.wrapping_mul(input).wrapping_add(972348567)) % 823945102;
                (next_seed, next_seed)
            }
        }

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Push;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Locked;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Unlocked;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Thank;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Open;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Tut;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Coin;

        impl<I> Function<I> for Coin {
            type Output = (Thank, Unlocked);

            fn call(_: I) -> Self::Output {
                (Thank, Unlocked)
            }
        }

        derive_closure!(Coin);

        impl Function<Locked> for Push {
            type Output = (Tut, Locked);

            fn call(_: Locked) -> Self::Output {
                (Tut, Locked)
            }
        }

        impl Function<Unlocked> for Push {
            type Output = (Open, Locked);

            fn call(_: Unlocked) -> Self::Output {
                (Open, Locked)
            }
        }

        derive_closure!(Push);

        let coin_s = State::point(Coin);
        let push_s = State::point(Push);

        let arr = coin_s.apply(Locked);
        assert_eq!(arr, (Thank, Unlocked));

        let arr = coin_s.chain(Const.curry_n().call(push_s)).apply(Locked);
        assert_eq!(arr, (Open, Locked));

        // Chaining
        let arr = coin_s
            .chain(Const.curry_n().call(push_s))
            .chain(Const.curry_n().call(push_s))
            .chain(Const.curry_n().call(coin_s))
            .chain(Const.curry_n().call(push_s))
            .apply(Locked);
        assert_eq!(arr, (Open, Locked));

        // FIXME:
        // Inlined sequence_a() call is using list semantics instead of State ones
        let monday_s = (coin_s, push_s, push_s, coin_s, push_s).cons();
        let _monday_s: (Coin, (Push, (Push, (Coin, (Push, ((), ())))))) = Id
            .clone()
            .call(monday_s.0)
            .fmap(PushFront.flip().curry())
            .apply({
                let this = monday_s.1;
                Id.call(this.0).fmap(PushFront.flip().curry()).apply({
                    let this = this.1;
                    Id.call(this.0).fmap(PushFront.flip().curry()).apply({
                        let this = this.1;
                        Id.call(this.0).fmap(PushFront.flip().curry()).apply({
                            let this = this.1;
                            let foo = Id
                                .call(this.0)
                                .fmap(PushFront.flip().curry())
                                .apply(((), ()));
                            foo
                        })
                    })
                })
            });

        // Put
        let test = Put.call(Locked).chain(Const.curry_n().call(push_s));
        let check1 = test.apply(Unlocked);
        assert_eq!(check1, (Tut, Locked));
        let test = test
            .chain(Const.curry_n().call(Put.call(Unlocked)))
            .chain(Const.curry_n().call(push_s));
        let check2 = test.apply(Unlocked);
        assert_eq!(check2, (Open, Locked));
    }
}
