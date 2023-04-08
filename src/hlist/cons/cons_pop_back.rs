use super::ConsList;

pub trait ConsPopBack: ConsList {
    type ConsPopBack: ConsList;

    fn cons_pop_back(self) -> Self::ConsPopBack;
}

impl<Head, Tail> ConsPopBack for (Head, Tail)
where
    (Head, Tail): ConsList,
    Tail: ConsPopBack,
    (Head, Tail::ConsPopBack): ConsList,
{
    type ConsPopBack = (Head, Tail::ConsPopBack);

    fn cons_pop_back(self) -> Self::ConsPopBack {
        (self.0, self.1.cons_pop_back())
    }
}

impl<Head> ConsPopBack for (Head, ()) {
    type ConsPopBack = ();

    fn cons_pop_back(self) -> Self::ConsPopBack {
        ()
    }
}

#[cfg(test)]
mod test {
    use crate::hlist::tuple::Cons;

    use super::ConsPopBack;

    #[test]
    fn test_cons_pop_back() {
        let list: (usize, (f32, (&str, ()))) = (1, 2.0, "three").cons();
        let list: (usize, (f32, ())) = list.cons_pop_back();
        let list: (usize, ()) = list.cons_pop_back();
        let list: () = list.cons_pop_back();
        assert_eq!((), list);
    }
}
