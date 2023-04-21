mod arr;
mod fanout;
mod first;
mod second;
mod split;

pub use arr::*;
pub use fanout::*;
pub use first::*;
pub use second::*;
pub use split::*;

use crate::t_funk::Category;

trait Arrow<Of>: Category<Of> {
    type Arr;
    type First;
    type Second;
    type Split<A>;
    type Fanout<A>;

    fn arr(self) -> <Self as Arr>::Arr
    where
        Self: Arr;

    fn first(self) -> <Self as First>::First
    where
        Self: First;

    fn second(self) -> <Self as Second>::Second
    where
        Self: Second;

    /// ***
    fn split<A>(self, a: A) -> Self::Split<A>
    where
        Self: Split<A>;

    /// &&&
    fn fanout<A>(self, a: A) -> <Self as Fanout<A>>::Fanout
    where
        Self: Fanout<A>;
}

impl<T, Of> Arrow<Of> for T
where
    T: Category<Of>,
{
    type Arr = T::Arr where T: Arr;
    type First = T::First where T: First;
    type Second = T::Second where T: Second;
    type Split<A> = Splitted<T, A> where T: Split<A>;
    type Fanout<A> = T::Fanout where T: Fanout<A>;

    fn arr(self) -> Self::Arr {
        Arr::arr(self)
    }

    fn first(self) -> Self::First {
        First::first(self)
    }

    fn second(self) -> Self::Second {
        Second::second(self)
    }

    fn split<A>(self, a: A) -> Self::Split<A> {
        Split::<A>::split(self, a)
    }

    fn fanout<A>(self, a: A) -> Self::Fanout<A> {
        Fanout::<A>::fanout(self, a)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        arrow::{First, Second},
        Add, Closure, ComposeL, Curry, Fanout, Mul, Split,
    };

    #[test]
    fn test() {
        let a1 = Add.curry_b(5);
        let a2 = Mul.curry_b(2);
        let res = a1.compose_l(a2).call(3);
        assert_eq!(res, 11);

        let q = (1, 2);

        let res = a1.first().call(q);
        assert_eq!(res, (6, 2));

        let res = a1.second().call(q);
        assert_eq!(res, (1, 7));

        let res = a1.split(a2).call(q);
        assert_eq!(res, (6, 4));

        let res = a1.fanout(a2).call(5);
        assert_eq!(res, (10, 10));
    }
}
