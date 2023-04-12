use crate::functional::{Composed, Copointed, Id, Pointed, Semigroup, Tagged};

use super::Monoid;

/// The monoid of endomorphisms under composition.
pub type Endo<T> = Tagged<TagEndo, T>;
pub enum TagEndo {}

impl<T> Monoid for Endo<T>
where
    T: Monoid,
{
    type Identity = Endo<Id>;

    fn mempty() -> Self::Identity {
        Endo::point(Id)
    }
}

impl<T, U> Semigroup<Endo<U>> for Endo<T> {
    type Appended = Endo<Composed<T, U>>;

    fn mappend(self, u: Endo<U>) -> Self::Appended {
        Endo::point(Composed::point((self.copoint(), u.copoint())))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        functional::{
            Add, Compose, Composed, Copointed, Curry, CurryA, CurryB, Foldr, Function, Point,
            Pointed, Semigroup, TagEndo, Tagged,
        },
        hlist::tuple::Cons,
    };

    use super::Endo;

    #[test]
    fn test_endo() {
        let foo = Endo::point(Add.curry().call(1));
        let foo = foo.mappend(Endo::point(Add.curry().call(2)));
        let foo = foo.mappend(Endo::point(Add.curry().call(3)));
        let bar = foo.copoint();
        let baz = bar.call(4);
        assert_eq!(baz, 10);
    }

    #[test]
    fn test_foldr() {
        let list = (1, 2, 3).cons();
        let folded = list.foldr(Add.curry(), 0);
        assert_eq!(folded, 6);
    }

    #[test]
    fn test_composition() {
        let add: Add = Add;
        let add_result: i32 = add.call((1, 2));
        assert_eq!(add_result, 3);

        let endo = Point::default();
        let _endo_result: Endo<Add> = endo.call(add);

        let add_endo: Composed<Add, Point<Tagged<TagEndo, i32>>> =
            Compose.call((add, Point::<Endo<i32>>::default()));
        let add_endo_result: Endo<i32> = add_endo.call((1, 2));
        assert_eq!(add_endo_result.copoint(), 3);

        let add_curry_a: CurryA<Add, i32, i32> = Add.curry();
        let add_curry_b: CurryB<Add, i32, i32> = add_curry_a.call(1);
        let add_curry_result: i32 = add_curry_b.call(1);
        assert_eq!(add_curry_result, 2);

        let add_curry_endo_a: Composed<
            CurryA<Add, i32, i32>,
            Point<Endo<<CurryA<Add, i32, i32> as Function<i32>>::Output>>,
        > = Compose.call((Add.curry(), Point::default()));
        let add_curry_endo_b: Tagged<TagEndo, CurryB<Add, i32, i32>> = add_curry_endo_a.call(1);
        let add_curry_endo_result: i32 = add_curry_endo_b.copoint().call(2);
        assert_eq!(add_curry_endo_result, 3);
    }
}
