use super::HList;

pub trait PopBack: HList {
    type PopBack: HList;

    fn pop_back(self) -> Self::PopBack;
}

impl<Head, Tail> PopBack for (Head, Tail)
where
    (Head, Tail): HList,
    Tail: PopBack,
    (Head, Tail::PopBack): HList,
{
    type PopBack = (Head, Tail::PopBack);

    fn pop_back(self) -> Self::PopBack {
        (self.0, self.1.pop_back())
    }
}

impl<Head> PopBack for (Head, ()) {
    type PopBack = ();

    fn pop_back(self) -> Self::PopBack {
        ()
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::tlist::ToHList;

    use super::PopBack;

    #[test]
    fn test_cons_pop_back() {
        let list: (usize, (f32, (&str, ()))) = (1, 2.0, "three").to_hlist();
        let list: (usize, (f32, ())) = list.pop_back();
        let list: (usize, ()) = list.pop_back();
        let list: () = list.pop_back();
        assert_eq!((), list);
    }
}
