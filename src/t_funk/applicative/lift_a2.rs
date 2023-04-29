use crate::macros::Closure;

use crate::t_funk::{ApplyF, Prefixed, Curry, Fmap, Function};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct LiftA2;

impl<F, X> Function<(F, X)> for LiftA2
where
    X: Fmap<F>,
{
    type Output = Prefixed<ApplyF, X::Fmap>;

    fn call((f, x): (F, X)) -> Self::Output {
        ApplyF.prefix(x.fmap(f))
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{Closure, Curry, Just, LiftA2, Tuple};

    #[test]
    fn test_lift_a2() {
        let foo = LiftA2.call((Tuple.curry(), Just(3))).call(Just(5));
        assert_eq!(foo, Just((3, 5)));
    }
}
