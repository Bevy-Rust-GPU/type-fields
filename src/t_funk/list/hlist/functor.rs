use crate::t_funk::{Closure, Fmap, Replace};

impl<Head, Tail, F> Fmap<F> for (Head, Tail)
where
    F: Clone + Closure<Head>,
    Tail: Fmap<F>,
{
    type Fmap = (F::Output, Tail::Fmap);

    fn fmap(self, f: F) -> Self::Fmap {
        (f.clone().call(self.0), self.1.fmap(f))
    }
}

impl<Head, Tail, T> Replace<T> for (Head, Tail)
where
    T: Clone,
    Tail: Replace<T>,
{
    type Replace = (T, Tail::Replace);

    fn replace(self, t: T) -> Self::Replace {
        (t.clone(), self.1.replace(t))
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
