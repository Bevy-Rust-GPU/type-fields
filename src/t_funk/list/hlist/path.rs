use super::HList;

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
impl<PathNext> Path for (Next, PathNext) where PathNext: Path {}
impl Path for (Here, ()) {}
impl Path for This {}

pub trait Paths: HList {
    type HeadPath: Path;
    type TailPath;
}

impl<Head, Tail> Paths for (Head, Tail)
where
    Self: HList,
    Head: Path,
    Tail: Paths,
{
    type HeadPath = Head;
    type TailPath = Tail;
}

impl<Head> Paths for (Head, ())
where
    Self: HList,
    Head: Path,
{
    type HeadPath = Head;
    type TailPath = ();
}
