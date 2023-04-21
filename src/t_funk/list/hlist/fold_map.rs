use crate::t_funk::{Fmap, FoldMap, Mconcat};

impl<Head, Tail, _Function> FoldMap<_Function> for (Head, Tail)
where
    (Head, Tail): Fmap<_Function>,
    <(Head, Tail) as Fmap<_Function>>::Fmap: Mconcat,
{
    type FoldMap = <<(Head, Tail) as Fmap<_Function>>::Fmap as Mconcat>::Mconcat;

    fn fold_map(self, f: _Function) -> Self::FoldMap {
        self.fmap(f).mconcat()
    }
}
