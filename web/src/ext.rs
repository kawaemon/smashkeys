pub trait IteratorAllEqExt {
    fn all_eq(self) -> bool;
}

impl<Iter, ItemA, ItemB> IteratorAllEqExt for Iter
where
    Iter: Iterator<Item = (ItemA, ItemB)>,
    ItemA: PartialEq<ItemB>,
{
    fn all_eq(mut self) -> bool {
        self.all(|(a, b)| a == b)
    }
}
