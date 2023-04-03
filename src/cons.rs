/// Takes a flat list of identifiers and nests it into a cons list.
///
/// Cons lists are useful for recursive trait evaluation.
macro_rules! cons_ident {
    ($ty:ident $(, $tys:ident)+) => {
        ($ty, cons_ident!($($tys),*))
    };
    ($ty:ident) => {
        ($ty, ())
    };
    () => {};
}

macro_rules! cons_expr {
    ($ty:expr $(, $tys:expr)+) => {
        ($ty, cons_expr!($($tys),*))
    };
    ($ty:expr) => {
        ($ty, ())
    };
    () => {};
}

macro_rules! cons_ty {
    ($ty:ty $(, $tys:ty)+) => {
        ($ty, cons_ty!($($tys),*))
    };
    ($ty:ty) => {
        ($ty, ())
    };
    () => {};
}

pub trait ConsLength {
    const LENGTH: usize;
}

impl<LHS, RHS> ConsLength for (LHS, RHS)
where
    RHS: ConsLength,
{
    const LENGTH: usize = RHS::LENGTH + 1;
}

impl<LHS> ConsLength for (LHS, ()) {
    const LENGTH: usize = 1;
}

/// Convert a flat tuple into a cons list.
/// ex. `(1, 2, 3, 4)` -> `(1, (2, (3, (4, ()))))`
pub trait Cons {
    type Cons: ConsLength + Uncons;

    fn cons(self) -> Self::Cons;
}

/// Immutably borrow a cons list from a flat tuple.
/// ex. `&(1, 2, 3, 4)` -> `(&1, (&2, (&3, (&4, ()))))`
pub trait ConsRef<'a> {
    type ConsRef: ConsLength + Uncons;

    fn cons_ref(&'a self) -> Self::ConsRef;
}

/// Mutably borrow a cons list from a flat tuple.
/// ex. `&mut (1, 2, 3, 4)` -> `(&1, (&2, (&3, (&4, ()))))`
pub trait ConsMut<'a> {
    type ConsMut: ConsLength + Uncons;

    fn cons_mut(&'a mut self) -> Self::ConsMut;
}

/// Convert a cons list into a flat tuple.
/// ex. `(1, (2, (3, (4, ())))) -> (1, 2, 3, 4)`
pub trait Uncons {
    type Uncons;

    fn uncons(self) -> Self::Uncons;
}

/// Create a cons list of default values from a flat tuple.
/// ex. `(usize, f32, &'static str) -> (0, (0.0, ("", ())))`
pub trait ConsDefault: Cons {
    fn cons_default() -> Self::Cons;
}

/// Create a cons list of cloned values from a flat tuple.
/// ex. `&(1, 2.0, "three") -> (1, (2.0, ("three", ())))`
pub trait ConsClone: Cons {
    fn cons_clone(&self) -> Self::Cons;
}

/// Implement `Cons` over a tuple with the provided generic types
macro_rules! impl_cons {
    ($($tys:ident),+) => {
        impl < $($tys),+ > Cons for ($($tys,)+) {
            type Cons = cons_ty!($($tys),+);

            fn cons(self) -> Self::Cons {
                #[allow(non_snake_case)]
                let ($($tys,)+) = self;
                cons_ident!($($tys),+)
            }
        }

        impl<'a, $($tys),+> ConsRef<'a> for ($($tys,)+)
        where
            $($tys: 'a),*
        {
            type ConsRef = cons_ty!($(&'a $tys),+);

            fn cons_ref(&'a self) -> Self::ConsRef {
                #[allow(non_snake_case)]
                let ($(ref $tys,)+) = self;
                cons_ident!($($tys),+)
            }
        }

        impl<'a, $($tys),+> ConsMut<'a> for ($($tys,)+)
        where
            $($tys: 'a),*
        {
            type ConsMut = cons_ty!($(&'a $tys),+);

            fn cons_mut(&'a mut self) -> Self::ConsMut {
                #[allow(non_snake_case)]
                let ($(ref mut $tys,)+) = self;
                cons_ident!($($tys),+)
            }
        }

        impl<$($tys),*> Uncons for cons_ident!($($tys),*) {
            type Uncons = ($($tys,)*);

            fn uncons(self) -> Self::Uncons {
                #[allow(non_snake_case)]
                let cons_ident!($($tys),*) = self;
                ($($tys,)*)
            }
        }

        impl<$($tys),+> ConsDefault for ($($tys,)+)
        where
            Self: Cons<Cons = cons_ty!($($tys),+)>,
            $($tys: Default),*
        {
            fn cons_default() -> Self::Cons {
                cons_expr!($($tys::default()),+)
            }
        }

        impl<$($tys),+> ConsClone for ($($tys,)+)
        where
            Self: Cons<Cons = cons_ty!($($tys),+)>,
            $($tys: Clone),*
        {
            fn cons_clone(&self) -> Self::Cons {
                #[allow(non_snake_case)]
                let ($(ref $tys,)+) = self;
                cons_expr!($($tys.clone()),+)
            }
        }
    }
}

// Static impls over tuples of arity 1..=12
impl_cons!(A);
impl_cons!(A, B);
impl_cons!(A, B, C);
impl_cons!(A, B, C, D);
impl_cons!(A, B, C, D, E);
impl_cons!(A, B, C, D, E, F);
impl_cons!(A, B, C, D, E, F, G);
impl_cons!(A, B, C, D, E, F, G, H);
impl_cons!(A, B, C, D, E, F, G, H, I);
impl_cons!(A, B, C, D, E, F, G, H, I, J);
impl_cons!(A, B, C, D, E, F, G, H, I, J, K);
impl_cons!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
pub mod test {
    use super::{Cons, Uncons};

    #[test]
    fn test_cons() {
        let c = (1,).cons();
        let _ = c.uncons();

        let c = (1, 2).cons();
        let _ = c.uncons();

        let c = (1, 2, 3).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4, 5).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4, 5, 6).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4, 5, 6, 7).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4, 5, 6, 7, 8).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4, 5, 6, 7, 8, 9).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11).cons();
        let _ = c.uncons();

        let c = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).cons();
        let _ = c.uncons();
    }
}
