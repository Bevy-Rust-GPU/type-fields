use type_fields_macros::functions;

#[functions]
pub trait Arr {
    type Arr;

    fn arr(self) -> Self::Arr;
}

impl<T> Arr for T {
    type Arr = Self;

    fn arr(self) -> Self::Arr {
        self
    }
}
