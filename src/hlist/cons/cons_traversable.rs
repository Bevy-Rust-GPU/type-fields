use crate::functional::{
    Applicative, Curried, Curry, Flip, Flipped, Functor, SequenceA, Traversable,
};

use super::PushFront;

impl<Head, Tail, F> Traversable<F> for (Head, Tail)
where
    (Head, Tail): Functor<F>,
    <(Head, Tail) as Functor<F>>::Mapped: SequenceA,
{
    type Traversed = <<(Head, Tail) as Functor<F>>::Mapped as SequenceA>::Sequenced;

    fn traverse(self, f: F) -> Self::Traversed {
        self.fmap(f).sequence_a()
    }
}

impl<Head, Tail> SequenceA for (Head, Tail)
where
    Self: Functor<Curried<Flipped<PushFront>>>,
    <Self as Functor<Curried<Flipped<PushFront>>>>::Mapped: Applicative<()>,
{
    type Sequenced =
        <<Self as Functor<Curried<Flipped<PushFront>>>>::Mapped as Applicative<()>>::Applied;

    fn sequence_a(self) -> Self::Sequenced {
        self.fmap(PushFront.flip().curry()).apply(())
    }
}

#[cfg(test)]
mod test {
    use super::Traversable;
    use crate::{functional::Function, hlist::tuple::Cons};

    #[test]
    fn test_traversable() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Test;


        impl Function<i32> for Test {
            type Output = (i32, (i32, ()));

            fn call(self, input: i32) -> Self::Output {
                (input + 1, (input + 2, ()))
            }
        }

        let foo = (0, 1, 2).cons();
        let bar = foo.traverse(Test);
        assert_eq!(
            bar,
            (
                ((1, (2, ())), ()),
                (((2, (3, ())), ()), (((3, (4, ())), ()), ()))
            )
        );
    }
}
