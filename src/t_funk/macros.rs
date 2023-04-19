/*
#[macro_export]
macro_rules! derive_closure {
    ($ident:ident $(<>)?) => {
        impl<_Input> crate::t_funk::Closure<_Input> for $ident where $ident: crate::t_funk::Function<_Input> {
            type Output = <$ident as crate::t_funk::Function<_Input>>::Output;

            fn call(self, input: _Input) -> Self::Output {
                <$ident as crate::t_funk::Function<_Input>>::call(input)
            }
        }
    };
    ($ident:ident < $($tys:ident),+ >) => {
        impl<_Input, $($tys),+> crate::t_funk::Closure<_Input> for $ident < $($tys),+ > where $ident < $($tys),+ >: crate::t_funk::Function<_Input> {
            type Output = <$ident < $($tys),+ > as crate::t_funk::Function<_Input>>::Output;

            fn call(self, input: _Input) -> Self::Output {
                <$ident < $($tys),+ > as crate::t_funk::Function<_Input>>::call(input)
            }
        }
    };
}
*/

#[macro_export]
macro_rules! derive_pointed {
    ($ident:ident < $ty:ident >) => {
        impl<$ty> crate::t_funk::Pointed for $ident<$ty> {
            type Pointed = $ty;

            fn point(unit: Self::Pointed) -> Self {
                $ident(unit)
            }
        }
    };
    ($ident:ident < $($ty:ident),+ >) => {
        impl<$($ty),+> crate::t_funk::Pointed for $ident<$($ty),+> {
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
        impl<$ty> crate::t_funk::Copointed for $ident<$ty> {
            type Copointed = $ty;

            fn copoint(self) -> Self::Copointed {
                self.0
            }
        }
    };
    ($ident:ident < $($ty:ident),+ >) => {
        impl<$($ty),+> crate::t_funk::Copointed for $ident<$($ty),+> {
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
        impl<$ty, _Function> crate::t_funk::Fmap<_Function> for $ident<$ty>
        where
            _Function: crate::t_funk::Closure<$ty>,
        {
            type Fmap = $ident<_Function::Output>;

            fn fmap(self, f: _Function) -> Self::Fmap {
                crate::t_funk::Pointed::point(
                    f.call(crate::t_funk::Copointed::copoint(self)),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! derive_applicative {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, _Value> crate::t_funk::Apply<$ident<_Value>> for $ident<$ty>
        where
            $ty: crate::t_funk::Closure<_Value>,
        {
            type Apply = $ident<$ty::Output>;

            fn apply(self, a: $ident<_Value>) -> Self::Apply
            where
                $ty: crate::t_funk::Closure<_Value>,
            {
                crate::t_funk::Pointed::point(
                    crate::t_funk::Copointed::copoint(self)
                        .call(crate::t_funk::Copointed::copoint(a)),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! derive_monad {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, _Function> crate::t_funk::Chain<_Function> for $ident<$ty>
        where
            _Function: crate::t_funk::Closure<$ty>,
        {
            type Chain = _Function::Output;

            fn chain(self, f: _Function) -> Self::Chain {
                f.call(crate::t_funk::Copointed::copoint(self))
            }
        }
    };
}

#[macro_export]
macro_rules! derive_monoid {
    ($ident:ident < $ty:ident >) => {
        impl<$ty> crate::t_funk::Mempty for $ident<$ty>
        where
            $ty: crate::t_funk::Mempty,
        {
            type Mempty = $ident<$ty::Mempty>;

            fn mempty() -> Self::Mempty {
                crate::t_funk::Pointed::point($ty::mempty())
            }
        }
    };
}

#[macro_export]
macro_rules! derive_semigroup {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, U> crate::t_funk::Mappend<$ident<U>> for $ident<$ty>
        where
            $ty: crate::t_funk::Mappend<U>,
        {
            type Mappend = $ident<$ty::Mappend>;

            fn mappend(self, t: $ident<U>) -> Self::Mappend {
                crate::t_funk::Pointed::point(
                    self.0.mappend(crate::t_funk::Copointed::copoint(t)),
                )
            }
        }
    };
}
