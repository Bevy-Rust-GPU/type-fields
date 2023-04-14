use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_pointed,
    functional::{Applicative, Const, CurriedA, Curry, Flip, Flipped, Function, Pointed},
};

use super::Monad;

/// 2-tuple constructor
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tuple;

impl<A, B> Function<(A, B)> for Tuple {
    type Output = (A, B);

    fn call(self, input: (A, B)) -> Self::Output {
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
        State(StateChain(self).flip().curry().call(f))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MakeState;

impl<I> Function<I> for MakeState {
    type Output = State<CurriedA<Tuple, I>>;

    fn call(self, input: I) -> Self::Output {
        State::point(Tuple.curry().call(input))
    }
}

/// TODO: This should be replaced with three-argument currying
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateChain<T>(State<T>);

impl<T, F, S, A, S2, F2> Function<(S, F)> for StateChain<T>
where
    T: Function<S, Output = (A, S2)>,
    F: Function<A, Output = State<F2>>,
    F2: Function<S2>,
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

    fn call(self, input: I) -> Self::Output {
        State::point(Const.curry().call(((), input)))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Get;

impl<I> Function<I> for Get
where
    I: Clone,
{
    type Output = State<CurriedA<Const, (I, I)>>;

    fn call(self, input: I) -> Self::Output {
        State::point(Const.curry().call((input.clone(), input)))
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{Applicative, Const, CurryN, Function, Monad, Pointed, Put, State};

    #[test]
    fn test_state() {
        struct Random;

        impl Function<i32> for Random {
            type Output = (i32, i32);

            fn call(self, input: i32) -> Self::Output {
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

            fn call(self, _: I) -> Self::Output {
                (Thank, Unlocked)
            }
        }

        impl Function<Locked> for Push {
            type Output = (Tut, Locked);

            fn call(self, _: Locked) -> Self::Output {
                (Tut, Locked)
            }
        }

        impl Function<Unlocked> for Push {
            type Output = (Open, Locked);

            fn call(self, _: Unlocked) -> Self::Output {
                (Open, Locked)
            }
        }

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
