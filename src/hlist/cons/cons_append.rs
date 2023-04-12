use super::ConsList;

pub trait ConsAppend<T>: ConsList {
    type Appended;

    fn cons_append(self, t: T) -> Self::Appended;
}

impl<Head, Tail, T> ConsAppend<T> for (Head, Tail)
where
    Self: ConsList,
    Tail: ConsAppend<T>,
{
    type Appended = (Head, Tail::Appended);

    fn cons_append(self, t: T) -> Self::Appended {
        (self.0, self.1.cons_append(t))
    }
}

impl<T> ConsAppend<T> for ()
where
    Self: ConsList,
    T: ConsList,
{
    type Appended = T;

    fn cons_append(self, t: T) -> Self::Appended {
        t
    }
}

#[cfg(test)]
mod test {
    use crate::hlist::{
        cons::{ConsAppend, Uncons},
        tuple::Cons,
    };

    #[test]
    fn test_cons_append() {
        let list_a = (1, 2, 3).cons();
        let list_b = (4, 5, 6).cons();
        let list_c = list_a.cons_append(list_b);
        assert_eq!(list_c.uncons(), (1, 2, 3, 4, 5, 6));
    }
}
