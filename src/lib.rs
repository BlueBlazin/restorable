use std::collections::VecDeque;

pub struct Restorable<T, I> 
where
    T: Iterator<Item = I>,
    I: Clone
{
    iterator: T,
    buf: VecDeque<Option<I>>,
    saving: bool,
}

impl<T, I> Restorable<T, I>
where
    T: Iterator<Item = I>,
    I: Clone
{
    pub fn from(iterator: T) -> Self {
        Restorable {
            iterator,
            buf: VecDeque::with_capacity(4),
            saving: false
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

impl<T, I> Iterator for Restorable<T, I>
where
    T: Iterator<Item = I>,
    I: Clone
{
    type Item = I;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.saving {
            let next_item = self.iterator.next();
            self.buf.push_back(next_item.clone());
            next_item
        } else {
            match self.buf.pop_front() {
                Some(saved_item) => saved_item,
                None => self.iterator.next()
            }
        }
    }
}

// fn main() {
//     let mut iterator = Restorable::from(0..8);
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
//     iterator.save();
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
//     iterator.restore();
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());
// }



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
