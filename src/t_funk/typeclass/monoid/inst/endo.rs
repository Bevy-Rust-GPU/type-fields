use crate::macros::{Copointed, Pointed};

use crate::t_funk::{
    closure::Compose, function::Id, Apply, Closure, Composed, Fmap, Mappend, Mempty, applicative::Pure,
};

/// The monoid of endomorphisms under composition.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Endo<T>(pub T);

impl<F1, F2> Fmap<F2> for Endo<F1>
where
    F1: Compose<F2>,
{
    type Fmap = Endo<<F1 as Compose<F2>>::Compose>;

    fn fmap(self, f: F2) -> Self::Fmap {
        Endo(self.0.compose(f))
    }
}

impl<F1, F2> Apply<Endo<F2>> for Endo<F1> {
    type Apply = Endo<EndoApplicative<F1, F2>>;

    fn apply(self, f: Endo<F2>) -> Self::Apply {
        Endo(EndoApplicative(self, f))
    }
}

impl<T> Pure for Endo<T> {
    type Pure<U> = Endo<U>;

    fn pure<U>(t: U) -> Self::Pure<U> {
        Endo(t)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndoApplicative<F1, F2>(Endo<F1>, Endo<F2>);

impl<F1, F2, T> Closure<T> for EndoApplicative<F1, F2>
where
    F1: Closure<T>,
    F2: Closure<T>,
    F1::Output: Closure<F2::Output>,
    T: Clone,
{
    type Output = <F1::Output as Closure<F2::Output>>::Output;

    fn call(self, input: T) -> Self::Output {
        let a = self.0 .0.call(input.clone());
        let b = self.1 .0.call(input);
        a.call(b)
    }
}

impl<T> Mempty for Endo<T>
where
    T: Mempty,
{
    type Mempty = Endo<Id>;

    fn mempty() -> Self::Mempty {
        Endo(Id)
    }
}

impl<T, U> Mappend<Endo<U>> for Endo<T> {
    type Mappend = Endo<Composed<T, U>>;

    fn mappend(self, u: Endo<U>) -> Self::Mappend {
        Endo(self.0.compose(u.0))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        t_funk::tlist::ToHList,
        t_funk::{
            closure::Compose, Add, Closure, Composed, Curried2, Curry2A, Curry2, Foldr, Mappend,
            PointF,
        },
    };

    use super::Endo;

    #[test]
    fn test_endo() {
        let foo = Endo(Add.prefix2(1));
        let foo = foo.mappend(Endo(Add.prefix2(2)));
        let Endo(foo) = foo.mappend(Endo(Add.prefix2(3)));
        let foo = foo.call(4);
        assert_eq!(foo, 10);
    }

    #[test]
    fn test_foldr() {
        let list = (1, 2, 3).to_hlist();
        let folded = list.foldr(Add, 0);
        assert_eq!(folded, 6);
    }

    #[test]
    fn test_composition() {
        let add_result: i32 = Add.call((1, 2));
        assert_eq!(add_result, 3);

        let endo = PointF::default();
        let _endo_result: Endo<Add> = endo.call(Add);

        let add_endo: Composed<PointF<Endo<i32>>, Add> =
            PointF::<Endo<i32>>::default().compose(Add);
        let add_endo_result: Endo<i32> = add_endo.call((1, 2));
        assert_eq!(add_endo_result, Endo(3));

        let add_curry_a: Curried2<Add> = Add.curry2();
        let add_curry_b: Curry2A<Add, i32> = add_curry_a.call(1);
        let add_curry_result: i32 = add_curry_b.call(1);
        assert_eq!(add_curry_result, 2);

        let add_curry_endo_a: Composed<
            PointF<Endo<<Curried2<Add> as Closure<i32>>::Output>>,
            Curried2<Add>,
        > = PointF::default().compose(Add.curry2());
        let Endo(add_curry_endo_b): Endo<Curry2A<Add, i32>> = add_curry_endo_a.call(1);
        let add_curry_endo_result: i32 = add_curry_endo_b.call(2);
        assert_eq!(add_curry_endo_result, 3);
    }
}
