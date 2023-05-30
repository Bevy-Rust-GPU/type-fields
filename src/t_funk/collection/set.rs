//! Type-level Set, which can contain at most one instance of a given type

use type_fields_macros::functions;

use crate::t_funk::closure::{Closure, Compose, Composed, Curried2, Curry2, Flip, Flipped};

#[functions]
pub trait Set<T> {
    fn set(self, t: T) -> Self;
}

impl<T, A, B> Set<(A, B)> for T
where
    T: Clone + Set<A> + Set<B>,
{
    fn set(self, (a, b): (A, B)) -> Self {
        self.set(a).set(b)
    }
}

impl<T, A, B, C> Set<(A, B, C)> for T
where
    T: Clone + Set<A> + Set<B> + Set<C>,
{
    fn set(self, (a, b, c): (A, B, C)) -> Self {
        self.set(a).set(b).set(c)
    }
}

impl<T> Set<()> for T {
    fn set(self, (): ()) -> Self {
        self
    }
}

#[functions]
pub trait Get<T> {
    fn get(self) -> T;
}

impl<T> Get<()> for T {
    fn get(self) -> () {
        ()
    }
}

impl<T, A, B> Get<(A, B)> for T
where
    T: Clone + Get<A> + Get<B>,
{
    fn get(self) -> (A, B) {
        (self.clone().get(), self.get())
    }
}

impl<T, A, B, C> Get<(A, B, C)> for T
where
    T: Clone + Get<A> + Get<B> + Get<C>,
{
    fn get(self) -> (A, B, C) {
        (self.clone().get(), self.clone().get(), self.clone().get())
    }
}

/// Lift function F(P) -> O into function F(S) -> O,
/// where F(S) -> O reads P from the type-level set S.
#[functions]
pub trait LiftGet<U> {
    type LiftGet;

    fn lift_get(self) -> Self::LiftGet;
}

pub type LiftGetT<T, U> = <T as LiftGet<U>>::LiftGet;

impl<T, U> LiftGet<U> for T
where
    T: Closure<U>,
{
    type LiftGet = Composed<T, GetF<U>>;

    fn lift_get(self) -> Self::LiftGet {
        GetF::<U>::default().compose_l(self)
    }
}

/// Lift function F(P) -> O into function F(P) -> F(S) -> S;
/// where F(S) -> S writes O into the type-level set S.
pub trait LiftSet<U, O> {
    type LiftSet;

    fn lift_set(self) -> Self::LiftSet;
}

pub type LiftSetT<T, U, O> = <T as LiftSet<U, O>>::LiftSet;

impl<T, U, O> LiftSet<U, O> for T
where
    T: Closure<U, Output = O>,
{
    type LiftSet = Composed<Curried2<Flipped<SetF>>, T>;

    fn lift_set(self) -> Self::LiftSet {
        self.compose_l(SetF.flip().curry2())
    }
}

#[functions]
pub trait LiftContext<U> {
    type LiftContext;

    fn lift_context(self) -> Self::LiftContext;
}

pub type LiftContextT<T, U> = <T as LiftContext<U>>::LiftContext;

impl<T, U> LiftContext<U> for T {
    type LiftContext = Composed<Curried2<Flipped<SetF>>, Composed<T, GetF<U>>>;

    fn lift_context(self) -> Self::LiftContext {
        GetF::<U>::default()
            .compose_l(self)
            .compose_l(SetF.flip().curry2())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    struct Context {
        int: i32,
        float: f32,
        char: char,
        string: &'static str,
    }

    impl Get<i32> for Context {
        fn get(self) -> i32 {
            self.int
        }
    }

    impl Get<f32> for Context {
        fn get(self) -> f32 {
            self.float
        }
    }

    impl Get<char> for Context {
        fn get(self) -> char {
            self.char
        }
    }

    impl Get<&'static str> for Context {
        fn get(self) -> &'static str {
            self.string
        }
    }

    impl Set<i32> for Context {
        fn set(self, t: i32) -> Self {
            Context { int: t, ..self }
        }
    }

    impl Set<f32> for Context {
        fn set(self, t: f32) -> Self {
            Context { float: t, ..self }
        }
    }

    impl Set<char> for Context {
        fn set(self, t: char) -> Self {
            Context { char: t, ..self }
        }
    }

    impl Set<&'static str> for Context {
        fn set(self, t: &'static str) -> Self {
            Context { string: t, ..self }
        }
    }

    #[test]
    fn test_set() {
        let ctx = Context {
            int: 1,
            float: 2.0,
            char: '3',
            string: "4",
        };

        let int = Get::<i32>::get(ctx);
        let float = Get::<f32>::get(ctx);
        let char = Get::<char>::get(ctx);
        let string = Get::<&'static str>::get(ctx);

        assert_eq!(int, 1);
        assert_eq!(float, 2.0);
        assert_eq!(char, '3');
        assert_eq!(string, "4");

        let ctx = Set::<i32>::set(ctx, 2);
        let ctx = Set::<f32>::set(ctx, 3.0);
        let ctx = Set::<char>::set(ctx, '4');
        let ctx = Set::<&'static str>::set(ctx, "5");

        assert_eq!(ctx.int, 2);
        assert_eq!(ctx.float, 3.0);
        assert_eq!(ctx.char, '4');
        assert_eq!(ctx.string, "5");
    }
}
