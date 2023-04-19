use type_fields_macros::functions;

/// A type with a binary associative function.
#[functions]
pub trait Mappend<T> {
    type Mappend;

    fn mappend(self, t: T) -> Self::Mappend;
}

#[functions]
pub trait SemigroupConcat: Sized {
    type Concatenated;

    fn mconcat(self) -> Self::Concatenated;
}

