use crate::t_funk::{
    hlist::{PopBack as HListPopBack, ToTList},
    tlist::TList,
};

pub trait PopBack: TList {
    type PopBack: TList;

    fn pop_back(self) -> Self::PopBack;
}

impl<T> PopBack for T
where
    T: TList,
    T::HList: HListPopBack,
    <T::HList as HListPopBack>::PopBack: ToTList,
{
    type PopBack = <<T::HList as HListPopBack>::PopBack as ToTList>::TList;

    fn pop_back(self) -> Self::PopBack {
        self.to_hlist().pop_back().to_tlist()
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::tlist::pop_back::PopBack;

    #[test]
    fn test_tuple_pop_back() {
        let list: (usize, f32, &str) = (1, 2.0, "three");
        let list: (usize, f32) = list.pop_back();
        let list: (usize,) = list.pop_back();
        let list: () = list.pop_back();
        assert_eq!((), list);
    }
}
