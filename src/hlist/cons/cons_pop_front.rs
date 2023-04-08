use super::ConsList;

pub trait ConsPopFront: ConsList {
    type ConsPopFront: ConsList;

    fn cons_pop_front(self) -> Self::ConsPopFront;
}

impl<Head, Tail> ConsPopFront for (Head, Tail)
where
    (Head, Tail): ConsList,
    Tail: ConsList,
{
    type ConsPopFront = Tail;

    fn cons_pop_front(self) -> Self::ConsPopFront {
        self.1
    }
}

#[cfg(test)]
mod test {
    use crate::hlist::tuple::Cons;

    use super::ConsPopFront;

    #[test]
    fn test_cons_pop_front() {
        let list: (usize, (f32, (&str, ()))) = (1, 2.0, "three").cons();
        let list: (f32, (&str, ())) = list.cons_pop_front();
        let list: (&str, ()) = list.cons_pop_front();
        let list: () = list.cons_pop_front();
        assert_eq!((), list);
    }
}
