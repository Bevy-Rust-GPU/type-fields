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

trait Arrow: Category {
    type Arr;
    type First
    where
        Self: First;
    type Second
    where
        Self: Second;
    type Splitted<A>
    where
        Self: Split<A>;
    type Fanouted<A>
    where
        Self: Fanout<A>;

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
    fn split<A>(self, a: A) -> Self::Split
    where
        Self: Split<A>;

    /// &&&
    fn fanout<A>(self, a: A) -> <Self as Fanout<A>>::Fanout
    where
        Self: Fanout<A>;
}

impl<T> Arrow for T
where
    T: Category,
{
    type Arr = T::Arr where T: Arr;
    type First = T::First where T: First;
    type Second = T::Second where T: Second;
    type Splitted<A> = Splitted<T, A> where T: Split<A>;
    type Fanouted<A> = T::Fanout where T: Fanout<A>;

    fn arr(self) -> T::Arr
    where
        T: Arr,
    {
        Arr::arr(self)
    }

    fn first(self) -> T::First
    where
        T: First,
    {
        First::first(self)
    }

    fn second(self) -> T::Second
    where
        T: Second,
    {
        Second::second(self)
    }

    fn split<A>(self, a: A) -> T::Split
    where
        T: Split<A>,
    {
        Split::<A>::split(self, a)
    }

    fn fanout<A>(self, a: A) -> T::Fanout
    where
        T: Fanout<A>,
    {
        Fanout::<A>::fanout(self, a)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        arrow::First, arrow::Second, Add, Closure, ComposeL, Curry, Fanout, Mul, Split,
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
