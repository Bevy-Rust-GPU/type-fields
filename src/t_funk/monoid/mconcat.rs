use type_fields_macros::functions;

#[functions]
pub trait Mconcat: Sized {
    type Mconcat;

    fn mconcat(self) -> Self::Mconcat;
}

