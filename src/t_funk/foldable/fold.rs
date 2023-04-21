use type_fields_macros::functions;

#[functions]
pub trait Fold {
    type Fold;

    fn fold(self) -> Self::Fold;
}
