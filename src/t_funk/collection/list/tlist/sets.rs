use crate::t_funk::hlist::{Paths, Sets as HListSets, ToTList};

use super::{TList, ToHList};

pub trait Sets<T, P>: TList {
    fn sets(self, t: T) -> Self;
}

impl<T, P, In> Sets<In, P> for T
where
    Self: TList,
    Self::HList: HListSets<In::HList, P>,
    In: ToHList,
    P: Paths,
{
    fn sets(self, t: In) -> Self {
        self.to_hlist().sets(t.to_hlist()).to_tlist()
    }
}

impl<T> Sets<(), ()> for T
where
    T: TList,
{
    fn sets(self, _: ()) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::tlist::Sets;

    #[test]
    fn test_tuple_sets() {
        let list = (1, 2.0, "three");
        let list = Sets::<(&str, f32, usize), _>::sets(list, ("hello", 7.0, 5));
        //let list = list.tuple_sets(("hello", 7.0, 5));
        assert_eq!((5, 7.0, "hello"), list);
    }
}
