use type_fields_macros::Closure;

use crate::macros::arrow::Fanout;

use crate::t_funk::{ComposeL, Function, Swap};
use crate::{
    macros::{category::category, Copointed, Pointed},
    t_funk::{
        arrow::First, category::Compose, function::Id, Arr, Closure, Curry2, Curry2A, Split, Tuple,
    },
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed, Fanout,
)]
pub struct Circuit<F>(pub F);

impl<F> crate::t_funk::category::Id for Circuit<F> {
    type Id = Circuit<Curry2A<Tuple, Id>>;

    fn id() -> Self::Id {
        Circuit(Tuple.prefix(Id))
    }
}

impl<C1, C2> Compose<Circuit<C2>> for Circuit<C1> {
    type Compose = Circuit<CircuitCompose<C1, C2>>;

    fn compose(self, f: Circuit<C2>) -> Self::Compose {
        Circuit(CircuitCompose(self.0, f.0))
    }
}

#[category]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CircuitCompose<C1, C2>(pub C1, pub C2);

impl<C1, C2, A, C1A, B, C2A, C> Closure<A> for CircuitCompose<C1, C2>
where
    C1: Closure<B, Output = (C2A, C)>,
    C2: Closure<A, Output = (C1A, B)>,
    C2A: Compose<C1A>,
{
    type Output = (C2A::Compose, C);

    fn call(self, a: A) -> Self::Output {
        let (cir1, b) = self.1.call(a);
        let (cir2, c) = self.0.call(b);
        (cir2.compose(cir1), c)
    }
}

impl<T, F> Arr<F> for Circuit<T> {
    type Arr = Circuit<CircuitArr<F>>;

    fn arr(f: F) -> Self::Arr {
        Circuit(CircuitArr(f))
    }
}

#[category]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct CircuitArr<F>(pub F);

impl<F, A> Closure<A> for CircuitArr<F>
where
    F: Closure<A> + Clone,
{
    type Output = (<Circuit<()> as Arr<F>>::Arr, <F as Closure<A>>::Output);

    fn call(self, a: A) -> Self::Output {
        (<Circuit<()> as Arr<F>>::arr(self.0.clone()), self.0.call(a))
    }
}

impl<T> First for Circuit<T> {
    type First = Circuit<CircuitFirst<T>>;

    fn first(self) -> Self::First {
        Circuit(CircuitFirst(self.0))
    }
}

#[category]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct CircuitFirst<CI>(CI);

impl<CI, CA, B, C, D> Closure<(B, D)> for CircuitFirst<CI>
where
    CI: Closure<B, Output = (CA, C)>,
    CA: First,
{
    type Output = (CA::First, (C, D));

    fn call(self, (b, d): (B, D)) -> Self::Output {
        let (ca, c) = self.0.call(b);
        (ca.first(), (c, d))
    }
}

impl<T, F> Split<Circuit<F>> for Circuit<T> {
    type Split = Circuit<
        CircuitCompose<
            CircuitArr<Swap>,
            CircuitCompose<CircuitFirst<F>, CircuitCompose<CircuitArr<Swap>, CircuitFirst<T>>>,
        >,
    >;

    fn split(self, g: Circuit<F>) -> Self::Split {
        self.first()
            .compose_l(<Circuit<()> as Arr<_>>::arr(Swap))
            .compose_l(g.first())
            .compose_l(<Circuit<()> as Arr<_>>::arr(Swap))
    }
}

/// Accumulator that outputs a value determined by the supplied function
#[category]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Accum;

impl<A, F> Function<(A, F)> for Accum {
    type Output = Circuit<AccumImpl<A, F>>;

    fn call((a, f): (A, F)) -> Self::Output {
        Circuit(AccumImpl(a, f))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct AccumImpl<A, F>(A, F);

impl<A, F, I, O, AI> Closure<I> for AccumImpl<A, F>
where
    F: Clone + Closure<(I, A), Output = (O, AI)>,
    Accum: Closure<(AI, F)>,
{
    type Output = (<Accum as Closure<(AI, F)>>::Output, O);

    fn call(self, input: I) -> Self::Output {
        let (out, acci) = self.1.clone().call((input, self.0));
        (Accum.call((acci, self.1)), out)
    }
}

/// Accumulator that outputs the accumulator value
struct AccumOut;

impl<A, F> Function<(A, F)> for AccumOut
where
    Accum: Closure<(A, AccumOutImpl<F>)>,
{
    type Output = <Accum as Closure<(A, AccumOutImpl<F>)>>::Output;

    fn call((a, f): (A, F)) -> Self::Output {
        Accum.call((a, AccumOutImpl(f)))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct AccumOutImpl<F>(F);

impl<F, A, B, B2> Closure<(A, B)> for AccumOutImpl<F>
where
    F: Closure<(A, B), Output = B2>,
    B2: Clone,
{
    type Output = (B2, B2);

    fn call(self, (a, b): (A, B)) -> Self::Output {
        let b = self.0.call((a, b));
        (b.clone(), b)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{Add, ComposeL, Div};

    use crate::t_funk::{function::Const, Arr, Closure, Curry2, Fanout};

    use super::{AccumImpl, AccumOutImpl, Circuit};

    // Concrete accumulator for totaling some numeric type via addition
    pub type Total<T> = Circuit<AccumImpl<T, AccumOutImpl<Add>>>;

    #[test]
    fn test_circuit() {
        // Running total
        let total = Total::<i32>::default();

        let Circuit(cir) = total;
        let (Circuit(cir), a) = cir.call(1);
        assert_eq!(a, 1);

        let (Circuit(cir), b) = cir.call(0);
        assert_eq!(b, 1);

        let (Circuit(cir), c) = cir.call(1);
        assert_eq!(c, 2);

        let (Circuit(cir), d) = cir.call(0);
        assert_eq!(d, 2);

        let (Circuit(cir), d) = cir.call(0);
        assert_eq!(d, 2);

        let (Circuit(_), d) = cir.call(2);
        assert_eq!(d, 4);

        // Statistical mean
        let total = Total::<f64>::default();

        let Circuit(cir) = total
            .fanout(<Circuit<()> as Arr<_>>::arr(Const.prefix(1.0)).compose_l(total))
            .compose_l(<Circuit<()> as Arr<_>>::arr(Div));

        let (Circuit(cir), res) = cir.call(0.0);
        assert_eq!(res, 0.0);

        let (Circuit(cir), res) = cir.call(10.0);
        assert_eq!(res, 5.0);

        let (Circuit(cir), res) = cir.call(7.0);
        assert_eq!(res, 5.6666666666666667);

        let (Circuit(_), res) = cir.call(8.0);
        assert_eq!(res, 6.25);
    }
}
