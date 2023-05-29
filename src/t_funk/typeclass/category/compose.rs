use crate::macros::functions;

/// Right-to-left composition
/// (. or <<<)
#[functions]
pub trait Compose<F>: Sized {
    type Compose;
    fn compose(self, f: F) -> Self::Compose;
}

pub type ComposeT<T, U> = <T as Compose<U>>::Compose;

/// Left-to-right composition
/// (>>>)
#[functions]
pub trait ComposeL<F>: Sized {
    type ComposeL;
    fn compose_l(self, f: F) -> Self::ComposeL;
}

pub type ComposeLT<T, U> = <T as ComposeL<U>>::ComposeL;

impl<T, F> ComposeL<F> for T
where
    F: Compose<T>,
{
    type ComposeL = F::Compose;

    fn compose_l(self, f: F) -> Self::ComposeL {
        f.compose(self)
    }
}
