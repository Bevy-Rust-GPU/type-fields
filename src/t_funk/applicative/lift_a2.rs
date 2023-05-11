use crate::macros::{arrow::arrow, category::category, Closure};

use crate::t_funk::{Apply, Fmap, Function};

/// Lift a binary function to actions
#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct LiftA2;

impl<F, A, B> Function<(F, A, B)> for LiftA2
where
    A: Fmap<F>,
    A::Fmap: Apply<B>,
{
    type Output = <A::Fmap as Apply<B>>::Apply;

    fn call((f, a, b): (F, A, B)) -> Self::Output {
        a.fmap(f).apply(b)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{Closure, Curry2, Just, LiftA2, Tuple};

    #[test]
    fn test_lift_a2() {
        let foo = LiftA2.call((Tuple.curry(), Just(3), Just(5)));
        assert_eq!(foo, Just((3, 5)));
    }
}
