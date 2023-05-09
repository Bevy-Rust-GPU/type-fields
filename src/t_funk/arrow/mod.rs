mod arr;
mod fanout;
mod first;
mod inst;
mod second;
mod split;

pub use arr::*;
pub use fanout::*;
pub use first::*;
pub use second::*;
pub use split::*;

use crate::t_funk::Category;

pub trait Arrow: Category {
    type Arr<F>
    where
        Self: Arr<F>;
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

    fn arr<F>(f: F) -> <Self as Arr<F>>::Arr
    where
        Self: Arr<F>;

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
    type Arr<F> = T::Arr where T: Arr<F>;
    type First = T::First where T: First;
    type Second = T::Second where T: Second;
    type Splitted<A> = Splitted<T, A> where T: Split<A>;
    type Fanouted<A> = T::Fanout where T: Fanout<A>;

    fn arr<F>(f: F) -> T::Arr
    where
        T: Arr<F>,
    {
        <T as Arr<F>>::arr(f)
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
        arrow::First, arrow::Second, closure::Compose, Add, Closure, Curry2, Fanout, Mul, Split,
    };

    #[test]
    fn test_arrow() {
        let a1 = Add.suffix(5);
        let a2 = Mul.suffix(2);

        let res = a1.compose(a2).call(3);
        assert_eq!(res, 11);

        let q = (1, 2);

        let res = a1.first().call(q);
        assert_eq!(res, (6, 2));

        let res = a1.second().call(q);
        assert_eq!(res, (1, 7));

        let res = a1.split(a2).call(q);
        assert_eq!(res, (6, 4));

        let res = a1.fanout(a2).call(6);
        assert_eq!(res, (11, 12));
    }
}
