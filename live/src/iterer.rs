
pub trait Iterer where Self: Sized + Iterator{
    fn folding<A, F: FnMut(A, Self::Item)->A>(self, acc: A, f: F) -> FoldingIter<Self, A, F>;
}
impl<I: Iterator> Iterer for I {
    fn folding<A, F: FnMut(A, Self::Item)->A>(self, acc: A, f: F) -> FoldingIter<Self, A, F> {
        FoldingIter {
            iter: self,
            f,
            acc,
        }
    }
}

pub struct FoldingIter<Inner, Acc, F>
where
    Inner: Iterator,
    F: FnMut(Acc, Inner::Item) -> Acc
{
    iter: Inner,
    f: F,
    acc: Acc
}

impl<Inner, I, Acc, F> Iterator for FoldingIter<Inner, Acc, F>
where Inner: Iterator<Item = I>, F: FnMut(Acc, I) -> Acc, Acc: Clone{
    type Item = Acc;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(inner) => {
                self.acc = (self.f)(self.acc.to_owned(), inner);
                Some(self.acc.to_owned())
            },
            None => None,
        }
    }
}
