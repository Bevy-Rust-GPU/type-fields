use type_fields_macros::functions;

use crate::t_funk::Replace;

/// A type that can emplace itself within a functor
#[functions]
pub trait FunctorEmplace<F>: Sized {
    /// `$>`
    fn emplace(self, f: F) -> F::Fmap
    where
        F: Replace<Self>,
    {
        f.replace(self)
    }
}

impl<T, F> FunctorEmplace<F> for T {}

/*
pub struct EmplaceF;
impl<_Type, F> type_fields::t_funk::Function<(_Type, F)> for EmplaceF
where
    _Type: FunctorEmplace<F>,
{
    type Output = F::Fmap;
    #[allow(non_snake_case)]
    fn call((t, F): (_Type, F)) -> Self::Output {
        t.emplace(F)
    }
}
type_fields::derive_closure!(EmplaceF);
*/
