use crate::{
    derive_closure,
    functional::{ApplyF, CurriedA, Curry, Fmap, Function},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftA2;

impl<F, X> Function<(F, X)> for LiftA2
where
    X: Fmap<F>,
{
    type Output = CurriedA<ApplyF, X::Fmap>;

    fn call((f, x): (F, X)) -> Self::Output {
        ApplyF.curry_a(x.fmap(f))
    }
}

derive_closure!(LiftA2);

#[cfg(test)]
mod test {
    use crate::functional::{Closure, Curry, Just, LiftA2, Pointed, Tuple};

    #[test]
    fn test_lift_a2() {
        let foo = LiftA2
            .call((Tuple.curry(), Just::point(3)))
            .call(Just::point(5));
        assert_eq!(foo, Just::point((3, 5)));
    }
}
