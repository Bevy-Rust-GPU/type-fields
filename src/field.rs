use crate::cons::Cons;

use super::path::Path;

pub trait FieldPath {
    type Type;
}

impl<T> FieldPath for T
where
    T: Cons,
{
    type Type = T::Cons;
}

/// Type-level field access using [`Path`]
pub trait Field<P>: Sized
where
    P: FieldPath,
    P::Type: Path<Self>,
{
    fn get(&mut self, _: P) -> &<P::Type as Path<Self>>::Type {
        P::Type::field(self)
    }

    fn get_mut(&mut self, _: P) -> &<P::Type as Path<Self>>::Type {
        P::Type::field(self)
    }

    fn set(&mut self, _: P, value: <P::Type as Path<Self>>::Type) {
        *P::Type::field(self) = value;
    }

    fn with(mut self, path: P, value: <P::Type as Path<Self>>::Type) -> Self {
        self.set(path, value);
        self
    }

    fn map<F>(mut self, _: P, f: F) -> Self
    where
        F: Fn(&mut <P::Type as Path<Self>>::Type),
    {
        f(P::Type::field(&mut self));
        self
    }
}

impl<T, P> Field<P> for T
where
    P: FieldPath,
    P::Type: Path<T>,
{
}
