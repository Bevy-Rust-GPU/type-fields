use crate::t_funk::{
    hlist::{PopFront as HListPopFront, ToTList},
    tlist::TList,
};

pub trait PopFront: TList {
    type PopFront: TList;

    fn pop_front(self) -> Self::PopFront;
}

impl<T> PopFront for T
where
    T: TList,
    T::HList: HListPopFront,
    <T::HList as HListPopFront>::PopFront: ToTList,
{
    type PopFront = <<T::HList as HListPopFront>::PopFront as ToTList>::TList;

    fn pop_front(self) -> Self::PopFront {
        self.to_hlist().cons_pop_front().to_tlist()
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::tlist::pop_front::PopFront;

    #[test]
    fn test_tuple_pop_back() {
        let list: (usize, f32, &str) = (1, 2.0, "three");
        let list: (f32, &str) = list.pop_front();
        let list: (&str,) = list.pop_front();
        let list: () = list.pop_front();
        assert_eq!((), list);
    }
}
