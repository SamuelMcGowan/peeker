mod eat;
pub use eat::*;
use peekmore::PeekMoreIterator;

use std::iter::Peekable;

pub trait Peek: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: Iterator> Peek for Peekable<I> {
    #[inline]
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }
}

impl<I: Iterator> Peek for PeekMoreIterator<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }
}

pub trait ConsumeIter: Iterator {
    fn consume(self) -> bool;
}

impl<I: Iterator> ConsumeIter for I {
    #[inline]
    fn consume(mut self) -> bool {
        let consumed_something = self.next().is_some();
        self.for_each(|_| {});
        consumed_something
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let nums: Vec<usize> = (0..10).peekable().next_while(|&n| n < 5).collect();
        assert_eq!(nums, vec![0, 1, 2, 3, 4])
    }

    #[test]
    fn by_ref() {
        let mut nums = (0..10).peekable();
        nums.by_ref().next_while(|&n| n < 5).consume();
        assert_eq!(nums.next(), Some(5));
    }
}
