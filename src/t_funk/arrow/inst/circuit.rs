use core::marker::PhantomData;

use crate::macros::arrow::{Fanout, Split};

use crate::{
    macros::{
        category::{Compose, Id},
        Copointed, Pointed,
    },
    t_funk::{
        arrow::First, category::Compose, function::Id, Arr, Closure, CurriedA, Curry, Pointed,
        Tuple,
    },
};

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Split,
    Fanout,
)]
pub struct Circuit<F>(pub F);

impl<F> crate::t_funk::category::Id for Circuit<F> {
    type Id = Circuit<CurriedA<Tuple, Id>>;

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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Id, Compose)]
pub struct CircuitCompose<C1, C2>(pub C1, pub C2);

impl<C1, C2, A, C1A, B, C2A, C> Closure<A> for CircuitCompose<C1, C2>
where
    C1: Closure<A, Output = (C1A, B)>,
    C2: Closure<B, Output = (C2A, C)>,
    C2A: Compose<C1A>,
{
    type Output = (C2A::Compose, C);

    fn call(self, a: A) -> Self::Output {
        let (cir1, b) = self.0.call(a);
        let (cir2, c) = self.1.call(b);
        (cir2.compose(cir1), c)
    }
}

impl<T, F> Arr<F> for Circuit<T> {
    type Arr = Circuit<CircuitArr<F>>;

    fn arr(f: F) -> Self::Arr {
        Circuit(CircuitArr(f))
    }
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Id,
    Compose,
)]
pub struct CircuitArr<F>(pub F);

impl<F, A> Closure<A> for CircuitArr<F>
where
    F: Arr<A> + Closure<A>,
    A: Clone,
{
    type Output = (<F as Arr<A>>::Arr, <F as Closure<A>>::Output);

    fn call(self, a: A) -> Self::Output {
        (<F as Arr<A>>::arr(a.clone()), self.0.call(a))
    }
}

#[derive(Id, Compose)]
struct CircuitFirst<CI, CA, C>(CI, PhantomData<(CA, C)>);

impl<CI, CA, C> Default for CircuitFirst<CI, CA, C>
where
    CI: Default,
{
    fn default() -> Self {
        CircuitFirst(Default::default(), PhantomData)
    }
}

impl<CI, CA, C> Clone for CircuitFirst<CI, CA, C>
where
    CI: Clone,
{
    fn clone(&self) -> Self {
        CircuitFirst(self.0.clone(), PhantomData)
    }
}

impl<CI, CA, C> Pointed for CircuitFirst<CI, CA, C> {
    type Pointed = CI;

    fn point(unit: Self::Pointed) -> Self {
        CircuitFirst(unit, PhantomData)
    }
}

impl<CI, CA, B, C, D> Closure<(B, D)> for CircuitFirst<CI, CA, C>
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

#[cfg(test)]
mod test {
    use crate::macros::{
        category::{Compose, Id},
        Closure, Copointed, Pointed,
    };

    use crate::t_funk::{
        category::Compose, function::Const, Add, Arr, Closure, Curry, Div, Fanout, Function,
    };

    use super::Circuit;

    /// Accumulator that outputs a value determined by the supplied function
    #[derive(
        Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Id, Compose,
    )]
    struct Accum;

    impl<A, F> Function<(A, F)> for Accum {
        type Output = Circuit<AccumImpl<A, F>>;

        fn call((a, f): (A, F)) -> Self::Output {
            Circuit(AccumImpl(a, f))
        }
    }

    #[derive(
        Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed,
    )]
    struct AccumImpl<A, F>(A, F);

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

    #[derive(
        Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed,
    )]
    struct AccumOutImpl<F>(F);

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

    // Concrete accumulator for totaling some numeric type via addition
    type Total<T> = Circuit<AccumImpl<T, AccumOutImpl<Add>>>;

    #[test]
    fn test_circuit() {
        let total = Total::<i32>::default();

        let Circuit(cir) = total;
        let (Circuit(cir), a) = cir.call(1);
        assert_eq!(a, 1);

        let (Circuit(cir), b) = cir.call(2);
        assert_eq!(b, 3);

        let (Circuit(cir), c) = cir.call(3);
        assert_eq!(c, 6);

        let (Circuit(_cir), d) = cir.call(4);
        assert_eq!(d, 10);

        let mean1 = total.fanout(<Circuit<()> as Arr<_>>::arr(Const.prefix(1.0)).compose(total));
        let Circuit(mean1) = mean1.compose(<Circuit<()> as Arr<_>>::arr(Div));

        //let res = mean1.call(2.0);
        //panic!("{res:?}");
    }
}
