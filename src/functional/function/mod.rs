mod add;
mod compose;
mod constant;
mod curry;
mod curry_n {
    use core::marker::PhantomData;

    use crate::{
        functional::Pointed,
        hlist::{
            cons::{ConsList, ConsPushBack, Uncons},
            tuple::Cons,
        },
    };

    use super::Function;

    pub trait CurryN<I> {
        type Curried;

        fn curry_n(self) -> Self::Curried;
    }

    impl<T, I> CurryN<I> for T
    where
        T: Function<I>,
        I: Cons,
    {
        type Curried = CurriedN<T, (), I::Cons>;

        fn curry_n(self) -> Self::Curried {
            CurriedN(self, (), PhantomData)
        }
    }

    pub struct CurriedN<F, AO, AI>(F, AO, PhantomData<AI>);

    impl<F, AI> Default for CurriedN<F, (), AI>
    where
        F: Default,
    {
        fn default() -> Self {
            CurriedN(Default::default(), (), PhantomData)
        }
    }

    impl<F, AO, AI> Clone for CurriedN<F, AO, AI>
    where
        F: Clone,
        AO: Clone,
    {
        fn clone(&self) -> Self {
            CurriedN(self.0.clone(), self.1.clone(), PhantomData)
        }
    }

    impl<F, AI> Pointed for CurriedN<F, (), AI>
    where
        F: Function<AI::Uncons>,
        AI: ConsList,
    {
        type Pointed = F;

        fn point(unit: Self::Pointed) -> Self {
            CurriedN(unit, (), PhantomData)
        }
    }

    impl<F, AO, I, P> Function<I, P> for CurriedN<F, AO, (I, ())>
    where
        AO: ConsPushBack<I, P>,
        F: Function<<AO::ConsPushBack as Uncons>::Uncons>,
    {
        type Output = F::Output;

        fn call(self, input: I) -> Self::Output {
            self.0.call(self.1.cons_push_back(input).uncons())
        }
    }

    impl<F, AO, Tail, Tail2, I, P> Function<I, P> for CurriedN<F, AO, (I, (Tail, Tail2))>
    where
        AO: ConsPushBack<I, P>,
    {
        type Output = CurriedN<F, AO::ConsPushBack, (Tail, Tail2)>;

        fn call(self, input: I) -> Self::Output {
            CurriedN(self.0, self.1.cons_push_back(input), PhantomData)
        }
    }

    #[cfg(test)]
    mod test {
        use crate::functional::{function::curry_n::CurryN, Add, Function};

        #[test]
        fn test_curry_n() {
            let curried = Add.curry_n();
            let curried = curried.call(1);
            let curried = curried.call(2);
            assert_eq!(curried, 3);
        }
    }
}
mod div;
mod flip;
mod id;
mod mul;
mod sub;

pub use add::*;
pub use compose::*;
pub use constant::*;
pub use curry::*;
pub use div::*;
pub use flip::*;
pub use id::*;
pub use mul::*;
pub use sub::*;

/// Abstract function trait.
///
/// Allows many signatures to be implemented on a single type,
/// unlike Fn* whose present implementation encodes a 1:1 coupling.
///
/// This allowing mapping over heterogenous lists.
pub trait Function<Inputs, Generics = ()> {
    type Output;

    fn call(self, input: Inputs) -> Self::Output;
}
