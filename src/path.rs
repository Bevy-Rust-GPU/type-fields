/// Type-level path
pub trait Path<T> {
    type Type;

    fn field(t: &mut T) -> &mut Self::Type;
}

impl<LHS, RHS, T> Path<T> for (LHS, RHS)
where
    LHS: Path<T>,
    RHS: Path<LHS::Type>,
    LHS::Type: 'static,
{
    type Type = <RHS as Path<LHS::Type>>::Type;

    fn field(t: &mut T) -> &mut Self::Type {
        <RHS as Path<LHS::Type>>::field(<LHS as Path<T>>::field(t))
    }
}

#[macro_export]
macro_rules ! path {
    ($ident:ident . $($next:ident).*) => {
        (path!($ident), path!($($next).*))
    };
    ($ident:ident) => {
        $ident
    };
}

