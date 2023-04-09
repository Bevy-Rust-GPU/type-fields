/// A type that can wrap a value
pub trait Pointed
where
    Self: Sized,
{
    /// The wrapped value
    type Pointed;

    /// Wrap `Pointed` into `Self`
    fn of(unit: Self::Pointed) -> Self;
}
