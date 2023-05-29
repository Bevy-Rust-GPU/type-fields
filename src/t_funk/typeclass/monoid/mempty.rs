use crate::macros::functions;

#[functions]
pub trait Mempty {
    type Mempty;

    fn mempty() -> Self::Mempty;
}

#[macro_export]
macro_rules! impl_mempty {
    ($ty:ty) => {
        impl Mempty for $ty {
            type Mempty = $ty;

            fn mempty() -> Self::Mempty {
                Default::default()
            }
        }
    }
}

impl_mempty!(bool);
impl_mempty!(u8);
impl_mempty!(u16);
impl_mempty!(u32);
impl_mempty!(u64);
impl_mempty!(i8);
impl_mempty!(i16);
impl_mempty!(i32);
impl_mempty!(i64);
impl_mempty!(f32);
impl_mempty!(f64);

impl<'a> Mempty for &'a str {
    type Mempty = &'a str;

    fn mempty() -> Self::Mempty {
        Default::default()
    }
}
