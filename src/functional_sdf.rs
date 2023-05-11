use core::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};

use crate::{
    macros::{arrow::arrow, category::category, functions, functor::Replace, Closure},
    t_funk::{
        hlist::{Cons, Nil, PushBackF},
        CallF, ComposeL, ComposeLT, Composed, Curry2, Curry2B, Either, Fanout, FanoutT, FmapT, Fst,
        Gt, Lt, MakeIf, MakePair, OutputT, Split, SplitT, Splitted,
    },
};

use crate::t_funk::{function::Id, tlist::ToHList, Closure, Fmap, Function};

// Translation input modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Translate<T>(T, T);

impl<T> Distance for Translate<T> {
    type Distance = TranslateF<T>;

    fn distance(self) -> Self::Distance {
        TranslateF(self.0, self.1)
    }
}

impl<T> Gradient for Translate<T> {
    type Gradient = TranslateF<T>;

    fn gradient(self) -> Self::Gradient {
        TranslateF(self.0, self.1)
    }
}

// General translation function
#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TranslateF<T>(T, T);

impl<T> Closure<(T, T)> for TranslateF<T>
where
    T: core::ops::Add<Output = T>,
{
    type Output = (T, T);

    fn call(self, (x, y): (T, T)) -> Self::Output {
        let TranslateF(dx, dy) = self;
        (x + dx, y + dy)
    }
}

// Point field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point;

impl Distance for Point {
    type Distance = PointDistance;

    fn distance(self) -> Self::Distance {
        PointDistance
    }
}

impl Gradient for Point {
    type Gradient = PointGradient;

    fn gradient(self) -> Self::Gradient {
        PointGradient
    }
}

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct PointDistance;

impl<T> Function<(T, T)> for PointDistance
where
    T: Clone + Add<Output = T> + Mul<Output = T>,
{
    type Output = T;

    fn call((x, y): (T, T)) -> Self::Output {
        x.clone() * x + y.clone() * y
    }
}

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct PointGradient;

impl<T> Function<(T, T)> for PointGradient
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    type Output = (T, T);

    fn call((x, y): (T, T)) -> Self::Output {
        let l = x.clone() * x.clone() + y.clone() * y.clone();
        (x / l.clone(), y / l)
    }
}

// Isosurface output modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Isosurface<T>(T);

impl<T> Distance for Isosurface<T> {
    type Distance = IsosurfaceDistance<T>;

    fn distance(self) -> Self::Distance {
        IsosurfaceDistance(self.0)
    }
}

impl<T> Gradient for Isosurface<T> {
    type Gradient = Id;

    fn gradient(self) -> Self::Gradient {
        Id
    }
}

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IsosurfaceDistance<T>(T);

impl<T> Closure<T> for IsosurfaceDistance<T>
where
    T: Sub<Output = T>,
{
    type Output = T;

    fn call(self, input: T) -> Self::Output {
        input - self.0
    }
}

// Given a symbol, return its function in the distance domain
#[functions]
pub trait Distance {
    type Distance;

    fn distance(self) -> Self::Distance;
}

pub type DistanceT<T> = <T as Distance>::Distance;

impl<LHS, RHS> Distance for Cons<LHS, RHS>
where
    LHS: Distance,
    RHS: Distance,
{
    type Distance = Cons<DistanceT<LHS>, DistanceT<RHS>>;

    fn distance(self) -> Self::Distance {
        Cons(self.0.distance(), self.1.distance())
    }
}

impl Distance for Nil {
    type Distance = Nil;

    fn distance(self) -> Self::Distance {
        Nil
    }
}

// Given a symbol, return its function in the gradient domain
#[functions]
pub trait Gradient {
    type Gradient;

    fn gradient(self) -> Self::Gradient;
}

pub type GradientT<T> = <T as Gradient>::Gradient;

impl<LHS, RHS> Gradient for Cons<LHS, RHS>
where
    LHS: Gradient,
    RHS: Gradient,
{
    type Gradient = Cons<GradientT<LHS>, GradientT<RHS>>;

    fn gradient(self) -> Self::Gradient {
        Cons(self.0.gradient(), self.1.gradient())
    }
}

impl Gradient for Nil {
    type Gradient = Nil;

    fn gradient(self) -> Self::Gradient {
        Nil
    }
}

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Replace)]
pub struct Shape<T>(pub T);

impl<T, F> Fmap<F> for Shape<T>
where
    T: Fmap<F>,
{
    type Fmap = Shape<FmapT<T, F>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Shape(self.0.fmap(f))
    }
}

impl<T> Distance for Shape<T>
where
    T: Distance,
{
    type Distance = Shape<DistanceT<T>>;

    fn distance(self) -> Self::Distance {
        Shape(self.0.distance())
    }
}

impl<T> Gradient for Shape<T>
where
    T: Gradient,
{
    type Gradient = Shape<GradientT<T>>;

    fn gradient(self) -> Self::Gradient {
        Shape(self.0.gradient())
    }
}

impl<T, I> Closure<I> for Shape<T>
where
    T: Closure<I>,
{
    type Output = OutputT<T, I>;

    fn call(self, input: I) -> Self::Output {
        self.0.call(input)
    }
}

impl<L, R> BitAnd<R> for Shape<L> {
    type Output = Meeted<Self, R>;

    fn bitand(self, rhs: R) -> Self::Output {
        Meeted(self, rhs)
    }
}

impl<L, R> BitOr<R> for Shape<L> {
    type Output = Joined<Self, R>;

    fn bitor(self, rhs: R) -> Self::Output {
        Joined(self, rhs)
    }
}

pub trait ToShape: Sized {
    type ToShape;

    fn shape(self) -> Shape<Self::ToShape>;
}

impl<T> ToShape for T
where
    T: ToHList,
{
    type ToShape = T::HList;
    fn shape(self) -> Shape<Self::ToShape> {
        Shape(self.to_hlist())
    }
}

// \/
trait Join<T>: Sized {
    fn join(self, t: T) -> Joined<Self, T> {
        Joined(self, t)
    }
}

impl<L, R> Join<R> for L {}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Joined<T, U>(pub T, pub U);

impl<L, R, F> Fmap<F> for Joined<L, R>
where
    L: Fmap<F>,
    R: Fmap<F>,
    F: Clone,
{
    type Fmap = Joined<L::Fmap, R::Fmap>;

    fn fmap(self, f: F) -> Self::Fmap {
        Joined(self.0.fmap(f.clone()), self.1.fmap(f))
    }
}

impl<L, R, P> Closure<P> for Joined<L, R>
where
    L: Clone + Closure<P>,
    OutputT<L, P>: Clone,
    R: Clone + Closure<P, Output = OutputT<L, P>>,
    P: Clone,
    Lt: Closure<(OutputT<L, P>, OutputT<L, P>), Output = bool>,
{
    type Output = OutputT<L, P>;

    fn call(self, p: P) -> Self::Output {
        let dl = self.0.clone().call(p.clone());
        let dr = self.1.clone().call(p);

        if Lt.call((dl.clone(), dr.clone())) {
            dl
        } else {
            dr
        }
    }
}

impl<L1, L2, R> BitAnd<R> for Joined<L1, L2> {
    type Output = Meeted<Self, R>;

    fn bitand(self, rhs: R) -> Self::Output {
        Meeted(self, rhs)
    }
}

impl<L1, L2, R> BitOr<R> for Joined<L1, L2> {
    type Output = Joined<Self, R>;

    fn bitor(self, rhs: R) -> Self::Output {
        Joined(self, rhs)
    }
}

// /\
trait Meet<T>: Sized {
    fn meet(self, t: T) -> Meeted<Self, T> {
        Meeted(self, t)
    }
}

impl<L, R> Meet<R> for L {}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Meeted<T, U>(pub T, pub U);

impl<L, R, F> Fmap<F> for Meeted<L, R>
where
    L: Fmap<F>,
    R: Fmap<F>,
    F: Clone,
{
    type Fmap = Meeted<L::Fmap, R::Fmap>;

    fn fmap(self, f: F) -> Self::Fmap {
        Meeted(self.0.fmap(f.clone()), self.1.fmap(f))
    }
}

impl<L, R, P> Closure<P> for Meeted<L, R>
where
    L: Clone + Closure<P>,
    OutputT<L, P>: Clone,
    R: Clone + Closure<P, Output = OutputT<L, P>>,
    P: Clone,
    Gt: Closure<(OutputT<L, P>, OutputT<L, P>), Output = bool>,
{
    type Output = OutputT<L, P>;

    fn call(self, p: P) -> Self::Output {
        let dl = self.0.clone().call(p.clone());
        let dr = self.1.clone().call(p);

        if Gt.call((dl.clone(), dr.clone())) {
            dl
        } else {
            dr
        }
    }
}

impl<L1, L2, R> BitAnd<R> for Meeted<L1, L2> {
    type Output = Meeted<Self, R>;

    fn bitand(self, rhs: R) -> Self::Output {
        Meeted(self, rhs)
    }
}

impl<L1, L2, R> BitOr<R> for Meeted<L1, L2> {
    type Output = Joined<Self, R>;

    fn bitor(self, rhs: R) -> Self::Output {
        Joined(self, rhs)
    }
}

impl<L, R> Distance for Either<L, R>
where
    L: Distance,
    R: Distance,
{
    type Distance = Either<DistanceT<L>, DistanceT<R>>;

    fn distance(self) -> Self::Distance {
        match self {
            Either::Left(l) => Either::Left(l.distance()),
            Either::Right(r) => Either::Right(r.distance()),
        }
    }
}

impl<L, R> Gradient for Either<L, R>
where
    L: Gradient,
    R: Gradient,
{
    type Gradient = Either<GradientT<L>, GradientT<R>>;

    fn gradient(self) -> Self::Gradient {
        match self {
            Either::Left(l) => Either::Left(l.gradient()),
            Either::Right(r) => Either::Right(r.gradient()),
        }
    }
}

impl<L, R, I> Closure<I> for Either<L, R>
where
    L: Closure<I>,
    R: Closure<I, Output = OutputT<L, I>>,
{
    type Output = OutputT<L, I>;

    fn call(self, i: I) -> Self::Output {
        match self {
            Either::Left(f) => f.call(i),
            Either::Right(f) => f.call(i),
        }
    }
}

#[functions]
pub trait Evaluate {
    type Evaluate;

    fn evaluate(self) -> Self::Evaluate;
}

pub type EvaluateT<T> = <T as Evaluate>::Evaluate;

impl<T> Evaluate for T
where
    T: Clone + Distance,
    DistanceT<T>: Split<Curry2B<PushBackF, T>>,
{
    type Evaluate = SplitT<DistanceT<T>, Curry2B<PushBackF, T>>;

    fn evaluate(self) -> Self::Evaluate {
        let d = self.clone().distance();
        d.split(PushBackF.suffix(self))
    }
}

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct MakeCons;

impl<T> Function<T> for MakeCons {
    type Output = Cons<T, Nil>;

    fn call(input: T) -> Self::Output {
        Cons(input, Nil)
    }
}

impl<T, U> Evaluate for Meeted<T, U>
where
    T: Clone + Evaluate,
    U: Clone + Evaluate,
    EvaluateT<T>: Fanout<EvaluateT<U>>,
    FanoutT<EvaluateT<T>, EvaluateT<U>>: ComposeL<TransposeTuple>,
{
    type Evaluate = Composed<
        Splitted<Composed<UnwrapEither, CallF>, Composed<MakeCons, CallF>>,
        Composed<
            TransposeTuple,
            Composed<
                Composed<
                    Splitted<
                        Splitted<MakeIf, MakeIf>,
                        Composed<Composed<Splitted<Id, Id>, MakePair>, Composed<Gt, Fst>>,
                    >,
                    MakePair,
                >,
                ComposeLT<FanoutT<EvaluateT<T>, EvaluateT<U>>, TransposeTuple>,
            >,
        >,
    >;

    fn evaluate(self) -> Self::Evaluate {
        let d0 = self.0.clone().evaluate();
        let d1 = self.1.clone().evaluate();

        d0.fanout(d1)
            .compose_l(TransposeTuple)
            .compose_l(
                MakeIf
                    .split(MakeIf)
                    .fanout(Fst.compose_l(Gt).compose_l(Id.fanout(Id))),
            )
            .compose_l(TransposeTuple)
            .compose_l((CallF.compose_l(UnwrapEither)).split(CallF.compose_l(MakeCons)))
    }
}

impl<T, U> Evaluate for Joined<T, U>
where
    T: Clone + Evaluate,
    U: Clone + Evaluate,
    EvaluateT<T>: Fanout<EvaluateT<U>>,
    FanoutT<EvaluateT<T>, EvaluateT<U>>: ComposeL<TransposeTuple>,
{
    type Evaluate = Composed<
        Splitted<Composed<UnwrapEither, CallF>, Composed<MakeCons, CallF>>,
        Composed<
            TransposeTuple,
            Composed<
                Composed<
                    Splitted<
                        Splitted<MakeIf, MakeIf>,
                        Composed<Composed<Splitted<Id, Id>, MakePair>, Composed<Lt, Fst>>,
                    >,
                    MakePair,
                >,
                ComposeLT<FanoutT<EvaluateT<T>, EvaluateT<U>>, TransposeTuple>,
            >,
        >,
    >;

    fn evaluate(self) -> Self::Evaluate {
        let d0 = self.0.clone().evaluate();
        let d1 = self.1.clone().evaluate();

        d0.fanout(d1)
            .compose_l(TransposeTuple)
            .compose_l(
                MakeIf
                    .split(MakeIf)
                    .fanout(Fst.compose_l(Lt).compose_l(Id.fanout(Id))),
            )
            .compose_l(TransposeTuple)
            .compose_l((CallF.compose_l(UnwrapEither)).split(CallF.compose_l(MakeCons)))
    }
}

// Given a matrix described by a 2x2 tuple, transpose it
#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct TransposeTuple;

impl<A, B, C, D> Function<((A, B), (C, D))> for TransposeTuple {
    type Output = ((A, C), (B, D));

    fn call(((a, b), (c, d)): ((A, B), (C, D))) -> Self::Output {
        ((a, c), (b, d))
    }
}

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct UnwrapEither;

impl<T> Function<Either<T, T>> for UnwrapEither {
    type Output = T;

    fn call(input: Either<T, T>) -> Self::Output {
        input.unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        functional_sdf::{
            Evaluate, EvaluateF, GradientF, Isosurface, Point, ToShape, Translate, TransposeTuple,
        },
        t_funk::{
            closure::Compose, function::Id, list::hlist::Nil, CallF, Closure, Curry2, Either,
            Fanout, Fmap, FmapF, Fst, Lt, MakeIf, Split, arrow::Second,
        },
    };

    #[test]
    fn test_functional_sdf() {
        let shape_a = (Translate(-1.0, -1.0), Point, Isosurface(2.0)).shape();
        let shape_b = (Translate(1.0, 1.0), Point, Isosurface(1.0)).shape();
        let shape_c = (Translate(0.0, 1.0), Point, Isosurface(3.0)).shape();
        let shape_d = (Translate(0.0, -1.0), Point, Isosurface(1.5)).shape();

        let p = (-1.0, 0.0);

        let oof = shape_a
            .evaluate()
            .fanout(shape_b.evaluate())
            .compose_l(TransposeTuple)
            .compose_l(
                MakeIf
                    .split(MakeIf)
                    .fanout(Fst.compose_l(Lt).compose_l(Id.fanout(Id))),
            )
            .compose_l(TransposeTuple)
            .compose_l((CallF.compose_l(Either::unwrap)).split(CallF));
        let _rab = oof.call((p, Nil));
        //panic!("{rab:#?}");

        let foo = shape_a.evaluate();
        let _bar = foo.call((p, Nil));
        //panic!("{bar:#?}");

        let foo = shape_a | shape_b | shape_c & shape_d;
        let bar = foo.evaluate();
        let _baz = bar.call((p, Nil));
        //panic!("{baz:#?}");

        let foo = (Translate(-1.0, -1.0), foo, Isosurface(2.0))
            .shape()
            .fmap(EvaluateF);

        let bar = foo.call((p, Nil));
        //panic!("{bar:#?}");

        let baz = FmapF
            .suffix(GradientF)
            .compose_l(CallF.suffix(p))
            .second()
            .call(bar);

        panic!("{baz:#?}");
    }
}
