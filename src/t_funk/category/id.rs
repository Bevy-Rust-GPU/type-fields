use type_fields_macros::functions;

#[functions]
pub trait Id {
    type Id;

    fn id() -> Self::Id;
}
