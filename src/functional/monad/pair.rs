use crate::{
    derive_copointed, derive_pointed,
    functional::{Closure, Copointed, Functor, Pointed},
};

/// Identity monad, used to lift values into a monadic context.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair<A, B>(A, B);

derive_pointed!(Pair<A, B>);
derive_copointed!(Pair<A, B>);

impl<A, B, F> Functor<F> for Pair<A, B>
where
    F: Clone + Closure<A> + Closure<B>,
{
    type Mapped = Pair<<F as Closure<A>>::Output, <F as Closure<B>>::Output>;

    fn fmap(self, f: F) -> Self::Mapped {
        let (a, b) = self.copoint();
        Pointed::point((f.clone().call(a), f.call(b)))
    }
}

#[cfg(test)]
mod test {
    use crate::{functional::{
        monad::Pair, test_functor_laws, Add, Copointed, Curry, Function, Functor, Mul, Pointed,
    }, derive_closure};

    #[test]
    fn test_pair() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Mul3;

        impl Function<i32> for Mul3 {
            type Output = i32;

            fn call(input: i32) -> Self::Output {
                input * 3
            }
        }

        impl Function<f32> for Mul3 {
            type Output = f32;

            fn call(input: f32) -> Self::Output {
                input * 3.0
            }
        }

        derive_closure!(Mul3);

        let id1: Pair<i32, f32> = Pair::point((5, 6.0));
        assert_eq!(id1.copoint(), (5, 6.0));

        let id2: Pair<i32, f32> = id1.fmap(Mul3);
        assert_eq!(id2.copoint(), (15, 18.0));
    }

    #[test]
    fn test_functor_laws_pair() {
        test_functor_laws(Pair::point((1, 2)), Add.curry_a(2), Mul.curry_a(2));
    }
}
