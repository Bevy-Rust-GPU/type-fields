use core::marker::PhantomData;

use crate::t_funk::{
    arrow::{First, Second},
    category::{Compose, Id},
    function::Id as IdF,
    hlist::Nil,
    Arr, Closure, ComposeL, Copointed, Fanout, MakePair, Mappend, Pointed, Split,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pure<F>(pub F);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Effect<F1>(pub F1);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seq<F1, F2>(pub F1, pub F2);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Par<F1, F2>(pub F1, pub F2);

impl<F> Id for Pure<F> {
    type Id = Pure<IdF>;

    fn id() -> Self::Id {
        Pure(IdF)
    }
}

impl<F> Id for Effect<F> {
    type Id = Pure<crate::t_funk::function::Id>;

    fn id() -> Self::Id {
        Pure(IdF)
    }
}

impl<F1, F2> Id for Seq<F1, F2> {
    type Id = Pure<crate::t_funk::function::Id>;

    fn id() -> Self::Id {
        Pure(IdF)
    }
}

impl<F1, F2> Id for Par<F1, F2> {
    type Id = Pure<crate::t_funk::function::Id>;

    fn id() -> Self::Id {
        Pure(IdF)
    }
}

impl<F1, F2> Compose<F2> for Pure<F1> {
    type Compose = Seq<F2, Pure<F1>>;

    fn compose(self, f: F2) -> Self::Compose {
        Seq(f, self)
    }
}

impl<F1, F2> Compose<F2> for Effect<F1> {
    type Compose = Seq<F2, Effect<F1>>;

    fn compose(self, f: F2) -> Self::Compose {
        Seq(f, self)
    }
}

impl<F1, F2, F3> Compose<F3> for Seq<F1, F2> {
    type Compose = Seq<F3, Seq<F1, F2>>;

    fn compose(self, f: F3) -> Self::Compose {
        Seq(f, self)
    }
}

impl<F1, F2, F3> Compose<F3> for Par<F1, F2> {
    type Compose = Seq<F3, Par<F1, F2>>;

    fn compose(self, f: F3) -> Self::Compose {
        Seq(f, self)
    }
}

impl<F1, F2> Arr<F2> for Pure<F1> {
    type Arr = Pure<F2>;

    fn arr(f: F2) -> Self::Arr {
        Pure(f)
    }
}

impl<F1, F2> Arr<F2> for Effect<F1> {
    type Arr = Pure<F2>;

    fn arr(f: F2) -> Self::Arr {
        Pure(f)
    }
}

impl<F1, F2, F3> Arr<F3> for Seq<F1, F2> {
    type Arr = Pure<F3>;

    fn arr(f: F3) -> Self::Arr {
        Pure(f)
    }
}

impl<F1, F2, F3> Arr<F3> for Par<F1, F2> {
    type Arr = Pure<F3>;

    fn arr(f: F3) -> Self::Arr {
        Pure(f)
    }
}

impl<F1> First for Pure<F1> {
    type First = Par<Pure<F1>, IdF>;

    fn first(self) -> Self::First {
        Par(self, IdF)
    }
}

impl<F1> First for Effect<F1> {
    type First = Par<Effect<F1>, IdF>;

    fn first(self) -> Self::First {
        Par(self, IdF)
    }
}

impl<F1, F2> First for Seq<F1, F2> {
    type First = Par<Seq<F1, F2>, IdF>;

    fn first(self) -> Self::First {
        Par(self, IdF)
    }
}

impl<F1, F2> First for Par<F1, F2> {
    type First = Par<Par<F1, F2>, IdF>;

    fn first(self) -> Self::First {
        Par(self, IdF)
    }
}

impl<F1> Second for Pure<F1> {
    type Second = Par<IdF, Pure<F1>>;

    fn second(self) -> Self::Second {
        Par(IdF, self)
    }
}

impl<F1> Second for Effect<F1> {
    type Second = Par<IdF, Effect<F1>>;

    fn second(self) -> Self::Second {
        Par(IdF, self)
    }
}

impl<F1, F2> Second for Seq<F1, F2> {
    type Second = Par<IdF, Seq<F1, F2>>;

    fn second(self) -> Self::Second {
        Par(IdF, self)
    }
}

impl<F1, F2> Second for Par<F1, F2> {
    type Second = Par<IdF, Par<F1, F2>>;

    fn second(self) -> Self::Second {
        Par(IdF, self)
    }
}

impl<F1, F2> Split<F2> for Pure<F1> {
    type Split = Par<Pure<F1>, F2>;

    fn split(self, g: F2) -> Self::Split {
        Par(self, g)
    }
}

impl<F1, F2> Split<F2> for Effect<F1> {
    type Split = Par<Effect<F1>, F2>;

    fn split(self, g: F2) -> Self::Split {
        Par(self, g)
    }
}

impl<F1, F2, F3> Split<F3> for Seq<F1, F2> {
    type Split = Par<Seq<F1, F2>, F3>;

    fn split(self, g: F3) -> Self::Split {
        Par(self, g)
    }
}

impl<F1, F2, F3> Split<F3> for Par<F1, F2> {
    type Split = Par<Par<F1, F2>, F3>;

    fn split(self, g: F3) -> Self::Split {
        Par(self, g)
    }
}

impl<F1, F2> Fanout<F2> for Pure<F1> {
    type Fanout = Seq<Pure<MakePair>, Par<Pure<F1>, F2>>;

    fn fanout(self, f: F2) -> Self::Fanout {
        Self::arr(MakePair).compose_l(self.split(f))
    }
}

impl<F1, F2> Fanout<F2> for Effect<F1> {
    type Fanout = Seq<Pure<MakePair>, Par<Effect<F1>, F2>>;

    fn fanout(self, f: F2) -> Self::Fanout {
        Self::arr(MakePair).compose_l(self.split(f))
    }
}

impl<F1, F2, F3> Fanout<F3> for Seq<F1, F2> {
    type Fanout = Seq<Pure<MakePair>, Par<Seq<F1, F2>, F3>>;

    fn fanout(self, f: F3) -> Self::Fanout {
        Self::arr(MakePair).compose_l(self.split(f))
    }
}

impl<F1, F2, F3> Fanout<F3> for Par<F1, F2> {
    type Fanout = Seq<Pure<MakePair>, Par<Par<F1, F2>, F3>>;

    fn fanout(self, f: F3) -> Self::Fanout {
        Self::arr(MakePair).compose_l(self.split(f))
    }
}

/// Traverses a free arrow construct,
/// folding its contents using `Mappend`
pub struct Analyze<F, A>(F, PhantomData<A>);

impl<F, A> Pointed for Analyze<F, A> {
    type Pointed = F;

    fn point(unit: Self::Pointed) -> Self {
        Analyze(unit, PhantomData)
    }
}

impl<F, A> Copointed for Analyze<F, A> {
    type Copointed = F;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<F, A> Default for Analyze<F, A>
where
    F: Default,
{
    fn default() -> Self {
        Analyze(F::default(), PhantomData)
    }
}

impl<F, A> Clone for Analyze<F, A>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Analyze(self.0.clone(), PhantomData)
    }
}

impl<F, A, P> Closure<Pure<P>> for Analyze<F, A> {
    type Output = Nil; //<MemptyF<A> as crate::t_funk::applicative::Pure>::Pure<A>;

    fn call(self, _: Pure<P>) -> Self::Output {
        Nil
        //<MemptyF<A> as crate::t_funk::applicative::Pure>::pure(Default::default())
    }
}

impl<F, A, E> Closure<Effect<E>> for Analyze<F, A>
where
    F: Closure<E>,
{
    type Output = F::Output;

    fn call(self, Effect(g): Effect<E>) -> Self::Output {
        self.0.call(g)
    }
}

impl<F, A, S1, S2> Closure<Seq<S1, S2>> for Analyze<F, A>
where
    Analyze<F, A>: Clone + Closure<S1> + Closure<S2>,
    <Analyze<F, A> as Closure<S1>>::Output: Mappend<<Analyze<F, A> as Closure<S2>>::Output>,
{
    type Output = <<Analyze<F, A> as Closure<S1>>::Output as Mappend<
        <Analyze<F, A> as Closure<S2>>::Output,
    >>::Mappend;

    fn call(self, Seq(s1, s2): Seq<S1, S2>) -> Self::Output {
        let fg = self.clone().call(s1);
        let fh = self.call(s2);

        fg.mappend(fh)
    }
}

impl<F, A, P1, P2> Closure<Par<P1, P2>> for Analyze<F, A>
where
    Analyze<F, A>: Clone + Closure<P1> + Closure<P2>,
    <Analyze<F, A> as Closure<P1>>::Output: Mappend<<Analyze<F, A> as Closure<P2>>::Output>,
{
    type Output = <<Analyze<F, A> as Closure<P1>>::Output as Mappend<
        <Analyze<F, A> as Closure<P2>>::Output,
    >>::Mappend;

    fn call(self, Par(p1, p2): Par<P1, P2>) -> Self::Output {
        let fg = self.clone().call(p1);
        let fh = self.call(p2);

        fg.mappend(fh)
    }
}

/// Traverses a free arrow construct,
/// ...
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvalA<F>(pub F);

impl<F, G> Closure<Pure<G>> for EvalA<F> {
    type Output = G;

    fn call(self, Pure(g): Pure<G>) -> Self::Output {
        //<A as Arr<G>>::arr(g)
        g
    }
}

impl<F, G> Closure<Effect<G>> for EvalA<F>
where
    F: Closure<G>,
{
    type Output = F::Output;

    fn call(self, Effect(g): Effect<G>) -> Self::Output {
        self.0.call(g)
    }
}

impl<F, G, H> Closure<Seq<G, H>> for EvalA<F>
where
    EvalA<F>: Clone + Closure<G> + Closure<H>,
    <EvalA<F> as Closure<H>>::Output: Compose<<EvalA<F> as Closure<G>>::Output>,
{
    type Output =
        <<EvalA<F> as Closure<H>>::Output as Compose<<EvalA<F> as Closure<G>>::Output>>::Compose;

    fn call(self, Seq(g, h): Seq<G, H>) -> Self::Output {
        let fg = self.clone().call(g);
        let fh = self.call(h);
        fh.compose(fg)
    }
}

impl<F, G, H> Closure<Par<G, H>> for EvalA<F>
where
    EvalA<F>: Clone + Closure<G> + Closure<H>,
    <EvalA<F> as Closure<G>>::Output: Split<<EvalA<F> as Closure<H>>::Output>,
{
    type Output =
        <<EvalA<F> as Closure<G>>::Output as Split<<EvalA<F> as Closure<H>>::Output>>::Split;

    fn call(self, Par(g, h): Par<G, H>) -> Self::Output {
        let fg = self.clone().call(g);
        let fh = self.call(h);
        fg.split(fh)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        macros::{
            category::{Compose, Id},
            Closure,
        },
        t_funk::{
            arrow::inst::Analyze, category::ComposeL, Add, Closure, Curry2, Curry2A, Div, Fanout,
            Function, Mul, Sum,
        },
    };

    use super::{Effect, EvalA};

    fn add(n: f32) -> Effect<Curry2A<Add, f32>> {
        Effect(Add.prefix2(n))
    }

    fn mul(n: f32) -> Effect<Curry2A<Mul, f32>> {
        Effect(Mul.prefix2(n))
    }

    fn div() -> Effect<Div> {
        Effect(Div)
    }

    #[derive(
        Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Id, Compose,
    )]
    struct Interpret;

    impl Function<Curry2A<Add, f32>> for Interpret {
        type Output = Curry2A<Add, f32>;

        fn call(input: Curry2A<Add, f32>) -> Self::Output {
            input
        }
    }

    impl Function<Curry2A<Mul, f32>> for Interpret {
        type Output = Curry2A<Mul, f32>;

        fn call(input: Curry2A<Mul, f32>) -> Self::Output {
            input
        }
    }

    impl Function<Div> for Interpret {
        type Output = Div;

        fn call(input: Div) -> Self::Output {
            input
        }
    }

    #[derive(
        Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Id, Compose,
    )]
    struct CountOps;

    impl Function<Curry2A<Add, f32>> for CountOps {
        type Output = Sum<usize>;

        fn call(_: Curry2A<Add, f32>) -> Self::Output {
            Sum(1)
        }
    }

    impl Function<Curry2A<Mul, f32>> for CountOps {
        type Output = Sum<usize>;

        fn call(_: Curry2A<Mul, f32>) -> Self::Output {
            Sum(1)
        }
    }

    impl Function<Div> for CountOps {
        type Output = Sum<usize>;

        fn call(_: Div) -> Self::Output {
            Sum(1)
        }
    }

    #[test]
    fn test_free_arrow() {
        let arr = add(2.0)
            .fanout(mul(2.0))
            .compose_l(div())
            .compose_l(add(6.0).fanout(mul(6.0)))
            .compose_l(div());

        let ops = Analyze::<CountOps, Sum<usize>>::default().call(arr);
        assert_eq!(ops, Sum(6));

        let foo = EvalA(Interpret).call(arr);
        let res = foo.call(5.0);
        assert_eq!(res, 1.5952381)
    }
}
