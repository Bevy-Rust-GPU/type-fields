/// Concrete tuple implementations for HLists and consable types.
use crate::{
    macro_cons,
    t_funk::{
        hlist::ToTList,
        tlist::{AsHListMut, AsHListRef, ToHList},
    },
};

/// Implement `Cons` over a tuple with the provided generic types
macro_rules! impl_cons_tuple {
    ($($tys:ident),+) => {
        impl < $($tys),+ > ToHList for ($($tys,)+) {
            type HList = crate::cons_ty!($($tys),+);

            fn to_hlist(self) -> Self::HList {
                #[allow(non_snake_case)]
                let ($($tys,)+) = self;
                crate::cons_ident!($($tys),+)
            }
        }

        impl<$($tys),+> AsHListRef for ($($tys,)+)
        {
            type HListRef<'a> = crate::cons_ty!($(&'a $tys),+) where $($tys: 'a),+;

            fn as_hlist_ref<'a>(&'a self) -> Self::HListRef<'a> {
                #[allow(non_snake_case)]
                let ($(ref $tys,)+) = self;
                crate::cons_ident!($($tys),+)
            }
        }

        impl<$($tys),+> AsHListMut for ($($tys,)+)
        {
            type HListMut<'a> = crate::cons_ty!($(&'a mut $tys),+) where $($tys: 'a),+;

            fn as_hlist_mut<'a>(&'a mut self) -> Self::HListMut<'a> {
                #[allow(non_snake_case)]
                let ($(ref mut $tys,)+) = self;
                crate::cons_ident!($($tys),+)
            }
        }

        impl<$($tys),*> ToTList for crate::cons_ident!($($tys),*) {
            type TList = ($($tys,)*);

            fn to_tlist(self) -> Self::TList {
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

    use crate::t_funk::{
        hlist::ToTList,
        tlist::{AsHListMut, AsHListRef, ToHList},
    };

    macro_rules! test_cons_tuple {
        ($($exprs:expr),+) => {
            let mut consable = ($($exprs,)*);
            let _cons = ($($exprs,)*).to_hlist();
            let _cons_ref = consable.as_hlist_ref();
            let _cons_mut = consable.as_hlist_mut();
            let _ = consable.to_hlist().to_tlist();
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
