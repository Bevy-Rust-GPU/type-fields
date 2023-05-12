use crate::t_funk::{
    applicative::Pure,
    list::hlist::{Cons, Nil},
    Apply, Fmap, Mappend,
};

impl<Head, Tail, U> Apply<U> for Cons<Head, Tail>
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

impl<T> Apply<T> for Nil {
    type Apply = Nil;

    fn apply(self, _: T) -> Self::Apply {
        self
    }
}

impl<T, N> Pure for Cons<T, N>
where
    N: Pure,
{
    type Pure<U> = Cons<U, Nil>;

    fn pure<U>(unit: U) -> Self::Pure<U> {
        Cons(unit, Nil)
    }
}

impl Pure for Nil {
    type Pure<T> = Cons<T, Nil>;

    fn pure<T>(unit: T) -> Self::Pure<T> {
        Cons(unit, Nil)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        hlist::{Cons, Nil},
        Add, Apply, Curry2, Mul,
    };

    #[test]
    fn test_hlist_applicative() {
        let funcs = Cons(Add.suffix2(2), Cons(Mul.suffix2(2), Nil));
        let nums = Cons(1, Cons(2, Cons(3, Nil)));
        let res = funcs.apply(nums);
        assert_eq!(
            res,
            Cons(3, Cons(4, Cons(5, Cons(2, Cons(4, Cons(6, Nil))))))
        );
    }
}
