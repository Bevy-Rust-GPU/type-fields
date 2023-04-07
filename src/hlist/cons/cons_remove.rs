use crate::hlist::{
    cons::ConsList,
    path::{Here, Next, Path, This},
};

/// Remove an element from a cons list by type, doing nothing if that type is not present.
pub trait ConsRemoveImpl<T, Path>: ConsList {
    type ConsTryRemove: ConsList;

    fn cons_remove_impl(self) -> Self::ConsTryRemove;
}

trait ValidPath: Path {}

impl<Tail> ValidPath for (Next, Tail) where Tail: ValidPath {}
impl ValidPath for (Here, ()) {}
impl ValidPath for (Next, This) {}

impl<Head, Tail, T, PathTail> ConsRemoveImpl<T, (Next, PathTail)> for (Head, Tail)
where
    Self: ConsList,
    Tail: ConsRemoveImpl<T, PathTail>,
    PathTail: ValidPath,
    (Head, Tail::ConsTryRemove): ConsList,
{
    type ConsTryRemove = (Head, Tail::ConsTryRemove);

    fn cons_remove_impl(self) -> Self::ConsTryRemove {
        (self.0, self.1.cons_remove_impl())
    }
}

impl<T, Tail> ConsRemoveImpl<T, (Here, ())> for (T, Tail)
where
    Self: ConsList,
    Tail: ConsList,
{
    type ConsTryRemove = Tail;

    fn cons_remove_impl(self) -> Self::ConsTryRemove {
        self.1
    }
}

pub trait ConsRemove<Path>: ConsList {
    fn cons_remove<T>(self) -> Self::ConsTryRemove
    where
        Self: ConsRemoveImpl<T, Path>;
}

impl<T, Path> ConsRemove<Path> for T
where
    T: ConsList,
{
    fn cons_remove<In>(self) -> <T as ConsRemoveImpl<In, Path>>::ConsTryRemove
    where
        Self: ConsRemoveImpl<In, Path>,
    {
        self.cons_remove_impl()
    }
}

#[cfg(test)]
mod test {
    use crate::hlist::{cons::cons_remove::ConsRemove, tuple::Cons};

    #[test]
    fn test_cons_remove() {
        let list: (usize, (f32, (&str, ()))) = (1, 2.0, "three").cons();
        let list: (usize, (f32, ())) = list.cons_remove::<&str>();
        let list: (usize, ()) = list.cons_remove::<f32>();
        let list: () = list.cons_remove::<usize>();
        assert_eq!(list, ());
    }
}

