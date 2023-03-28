/// Type-level path
pub trait Path<T> {
    type Type;

    fn field(self, t: &mut T) -> &mut Self::Type;
}

/// Fn type blanket impl.
/// Allows mutable getters to be treated as path segments.
impl<F, T, O> Path<T> for F
where
    F: Fn(&mut T) -> &mut O,
{
    type Type = O;

    fn field(self, t: &mut T) -> &mut Self::Type {
        self(t)
    }
}

/// Cons cell blanket impls.
/// Allows nested cons cells to be treated as a path.
impl<LHS, RHS, T> Path<T> for (LHS, RHS)
where
    LHS: Path<T>,
    RHS: Path<LHS::Type>,
    LHS::Type: 'static,
{
    type Type = <RHS as Path<LHS::Type>>::Type;

    fn field(self, t: &mut T) -> &mut Self::Type {
        let (lhs, rhs) = self;
        rhs.field(lhs.field(t))
    }
}

impl<T> Path<T> for () {
    type Type = T;

    fn field(self, t: &mut T) -> &mut Self::Type {
        t
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
