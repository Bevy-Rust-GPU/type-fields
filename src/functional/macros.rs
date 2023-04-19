/// Derive `Closure<T>` for a type that implements `Function<T>`
#[macro_export]
macro_rules! derive_closure {
    ($ident:ident $(<>)?) => {
        impl<_Input> crate::functional::Closure<_Input> for $ident where $ident: crate::functional::Function<_Input> {
            type Output = <$ident as crate::functional::Function<_Input>>::Output;

            fn call(self, input: _Input) -> Self::Output {
                <$ident as crate::functional::Function<_Input>>::call(input)
            }
        }
    };
    ($ident:ident < $($tys:ident),+ >) => {
        impl<_Input, $($tys),+> crate::functional::Closure<_Input> for $ident < $($tys),+ > where $ident < $($tys),+ >: crate::functional::Function<_Input> {
            type Output = <$ident < $($tys),+ > as crate::functional::Function<_Input>>::Output;

            fn call(self, input: _Input) -> Self::Output {
                <$ident < $($tys),+ > as crate::functional::Function<_Input>>::call(input)
            }
        }
    };
}

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
        impl<$ty, _Function> crate::functional::Fmap<_Function> for $ident<$ty>
        where
            _Function: crate::functional::Closure<$ty>,
        {
            type Fmap = $ident<_Function::Output>;

            fn fmap(self, f: _Function) -> Self::Fmap {
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
        impl<$ty, _Value> crate::functional::Apply<$ident<_Value>> for $ident<$ty>
        where
            $ty: crate::functional::Closure<_Value>,
        {
            type Apply = $ident<$ty::Output>;

            fn apply(self, a: $ident<_Value>) -> Self::Apply
            where
                $ty: crate::functional::Closure<_Value>,
            {
                crate::functional::Pointed::point(
                    crate::functional::Copointed::copoint(self)
                        .call(crate::functional::Copointed::copoint(a)),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! derive_monad {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, _Function> crate::functional::Chain<_Function> for $ident<$ty>
        where
            _Function: crate::functional::Closure<$ty>,
        {
            type Chain = _Function::Output;

            fn chain(self, f: _Function) -> Self::Chain {
                f.call(crate::functional::Copointed::copoint(self))
            }
        }
    };
}

#[macro_export]
macro_rules! derive_monoid {
    ($ident:ident < $ty:ident >) => {
        impl<$ty> crate::functional::Mempty for $ident<$ty>
        where
            $ty: crate::functional::Mempty,
        {
            type Mempty = $ident<$ty::Mempty>;

            fn mempty() -> Self::Mempty {
                crate::functional::Pointed::point($ty::mempty())
            }
        }
    };
}

#[macro_export]
macro_rules! derive_semigroup {
    ($ident:ident < $ty:ident >) => {
        impl<$ty, U> crate::functional::Mappend<$ident<U>> for $ident<$ty>
        where
            $ty: crate::functional::Mappend<U>,
        {
            type Mappend = $ident<$ty::Mappend>;

            fn mappend(self, t: $ident<U>) -> Self::Mappend {
                crate::functional::Pointed::point(
                    self.0.mappend(crate::functional::Copointed::copoint(t)),
                )
            }
        }
    };
}
