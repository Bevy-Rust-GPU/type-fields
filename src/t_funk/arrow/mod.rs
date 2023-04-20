use crate::t_funk::Category;

mod arr {}
mod first {}
mod second {}
mod split {}
mod fanout {}

pub use arr::*;
pub use fanout::*;
pub use first::*;
pub use second::*;
pub use split::*;

trait Arrow<Of>: Category<Of> {
    type Arr<F>;
    type First;
    type Second;
    type Split<A>;
    type Fanout<A>;

    fn arr<F>(f: F) -> Self::Arr<F>;
    fn first(self) -> Self::First;
    fn second(self) -> Self::Second;

    /// ***
    fn split<A>(self, a: A) -> Self::Split<A>;

    /// &&&
    fn fanout<A>(self, a: A) -> Self::Fanout<A>;
}

impl<T, Of> Arrow<Of> for T
where
    T: Category<Of>,
{
    type Arr<F> = ();
    type First = ();
    type Second = ();
    type Split<A> = ();
    type Fanout<A> = ();

    fn arr<F>(_f: F) -> Self::Arr<F> {
        todo!()
    }

    fn first(self) -> Self::First {
        todo!()
    }

    fn second(self) -> Self::Second {
        todo!()
    }

    fn split<A>(self, _a: A) -> Self::Split<A> {
        todo!()
    }

    fn fanout<A>(self, _a: A) -> Self::Fanout<A> {
        todo!()
    }
}
