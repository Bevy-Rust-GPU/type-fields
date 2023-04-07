/// Concrete tuple implementations for cons lists and consable types.
use crate::{
    hlist::{
        cons::Uncons,
        tuple::{Cons, ConsMut, ConsRef},
    },
    macro_cons,
};

/// Implement `Cons` over a tuple with the provided generic types
macro_rules! impl_cons_tuple {
    ($($tys:ident),+) => {
        impl < $($tys),+ > Cons for ($($tys,)+) {
            type Cons = crate::cons_ty!($($tys),+);

            fn cons(self) -> Self::Cons {
                #[allow(non_snake_case)]
                let ($($tys,)+) = self;
                crate::cons_ident!($($tys),+)
            }
        }

        impl<$($tys),+> ConsRef for ($($tys,)+)
        {
            type ConsRef<'a> = crate::cons_ty!($(&'a $tys),+) where $($tys: 'a),+;

            fn cons_ref<'a>(&'a self) -> Self::ConsRef<'a> {
                #[allow(non_snake_case)]
                let ($(ref $tys,)+) = self;
                crate::cons_ident!($($tys),+)
            }
        }

        impl<$($tys),+> ConsMut for ($($tys,)+)
        {
            type ConsMut<'a> = crate::cons_ty!($(&'a mut $tys),+) where $($tys: 'a),+;

            fn cons_mut<'a>(&'a mut self) -> Self::ConsMut<'a> {
                #[allow(non_snake_case)]
                let ($(ref mut $tys,)+) = self;
                crate::cons_ident!($($tys),+)
            }
        }

        impl<$($tys),*> Uncons for crate::cons_ident!($($tys),*) {
            type Uncons = ($($tys,)*);

            fn uncons(self) -> Self::Uncons {
                #[allow(non_snake_case)]
                let crate::cons_ident!($($tys),*) = self;
                ($($tys,)*)
            }
        }

    }
}

// Static impls
macro_cons!(impl_cons_tuple!, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

#[cfg(test)]
pub mod test {
    use crate::macro_cons;

    use super::{Cons, ConsMut, ConsRef, Uncons};

    macro_rules! test_cons_tuple {
        ($($exprs:expr),+) => {
            let mut consable = ($($exprs,)*);
            let _cons = ($($exprs,)*).cons();
            let _cons_ref = consable.cons_ref();
            let _cons_mut = consable.cons_mut();
            let _ = consable.cons().uncons();
        };
    }

    #[test]
    fn test_cons() {
        macro_cons!(
            test_cons_tuple!, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26
        );
    }
}
