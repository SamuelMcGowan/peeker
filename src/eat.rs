use crate::Peek;

#[must_use]
pub struct NextWhile<'a, I: Peek, F: Fn(&I::Item) -> bool> {
    inner: &'a mut I,
    predicate: F,
}

impl<'a, I: Peek, F: Fn(&I::Item) -> bool> Iterator for NextWhile<'a, I, F> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.peek() {
            Some(item) if (self.predicate)(item) => self.inner.next(),
            _ => None,
        }
    }
}

pub trait NextIf: Peek + Sized {
    fn next_if<F: Fn(&Self::Item) -> bool>(&mut self, predicate: F) -> bool;

    #[inline]
    fn next_while<F: Fn(&Self::Item) -> bool>(&mut self, predicate: F) -> NextWhile<Self, F> {
        NextWhile {
            inner: self,
            predicate,
        }
    }
}

impl<I: Peek + Sized> NextIf for I {
    #[inline]
    fn next_if<F: Fn(&Self::Item) -> bool>(&mut self, predicate: F) -> bool {
        match self.peek() {
            Some(item) if predicate(item) => {
                self.next();
                true
            }
            _ => false,
        }
    }
}

pub trait Eat: Iterator {
    fn eat(&mut self, value: Self::Item) -> bool;
}

impl<I: NextIf> Eat for I
where
    Self::Item: PartialEq,
{
    #[inline]
    fn eat(&mut self, value: Self::Item) -> bool {
        self.next_if(|item| *item == value)
    }
}

// if lexer.eat_if(|token| token.is_sep()).is_some() {
//     // ...
// }
