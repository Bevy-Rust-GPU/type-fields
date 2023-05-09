use crate::t_funk::{Apply, Fmap, Mappend, applicative::Pure};

impl<Head, Tail, U> Apply<U> for (Head, Tail)
where
    Tail: Apply<U>,
    U: Clone + Fmap<Head>,
    U::Fmap: Mappend<Tail::Apply>,
{
    type Apply = <U::Fmap as Mappend<<Tail as Apply<U>>::Apply>>::Mappend;

    fn apply(self, a: U) -> Self::Apply {
        a.clone().fmap(self.0).mappend(self.1.apply(a))
    }
}

impl<Head, Tail> Pure for (Head, Tail)
where
    Tail: Pure,
{
    type Pure<T> = (T, ());

    fn pure<T>(unit: T) -> Self::Pure<T> {
        (unit, ())
    }
}

impl Pure for () {
    type Pure<T> = (T, ());

    fn pure<T>(unit: T) -> Self::Pure<T> {
        (unit, ())
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{Add, Apply, Curry2, Mul};

    #[test]
    fn test_hlist_applicative() {
        let funcs = (Add.suffix(2), (Mul.suffix(2), ()));
        let nums = (1, (2, (3, ())));
        let res = funcs.apply(nums);
        assert_eq!(res, (3, (4, (5, (2, (4, (6, ())))))));
    }
}
