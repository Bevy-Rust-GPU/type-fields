use crate::t_funk::hlist::{HList, Here, Next, Path, This};

/// Remove an element from a cons list by type, doing nothing if that type is not present.
pub trait RemoveImpl<T, Path>: HList {
    type Remove: HList;

    fn remove_impl(self) -> Self::Remove;
}

trait ValidPath: Path {}

impl<Tail> ValidPath for (Next, Tail) where Tail: ValidPath {}
impl ValidPath for (Here, ()) {}
impl ValidPath for (Next, This) {}

impl<Head, Tail, T, PathTail> RemoveImpl<T, (Next, PathTail)> for (Head, Tail)
where
    Self: HList,
    Tail: RemoveImpl<T, PathTail>,
    PathTail: ValidPath,
    (Head, Tail::Remove): HList,
{
    type Remove = (Head, Tail::Remove);

    fn remove_impl(self) -> Self::Remove {
        (self.0, self.1.remove_impl())
    }
}

impl<T, Tail> RemoveImpl<T, (Here, ())> for (T, Tail)
where
    Self: HList,
    Tail: HList,
{
    type Remove = Tail;

    fn remove_impl(self) -> Self::Remove {
        self.1
    }
}

pub trait Remove<Path>: HList {
    fn remove<T>(self) -> Self::Remove
    where
        Self: RemoveImpl<T, Path>;
}

impl<T, Path> Remove<Path> for T
where
    T: HList,
{
    fn remove<In>(self) -> <T as RemoveImpl<In, Path>>::Remove
    where
        Self: RemoveImpl<In, Path>,
    {
        self.remove_impl()
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{hlist::remove::Remove, tlist::ToHList};

    #[test]
    fn test_cons_remove() {
        let list: (usize, (f32, (&str, ()))) = (1, 2.0, "three").to_hlist();
        let list: (usize, (f32, ())) = list.remove::<&str>();
        let list: (usize, ()) = list.remove::<f32>();
        let list: () = list.remove::<usize>();
        assert_eq!(list, ());
    }
}
