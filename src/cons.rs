/// Convert a flat tuple into a nested tuple.
/// ex. `(1, 2, 3, 4)` -> `(1, (2, (3, 4)))`
///
/// Useful for traversing heterogeneous lists at type time via trait.
pub trait Cons {
    type Cons;

    fn cons(self) -> Self::Cons;
}

/// Takes a flat list of identifiers and composes it into a nested tuple
macro_rules! cons {
    ($ty:ident $(, $tys:ident)+) => {
        ($ty, cons!($($tys),*))
    };
    ($ty:ident) => {
        $ty
    };
    () => {};
}

/// Implement `Cons` over a tuple with the provided generic types
macro_rules! impl_cons {
    ($($tys:ident),+) => {
        impl < $($tys),+ > Cons for ($($tys,)+) {
            type Cons = cons!($($tys),+);

            fn cons(self) -> Self::Cons {
                #[allow(non_snake_case)]
                let ($($tys,)+) = self;
                cons!($($tys),+)
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
