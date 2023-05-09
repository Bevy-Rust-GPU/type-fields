use crate::{
    macros::{monad::Then, Closure},
    t_funk::{monad::{Chain, ChainF}, Closure, Curry2, Fmap, Function, Curry2B},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Then)]
pub struct Free<F>(pub F);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Then)]
pub struct Return<F>(pub F);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct MakeReturn;

impl<T> Function<T> for MakeReturn {
    type Output = Return<T>;

    fn call(input: T) -> Self::Output {
        Return(input)
    }
}

impl<T, F> Chain<F> for Free<T>
where
    T: Fmap<Curry2B<ChainF, F>>,
{
    type Chain = Free<<T as Fmap<Curry2B<ChainF, F>>>::Fmap>;

    fn chain(self, f: F) -> Self::Chain {
        Free(self.0.fmap(ChainF.suffix(f)))
    }
}

impl<T, F> Chain<F> for Return<T>
where
    F: Closure<T>,
{
    type Chain = F::Output;

    fn chain(self, f: F) -> Self::Chain {
        f.call(self.0)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct LiftFree;

impl<T> Function<T> for LiftFree
where
    T: Fmap<MakeReturn>,
{
    type Output = Free<T::Fmap>;

    fn call(input: T) -> Self::Output {
        Free(input.fmap(MakeReturn))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        do_monad,
        macros::Closure,
        t_funk::{
            closure::Compose, function::Id, monad::Chain, Closure, Composed, Fmap, Free, Function,
            LiftFree, MakeReturn, Then,
        },
    };

    use super::Return;

    // DSL
    type String = &'static str;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    pub struct Get<F>(pub String, pub F);

    impl<T, F> Fmap<F> for Get<T> {
        type Fmap = Get<Composed<F, T>>;

        fn fmap(self, f: F) -> Self::Fmap {
            Get(self.0, f.compose(self.1))
        }
    }

    fn get(key: String) -> Free<Get<Composed<MakeReturn, Id>>> {
        LiftFree.call(Get(key, Id))
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    pub struct Set<T>(pub String, pub String, pub T);

    impl<T, F> Fmap<F> for Set<T>
    where
        F: Closure<T>,
    {
        type Fmap = Set<F::Output>;

        fn fmap(self, f: F) -> Self::Fmap {
            Set(self.0, self.1, f.call(self.2))
        }
    }

    fn set(key: String, value: String) -> Free<Set<Return<()>>> {
        LiftFree.call(Set(key, value, ()))
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    pub struct End;

    impl<F> Fmap<F> for End {
        type Fmap = End;

        fn fmap(self, _: F) -> Self::Fmap {
            End
        }
    }

    fn end() -> Free<End> {
        LiftFree.call(End)
    }

    // Concrete interpreter
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
    struct RunIo;

    impl<T> Function<Free<Get<T>>> for RunIo
    where
        T: Closure<String>,
        RunIo: Closure<T::Output>,
    {
        type Output = <RunIo as Closure<T::Output>>::Output;

        fn call(Free(Get(_key, next)): Free<Get<T>>) -> Self::Output {
            //panic!("Get {}", key);
            RunIo.call(next.call("foo"))
        }
    }

    impl<T> Function<Free<Set<T>>> for RunIo
    where
        RunIo: Closure<T>,
    {
        type Output = <RunIo as Closure<T>>::Output;

        fn call(Free(Set(_key, _value, next)): Free<Set<T>>) -> Self::Output {
            //panic!("Set {} to {}", key, value);
            RunIo.call(next)
        }
    }

    impl Function<Free<End>> for RunIo {
        type Output = ();

        fn call(_: Free<End>) -> Self::Output {
            //panic!("End");
        }
    }

    impl<T> Function<Return<T>> for RunIo
    where
        T: core::fmt::Debug,
    {
        type Output = Free<()>;

        fn call(_: Return<T>) -> Self::Output {
            Free(())
        }
    }

    #[test]
    fn foo() {
        let mon = do_monad! {
            foo <- get("foo");
            set("bar", foo);
            end()
        };

        let _zap = RunIo.call(mon);
    }
}
