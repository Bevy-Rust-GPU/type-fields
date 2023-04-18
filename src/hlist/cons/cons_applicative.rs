use crate::functional::{Applicative, Functor, Pure, Semigroup};

impl<Head, Tail, U> Applicative<U> for (Head, Tail)
where
    Tail: Applicative<U>,
    U: Clone + Functor<Head>,
    U::Mapped: Semigroup<Tail::Applied>,
{
    type Applied = <U::Mapped as Semigroup<<Tail as Applicative<U>>::Applied>>::Appended;

    fn apply(self, a: U) -> Self::Applied {
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
    use crate::functional::{Add, Applicative, Curry, Flip, Mul};

    #[test]
    fn test_cons_applicative() {
        let funcs = (Add.flip().curry_a(2), (Mul.flip().curry_a(2), ()));
        let nums = (1, (2, (3, ())));
        let res = funcs.apply(nums);
        assert_eq!(res, (3, (4, (5, (2, (4, (6, ())))))));
    }
}
