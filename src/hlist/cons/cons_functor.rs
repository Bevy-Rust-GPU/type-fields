use crate::functional::{Closure, Functor};

impl<Head, Tail, F> Functor<F> for (Head, Tail)
where
    F: Clone + Closure<Head>,
    Tail: Functor<F>,
{
    type Mapped = (F::Output, Tail::Mapped);

    fn fmap(self, f: F) -> Self::Mapped {
        (f.clone().call(self.0), self.1.fmap(f))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        derive_closure,
        functional::{Function, Functor},
        hlist::{cons::Uncons, tuple::Cons},
    };

    #[test]
    fn test_cons_functor() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Mul2;

        impl Function<u32> for Mul2 {
            type Output = u32;

            fn call(input: u32) -> Self::Output {
                input * 2
            }
        }

        impl Function<f32> for Mul2 {
            type Output = f32;

            fn call(input: f32) -> Self::Output {
                input * 2.0
            }
        }

        derive_closure!(Mul2);

        let list = (1u32, 2.0f32, 3u32).cons();
        let list = list.fmap(Mul2);
        assert_eq!(list.uncons(), (2, 4.0, 6))
    }
}
