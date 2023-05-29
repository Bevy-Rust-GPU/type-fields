use crate::t_funk::{Fmap, FoldMap, Mconcat};

use super::Cons;

impl<Head, Tail, _Function> FoldMap<_Function> for Cons<Head, Tail>
where
    Cons<Head, Tail>: Fmap<_Function>,
    <Cons<Head, Tail> as Fmap<_Function>>::Fmap: Mconcat,
{
    type FoldMap = <<Cons<Head, Tail> as Fmap<_Function>>::Fmap as Mconcat>::Mconcat;

    fn fold_map(self, f: _Function) -> Self::FoldMap {
        self.fmap(f).mconcat()
    }
}
