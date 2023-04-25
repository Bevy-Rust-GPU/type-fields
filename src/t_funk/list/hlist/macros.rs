/// Macros for composing HList literals.

/// Takes a flat list of tokens and nests it into a HList.
#[macro_export]
macro_rules! cons {
    ($($exprs:expr),*) => {
        crate::cons_expr!($($exprs),*)
    };
    ($($tys:ty),*) => {
        crate::cons_ty!($($tys),*)
    };
    ($($idents:ident),*) => {
        crate::cons_ident!($($idents),*)
    };
}

/// Takes a flat list of identifiers and nests it into a HList.
#[macro_export]
macro_rules! cons_ident {
    ($ident:ident $(, $idents:ident)+) => {
        ($ident, crate::cons_ident!($($idents),*))
    };
    ($ident:ident) => {
        ($ident, ())
    };
    () => {};
}

/// Takes a flat list of expressions and nests it into a HList.
#[macro_export]
macro_rules! cons_expr {
    ($expr:expr $(, $exprs:expr)+) => {
        ($expr, crate::cons_expr!($($exprs),*))
    };
    ($expr:expr) => {
        ($expr, ())
    };
    () => {};
}

/// Takes a flat list of types and nests it into a HList.
#[macro_export]
macro_rules! cons_ty {
    ($ty:ty $(, $tys:ty)+) => {
        ($ty, crate::cons_ty!($($tys),*))
    };
    ($ty:ty) => {
        ($ty, ())
    };
    () => {};
}

/// Calls the provided macro once for each sub-list in the provided list.
#[macro_export]
macro_rules! macro_cons {
    ($call:ident !, $ident:tt, $($idents:tt),+) => {
        $call!($ident, $($idents),+);
        macro_cons!($call !, $($idents),+);
    };
    ($call:ident !, $ident:tt) => {
        $call!($ident);
    }
}

