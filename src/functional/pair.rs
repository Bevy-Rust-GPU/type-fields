use super::{Copointed, Function, Functor, Pointed};

/// Identity monad, used to lift values into a monadic context.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair<A, B>(A, B);

impl<A, B> Pointed for Pair<A, B> {
    type Pointed = (A, B);

    fn point((a, b): Self::Pointed) -> Self {
        Pair(a, b)
    }
}

impl<A, B> Copointed for Pair<A, B> {
    type Copointed = (A, B);

    fn copoint(self) -> Self::Copointed {
        (self.0, self.1)
    }
}

impl<A, B, F> Functor<F> for Pair<A, B>
where
    F: Clone + Function<A> + Function<B>,
{
    type Mapped = Pair<<F as Function<A>>::Output, <F as Function<B>>::Output>;

    fn fmap(self, f: F) -> Self::Mapped {
        let (a, b) = self.copoint();
        Pointed::point((f.clone().call(a), f.call(b)))
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{pair::Pair, Copointed, Function, Functor, Pointed};

    #[test]
    fn test_pair() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Mul3;

        impl Function<i32> for Mul3 {
            type Output = i32;

            fn call(self, input: i32) -> Self::Output {
                input * 3
            }
        }

        impl Function<f32> for Mul3 {
            type Output = f32;

            fn call(self, input: f32) -> Self::Output {
                input * 3.0
            }
        }

        let id1 = Pair::point((5, 6.0));
        let id2: Pair<i32, f32> = id1.fmap(Mul3);
        assert_eq!(id1.copoint(), (5, 6.0));
        assert_eq!(id2.copoint(), (15, 18.0));
    }
}
