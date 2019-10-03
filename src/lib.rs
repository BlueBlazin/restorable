use std::collections::VecDeque;

pub struct RestorableIter<T, I>
where
    T: Iterator<Item = I>,
    I: Clone,
{
    inner: T,
    buf: VecDeque<Option<I>>,
    saving: bool,
}

impl<T, I> RestorableIter<T, I>
where
    T: Iterator<Item = I>,
    I: Clone,
{
    pub fn new(inner: T) -> Self {
        RestorableIter {
            inner,
            buf: VecDeque::with_capacity(4),
            saving: false,
        }
    }
    #[inline]
    pub fn save(&mut self) {
        self.saving = true;
    }
    #[inline]
    pub fn restore(&mut self) {
        self.saving = false;
    }
}

impl<T, I> Iterator for RestorableIter<T, I>
where
    T: Iterator<Item = I>,
    I: Clone,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        if self.saving {
            let next_item = self.inner.next();
            self.buf.push_back(next_item.clone());
            next_item
        } else {
            match self.buf.pop_front() {
                Some(saved_item) => saved_item,
                None => self.inner.next(),
            }
        }
    }
}

pub trait Restorable<I>: Iterator
where
    I: Clone,
{
    fn iter_restorable(self) -> RestorableIter<Self, I>
    where
        Self: Sized + Iterator<Item = I>,
    {
        RestorableIter::new(self)
    }
}

impl<I, T: ?Sized> Restorable<I> for T
where
    T: Iterator<Item = I>,
    I: Clone,
{
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_restorable_new() {
        let mut iterator = RestorableIter::new(0..5);
        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        iterator.save();
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        iterator.restore();
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_iter_restorable() {
        let mut iterator = (0..5).iter_restorable();
        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        iterator.save();
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        iterator.restore();
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), None);
    }
}
