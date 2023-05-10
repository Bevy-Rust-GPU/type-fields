use core::ops::{Add, BitAnd, BitOr, Deref, DerefMut, Div, Mul, Sub};

use crate::{
    macros::{
        functions,
        functor::{Fmap, Replace},
        Closure,
    },
    t_funk::{
        hlist::{Cons, Nil},
        Gt, Lt,
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

impl<LHS, RHS> Distance for Cons<LHS, RHS>
where
    LHS: Distance,
    RHS: Distance,
{
    type Distance = Cons<LHS::Distance, RHS::Distance>;

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

impl<LHS, RHS> Gradient for Cons<LHS, RHS>
where
    LHS: Gradient,
    RHS: Gradient,
{
    type Gradient = Cons<LHS::Gradient, RHS::Gradient>;

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

// Given a point, evaluate a distance and corresponding shape
#[functions]
pub trait Evaluate<P, E> {
    type Evaluate;

    fn evaluate(self, p: P) -> (E, Self::Evaluate);
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Fmap, Replace)]
pub struct Shape<T>(pub T);

impl<T> Distance for Shape<T>
where
    T: Distance,
{
    type Distance = Shape<T::Distance>;

    fn distance(self) -> Self::Distance {
        Shape(self.0.distance())
    }
}

impl<T> Gradient for Shape<T>
where
    T: Gradient,
{
    type Gradient = Shape<T::Gradient>;

    fn gradient(self) -> Self::Gradient {
        Shape(self.0.gradient())
    }
}

impl<T, I> Closure<I> for Shape<T>
where
    T: Closure<I>,
{
    type Output = T::Output;

    fn call(self, input: I) -> Self::Output {
        self.0.call(input)
    }
}

impl<T, P, E> Evaluate<P, E> for Shape<T>
where
    Shape<T>: Clone + Distance,
    <Shape<T> as Distance>::Distance: Closure<P, Output = E>,
{
    type Evaluate = T;

    fn evaluate(self, p: P) -> (E, Self::Evaluate) {
        (self.clone().distance().call(p), self.0)
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

impl<L, R> Distance for Joined<L, R>
where
    L: Distance,
    R: Distance,
{
    type Distance = Joined<L::Distance, R::Distance>;

    fn distance(self) -> Self::Distance {
        Joined(self.0.distance(), self.1.distance())
    }
}

impl<L, R> Gradient for Joined<L, R>
where
    L: Gradient,
    R: Gradient,
{
    type Gradient = Joined<L::Gradient, R::Gradient>;

    fn gradient(self) -> Self::Gradient {
        Joined(self.0.gradient(), self.1.gradient())
    }
}

impl<L, R, P> Closure<P> for Joined<L, R>
where
    L: Clone + Closure<P>,
    L::Output: Clone,
    R: Clone + Closure<P, Output = L::Output>,
    P: Clone,
    Lt: Closure<(L::Output, L::Output), Output = bool>,
{
    type Output = L::Output;

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

impl<L, R, P, E> Evaluate<P, E> for Joined<L, R>
where
    L: Clone,
    L: Evaluate<P, E>,
    R: Clone,
    R: Evaluate<P, E>,
    P: Clone,
    E: Clone,
    Lt: Closure<(E, E), Output = bool>,
{
    type Evaluate = Either<L::Evaluate, R::Evaluate>;

    fn evaluate(self, p: P) -> (E, Self::Evaluate) {
        let (dl, l) = self.0.clone().evaluate(p.clone());
        let (dr, r) = self.1.clone().evaluate(p);

        if Lt.call((dl.clone(), dr.clone())) {
            (dl, Either::Left(l))
        } else {
            (dr, Either::Right(r))
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

impl<L, R> Distance for Meeted<L, R>
where
    L: Distance,
    R: Distance,
{
    type Distance = Meeted<L::Distance, R::Distance>;

    fn distance(self) -> Self::Distance {
        Meeted(self.0.distance(), self.1.distance())
    }
}

impl<L, R> Gradient for Meeted<L, R>
where
    L: Gradient,
    R: Gradient,
{
    type Gradient = Meeted<L::Gradient, R::Gradient>;

    fn gradient(self) -> Self::Gradient {
        Meeted(self.0.gradient(), self.1.gradient())
    }
}

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
    L::Output: Clone,
    R: Clone + Closure<P, Output = L::Output>,
    P: Clone,
    Gt: Closure<(L::Output, L::Output), Output = bool>,
{
    type Output = L::Output;

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

impl<L, R, P, E> Evaluate<P, E> for Meeted<L, R>
where
    L: Clone,
    L: Evaluate<P, E>,
    R: Clone,
    R: Evaluate<P, E>,
    P: Clone,
    E: Clone,
    Gt: Closure<(E, E), Output = bool>,
{
    type Evaluate = Either<L::Evaluate, R::Evaluate>;

    fn evaluate(self, p: P) -> (E, Self::Evaluate) {
        let (dl, l) = self.0.clone().evaluate(p.clone());
        let (dr, r) = self.1.clone().evaluate(p);

        if Gt.call((dl.clone(), dr.clone())) {
            (dl, Either::Left(l))
        } else {
            (dr, Either::Right(r))
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct Min;

impl Function<(f32, f32)> for Min {
    type Output = f32;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l.min(r)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct Max;

impl Function<(f32, f32)> for Max {
    type Output = f32;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l.max(r)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<T> Deref for Either<T, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => r,
        }
    }
}

impl<T> DerefMut for Either<T, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => r,
        }
    }
}

impl<L, R> Either<L, R> {
    pub fn try_left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub fn try_right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    pub fn left(self) -> L {
        self.try_left().expect("Called left() on a Right value")
    }

    pub fn right(self) -> R {
        self.try_right().expect("Called right() on a Left value")
    }
}

impl<T> Either<T, T> {
    pub fn unwrap(self) -> T {
        match self {
            Either::Left(t) | Either::Right(t) => t,
        }
    }
}

impl<L, R, F> Fmap<F> for Either<L, R>
where
    L: Fmap<F>,
    R: Fmap<F>,
{
    type Fmap = Either<L::Fmap, R::Fmap>;

    fn fmap(self, f: F) -> Self::Fmap {
        match self {
            Either::Left(l) => Either::Left(l.fmap(f)),
            Either::Right(r) => Either::Right(r.fmap(f)),
        }
    }
}

impl<L, R> Distance for Either<L, R>
where
    L: Distance,
    R: Distance,
{
    type Distance = Either<L::Distance, R::Distance>;

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
    type Gradient = Either<L::Gradient, R::Gradient>;

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
    R: Closure<I, Output = L::Output>,
{
    type Output = L::Output;

    fn call(self, i: I) -> Self::Output {
        match self {
            Either::Left(f) => f.call(i),
            Either::Right(f) => f.call(i),
        }
    }
}

impl<L, R, P, E> Evaluate<P, E> for Either<L, R>
where
    L: Evaluate<P, E>,
    R: Evaluate<P, E>,
{
    type Evaluate = Either<L::Evaluate, R::Evaluate>;

    fn evaluate(self, p: P) -> (E, Self::Evaluate) {
        match self {
            Either::Left(l) => {
                let (e, l) = l.evaluate(p);
                (e, Either::Left(l))
            }
            Either::Right(r) => {
                let (e, r) = r.evaluate(p);
                (e, Either::Right(r))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::t_funk::{
        branch, branch_l, branch_r, leaf, root, All, Any, CallF, ComposeL, Curry2, Foldr, MconcatF,
        Split,
    };

    #[test]
    fn test_functional_sdf() {
        let shape_a = (Translate(-1.0, -1.0), Point, Isosurface(2.0)).shape();
        let shape_b = (Translate(1.0, 1.0), Point, Isosurface(1.0)).shape();
        let shape_c = (Translate(0.0, 1.0), Point, Isosurface(3.0)).shape();
        let shape_d = (Translate(0.0, -1.0), Point, Isosurface(1.5)).shape();

        let p = (0.0, 0.0);

        let joined = shape_a | shape_b | shape_c & shape_d;
        let eval = joined.evaluate(p);
        //panic!("{eval:#?}");

        let arr = Id.split(GradientF.compose_l(CallF.suffix(p))).call(eval);
        //panic!("{arr:#?}");

        let shape_e = (Translate(-1.0, -1.0), joined, Isosurface(2.0)).shape();
        let eval = shape_e.evaluate(p);
        panic!("{eval:#?}");

        //let arr = Id.split(GradientF.compose_l(CallF.suffix(p))).call(eval);
        //panic!("{arr:#?}");
    }
}
