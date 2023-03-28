/// Takes a flat list of identifiers and nests it into a cons list.
///
/// Cons lists are useful for recursive trait evaluation.
macro_rules! cons {
    ($ty:ident $(, $tys:ident)+) => {
        ($ty, cons!($($tys),*))
    };
    ($ty:ident) => {
        ($ty, ())
    };
    () => {};
}

/// Convert a flat tuple into a cons list.
/// ex. `(1, 2, 3, 4)` -> `(1, (2, (3, (4, ()))))`
pub trait Cons {
    type Cons;

    fn cons(self) -> Self::Cons;
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

// Static `Cons` impls over tuples of arity 1..=12
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

/// Convert a cons list into a flat tuple.
/// ex. `(1, (2, (3, (4, ())))) -> (1, 2, 3, 4)`
pub trait Uncons<const COUNT: usize> {
    type Uncons;

    fn uncons(self) -> Self::Uncons;
}

/// Implement `Uncons` over a tuple with the provided generic types
macro_rules! impl_uncons {
    ($idx:expr, $($tys:ident),*) => {
        impl<$($tys),*> Uncons<$idx> for cons!($($tys),*) {
            type Uncons = ($($tys,)*);

            fn uncons(self) -> Self::Uncons {
                #[allow(non_snake_case)]
                let cons!($($tys),*) = self;
                ($($tys,)*)
            }
        }
    };
}

// Static `Uncons` impls over tuples of arity 1..=12
impl_uncons!(1, A);
impl_uncons!(2, A, B);
impl_uncons!(3, A, B, C);
impl_uncons!(4, A, B, C, D);
impl_uncons!(5, A, B, C, D, E);
impl_uncons!(6, A, B, C, D, E, F);
impl_uncons!(7, A, B, C, D, E, F, G);
impl_uncons!(8, A, B, C, D, E, F, G, H);
impl_uncons!(9, A, B, C, D, E, F, G, H, I);
impl_uncons!(10, A, B, C, D, E, F, G, H, I, J);
impl_uncons!(11, A, B, C, D, E, F, G, H, I, J, K);
impl_uncons!(12, A, B, C, D, E, F, G, H, I, J, K, L);

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
