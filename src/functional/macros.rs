#[macro_export]
macro_rules! derive_pointed {
    ($ident:ident < $ty:ident >) => {
        impl<$ty> crate::functional::Pointed for $ident<$ty> {
            type Pointed = $ty;

            fn point(unit: Self::Pointed) -> Self {
                $ident(unit)
            }
        }
    };
    ($ident:ident < $($ty:ident),+ >) => {
        impl<$($ty),+> crate::functional::Pointed for $ident<$($ty),+> {
            type Pointed = ($($ty),+);

            fn point(unit: Self::Pointed) -> Self {
                #[allow(non_snake_case)]
                let ($($ty),+) = unit;
                $ident($($ty),+)
            }
        }
    };
}

#[macro_export]
macro_rules! derive_copointed {
    ($ident:ident < $ty:ident >) => {
        impl<$ty> crate::functional::Copointed for $ident<$ty> {
            type Copointed = $ty;

            fn copoint(self) -> Self::Copointed {
                self.0
            }
        }
    };
    ($ident:ident < $($ty:ident),+ >) => {
        impl<$($ty),+> crate::functional::Copointed for $ident<$($ty),+> {
            type Copointed = ($($ty),+);

            fn copoint(self) -> Self::Copointed {
                #[allow(non_snake_case)]
                let $ident($($ty),+) = self;
                ($($ty),+)
            }
        }
    };
}

#[macro_export]
macro_rules! derive_functor {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, F> crate::functional::Functor<F> for $ident<$ty>
        where
            F: crate::functional::Function<$ty>,
        {
            type Mapped = $ident<F::Output>;

            fn fmap(self, f: F) -> Self::Mapped {
                crate::functional::Pointed::point(
                    f.call(crate::functional::Copointed::copoint(self)),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! derive_applicative {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, _Value> crate::functional::Applicative<_Value> for $ident<$ty>
        where
            $ty: crate::functional::Function<_Value>,
        {
            type Applied = $ty::Output;

            fn apply(self, a: _Value) -> Self::Applied
            where
                $ty: crate::functional::Function<_Value>,
            {
                crate::functional::Copointed::copoint(self).call(a)
            }
        }
    };
}

#[macro_export]
macro_rules! derive_monad {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, _Function> crate::functional::Monad<_Function> for $ident<$ty>
        where
            _Function: crate::functional::Function<$ty>,
        {
            type Chained = _Function::Output;

            fn chain(self, f: _Function) -> Self::Chained {
                f.call(crate::functional::Copointed::copoint(self))
            }
        }
    };
}

#[macro_export]
macro_rules! derive_monoid {
    ($ident:ident < $ty:ident >) => {
        impl<$ty> crate::functional::Monoid for $ident<$ty>
        where
            $ty: crate::functional::Monoid,
        {
            type Identity = $ident<$ty::Identity>;

            fn mempty() -> Self::Identity {
                crate::functional::Pointed::point($ty::mempty())
            }
        }
    };
}

#[macro_export]
macro_rules! derive_semigroup {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, U> crate::functional::Semigroup<$ident<U>> for $ident<$ty>
        where
            $ty: crate::functional::Semigroup<U>,
        {
            type Appended = $ident<$ty::Appended>;

            fn mappend(self, t: $ident<U>) -> Self::Appended {
                crate::functional::Pointed::point(
                    self.0.mappend(crate::functional::Copointed::copoint(t)),
                )
            }
        }
    };
}
