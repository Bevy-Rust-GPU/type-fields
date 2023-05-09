use super::{Cons, HList, Nil};

/// `Path` component referencing the next cell.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Next;

/// `Path` component referencing the current cell.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Here;

/// `Path` component referencing `Self`.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct This;

/// Path to an item in a HList, in the form `(Next, (Next, (Here, ())))`.
///
/// Practically, this is a HList that can only contain `Next` and `Here`.
pub trait Path {}
impl<PathNext> Path for Cons<Next, PathNext> where PathNext: Path {}
impl Path for Cons<Here, Nil> {}
impl Path for This {}

pub trait Paths: HList {
    type HeadPath: Path;
    type TailPath;
}

impl<T, N> Paths for Cons<T, N>
where
    Self: HList,
    T: Path,
    N: Paths,
{
    type HeadPath = T;
    type TailPath = N;
}

impl<T> Paths for Cons<T, Nil>
where
    Self: HList,
    T: Path,
{
    type HeadPath = T;
    type TailPath = Nil;
}
