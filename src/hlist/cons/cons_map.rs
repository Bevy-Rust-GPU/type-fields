use crate::functional::Function;

use super::ConsList;

pub trait ConsMap<F>: ConsList {
    type ConsMap: ConsList;
    fn map(self, f: F) -> Self::ConsMap;
}

impl<Head, Tail, F> ConsMap<F> for (Head, Tail)
where
    Self: ConsList,
    F: Clone + Function<Head>,
    Tail: ConsMap<F>,
    (F::Output, Tail::ConsMap): ConsList,
{
    type ConsMap = (F::Output, Tail::ConsMap);

    fn map(self, f: F) -> Self::ConsMap {
        (f.clone().call(self.0), self.1.map(f))
    }
}

impl<F> ConsMap<F> for () {
    type ConsMap = ();

    fn map(self, _: F) -> Self::ConsMap {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::hlist::{
        cons::{
            cons_map::{ConsMap, Function},
            Uncons,
        },
        tuple::Cons,
    };

    #[test]
    fn test_cons_map() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Mul;

        impl Function<u32> for Mul {
            type Output = u32;

            fn call(self, input: u32) -> Self::Output {
                input * 2
            }
        }

        impl Function<f32> for Mul {
            type Output = f32;

            fn call(self, input: f32) -> Self::Output {
                input * 2.0
            }
        }

        let list = (1u32, 2.0f32, 3u32).cons();
        let list = list.map(Mul);
        assert_eq!(list.uncons(), (2, 4.0, 6))
    }
}
