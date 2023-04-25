use type_fields_macros::functions;

#[functions]
pub trait Fanout<F> {
    type Fanout;

    fn fanout(self, f: F) -> Self::Fanout;
}
