use crate::t_funk::hlist::{HList, Here, Next, Path, This};

use super::{Cons, Nil};

/// Remove an element from a HList by type, doing nothing if that type is not present.
pub trait RemoveImpl<T, Path>: HList {
    type Remove: HList;

    fn remove_impl(self) -> Self::Remove;
}

trait ValidPath: Path {}

impl<Tail> ValidPath for Cons<Next, Tail> where Tail: ValidPath {}
impl ValidPath for Cons<Here, Nil> {}
impl ValidPath for Cons<Next, This> {}

impl<Head, Tail, T, PathTail> RemoveImpl<T, Cons<Next, PathTail>> for Cons<Head, Tail>
where
    Self: HList,
    Tail: RemoveImpl<T, PathTail>,
    PathTail: ValidPath,
    Cons<Head, Tail::Remove>: HList,
{
    type Remove = Cons<Head, Tail::Remove>;

    fn remove_impl(self) -> Self::Remove {
        Cons(self.0, self.1.remove_impl())
    }
}

impl<T, Tail> RemoveImpl<T, Cons<Here, Nil>> for Cons<T, Tail>
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
    use crate::t_funk::{
        hlist::{remove::Remove, Cons, Nil},
        tlist::ToHList,
    };

    #[test]
    fn test_hlist_remove() {
        let list: Cons<usize, Cons<f32, Cons<&str, Nil>>> = (1, 2.0, "three").to_hlist();
        let list: Cons<usize, Cons<f32, Nil>> = list.remove::<&str>();
        let list: Cons<usize, Nil> = list.remove::<f32>();
        let list: Nil = list.remove::<usize>();
        assert_eq!(list, Nil);
    }
}
