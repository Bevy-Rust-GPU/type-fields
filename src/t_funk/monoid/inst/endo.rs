use type_fields_macros::{Copointed, Pointed};

use crate::t_funk::{
    Apply, Closure, Compose, Composed, Copointed, Fmap, Id, Mappend, Mempty, Pointed, Pure,
};

/// The monoid of endomorphisms under composition.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Endo<T>(T);

impl<F1, F2> Fmap<F2> for Endo<F1> {
    type Fmap = Endo<Composed<F1, F2>>;

    fn fmap(self, f: F2) -> Self::Fmap {
        Endo(self.copoint().compose(f))
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
        Endo::point(t)
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
        let a = self.0.copoint().call(input.clone());
        let b = self.1.copoint().call(input);
        a.call(b)
    }
}

impl<T> Mempty for Endo<T>
where
    T: Mempty,
{
    type Mempty = Endo<Id>;

    fn mempty() -> Self::Mempty {
        Endo::point(Id)
    }
}

impl<T, U> Mappend<Endo<U>> for Endo<T> {
    type Mappend = Endo<Composed<T, U>>;

    fn mappend(self, u: Endo<U>) -> Self::Mappend {
        Endo::point(Composed::point((self.copoint(), u.copoint())))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        t_funk::tlist::ToHList,
        t_funk::{
            Add, Closure, Compose, Composed, Copointed, CurriedN, CurryN, Foldr, Mappend, PointF,
            Pointed,
        },
    };

    use super::Endo;

    #[test]
    fn test_endo() {
        let foo = Endo::point(Add.curry_n().call(1));
        let foo = foo.mappend(Endo::point(Add.curry_n().call(2)));
        let foo = foo.mappend(Endo::point(Add.curry_n().call(3)));
        let bar = foo.copoint();
        let baz = bar.call(4);
        assert_eq!(baz, 10);
    }

    #[test]
    fn test_foldr() {
        let list = (1, 2, 3).to_hlist();
        let folded = list.foldr(Add.curry_n(), 0);
        assert_eq!(folded, 6);
    }

    #[test]
    fn test_composition() {
        let add: Add = Add;
        let add_result: i32 = add.call((1, 2));
        assert_eq!(add_result, 3);

        let endo = PointF::default();
        let _endo_result: Endo<Add> = endo.call(add);

        let add_endo: Composed<Add, PointF<Endo<i32>>> =
            add.compose(PointF::<Endo<i32>>::default());
        let add_endo_result: Endo<i32> = add_endo.call((1, 2));
        assert_eq!(add_endo_result.copoint(), 3);

        let add_curry_a: CurriedN<Add, (), _> = Add.curry_n();
        let add_curry_b: CurriedN<Add, (i32, ()), _> = add_curry_a.call(1);
        let add_curry_result: i32 = add_curry_b.call(1);
        assert_eq!(add_curry_result, 2);

        let add_curry_endo_a: Composed<
            CurriedN<Add, (), _>,
            PointF<Endo<<CurriedN<Add, (), _> as Closure<i32>>::Output>>,
        > = Add.curry_n().compose(PointF::default());
        let add_curry_endo_b: Endo<CurriedN<Add, (i32, ()), _>> = add_curry_endo_a.call(1);
        let add_curry_endo_result: i32 = add_curry_endo_b.copoint().call(2);
        assert_eq!(add_curry_endo_result, 3);
    }
}
