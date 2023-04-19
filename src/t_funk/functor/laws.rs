use crate::t_funk::{Closure, Compose, Composed, Id};

use super::Fmap;

pub fn test_functor_laws<F, F1, F2, G>(f: F, f1: F1, f2: F2)
where
    F: core::fmt::Debug
        + Clone
        + PartialEq
        + Fmap<Id, Fmap = F>
        + Fmap<F1>
        + Fmap<F2>
        + Fmap<Composed<F1, F2>, Fmap = G>,
    <F as Fmap<Composed<F1, F2>>>::Fmap: core::fmt::Debug,
    <F as Fmap<F1>>::Fmap: core::fmt::Debug + Fmap<F2, Fmap = G>,
    <<F as Fmap<F1>>::Fmap as Fmap<F2>>::Fmap: core::fmt::Debug + PartialEq,
    F1: Clone,
    F2: Clone,
{
    test_functor_identity(f.clone());
    test_functor_composition(f, f1, f2)
}

pub fn test_functor_identity<F>(f: F)
where
    F: core::fmt::Debug + Clone + Fmap<Id, Fmap = F>,
    F::Fmap: core::fmt::Debug + PartialEq,
{
    assert_eq!(f.clone().fmap(Id), Closure::call(Id, f));
}

pub fn test_functor_composition<F, F1, F2, G>(f: F, f1: F1, f2: F2)
where
    F: Clone + Fmap<F1> + Fmap<F2> + Fmap<Composed<F1, F2>, Fmap = G>,
    F1: Clone,
    F2: Clone,
    <F as Fmap<F1>>::Fmap: Fmap<F2, Fmap = G>,
    <<F as Fmap<F1>>::Fmap as Fmap<F2>>::Fmap: core::fmt::Debug + PartialEq,
    <F as Fmap<Composed<F1, F2>>>::Fmap: core::fmt::Debug,
{
    assert_eq!(
        f.clone().fmap(f1.clone()).fmap(f2.clone()),
        f.fmap(f1.compose(f2))
    )
}

