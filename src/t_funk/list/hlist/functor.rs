use super::{Cons, Nil};
use crate::t_funk::{Closure, Fmap, Replace};

impl<T, N, F> Fmap<F> for Cons<T, N>
where
    F: Clone + Closure<T>,
    N: Fmap<F>,
{
    type Fmap = Cons<F::Output, N::Fmap>;

    fn fmap(self, f: F) -> Self::Fmap {
        Cons(f.clone().call(self.0), self.1.fmap(f))
    }
}

impl<F> Fmap<F> for Nil {
    type Fmap = Nil;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T, N, U> Replace<U> for Cons<T, N>
where
    U: Clone,
    N: Replace<U>,
{
    type Replace = Cons<U, N::Replace>;

    fn replace(self, t: U) -> Self::Replace {
        Cons(t.clone(), self.1.replace(t))
    }
}

impl<T> Replace<T> for Nil {
    type Replace = Nil;

    fn replace(self, _: T) -> Self::Replace {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::macros::Closure;

    use crate::t_funk::{hlist::ToTList, tlist::ToHList, Fmap, Function};

    #[test]
    fn test_hlist_functor() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
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

        let list = (1u32, 2.0f32, 3u32).to_hlist();
        let list = list.fmap(Mul2);
        assert_eq!(list.to_tlist(), (2, 4.0, 6))
    }
}
