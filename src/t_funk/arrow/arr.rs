pub trait Arr<F> {
    type Arr;

    fn arr(f: F) -> Self::Arr;
}
