use crate::{path::Path, t_funk::tlist::ToHList};

/// Type-level field access using [`Path`]
pub trait Field<V, P>: Sized
where
    P: FieldPath<V, Self>,
{
    fn get(&mut self, path: P) -> &P::Type {
        path.field(self)
    }

    fn get_mut(&mut self, path: P) -> &P::Type {
        path.field(self)
    }

    fn set(&mut self, path: P, value: P::Type) {
        *path.field(self) = value;
    }

    fn with(mut self, path: P, value: P::Type) -> Self {
        self.set(path, value);
        self
    }

    fn map<F>(mut self, path: P, f: F) -> Self
    where
        F: Fn(&mut P::Type),
    {
        f(path.field(&mut self));
        self
    }
}

/// Blanket impl
impl<V, T, P> Field<V, P> for T where P: FieldPath<V, T> + 'static {}

/// Marker type for [`Cons`] impls
pub enum Cell {}

/// Marker type for [`Fn`] impls
pub enum Leaf {}

/// Composes [`Path`] impls over [`Cons`] and [`Fn`]
pub trait FieldPath<V, T> {
    type Type;

    fn field(self, t: &mut T) -> &mut Self::Type;
}

/// Leaf [`Fn`] type blanket impl
impl<F, T, O> FieldPath<Leaf, T> for F
where
    F: Fn(&mut T) -> &mut O,
{
    type Type = O;

    fn field(self, t: &mut T) -> &mut Self::Type {
        self(t)
    }
}

/// Cell [`Cons`] type blanket impl
impl<C, T> FieldPath<Cell, T> for C
where
    C: ToHList,
    <C as ToHList>::HList: Path<T>,
{
    type Type = <<C as ToHList>::HList as Path<T>>::Type;

    fn field(self, t: &mut T) -> &mut Self::Type {
        self.to_hlist().field(t)
    }
}
