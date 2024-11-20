use std::{collections::HashSet, hash::Hash};

pub trait IteratorExt: Iterator {
    fn unique(self) -> UniqueIterator<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        UniqueIterator {
            iter: self,
            seen: HashSet::new(),
        }
    }
}

pub struct UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I> Iterator for UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| self.seen.insert(item.clone()))
    }
}

// blanket implementation instead of manual implementation for each type
impl<I> IteratorExt for I where I: Iterator {}
// impl IteratorExt for std::vec::IntoIter<&str> {}
// impl IteratorExt for std::vec::IntoIter<String> {}
// impl IteratorExt for std::vec::IntoIter<i32> {}
// impl IteratorExt for std::collections::binary_heap::IntoIter<i32> {}
// impl IteratorExt for std::collections::binary_heap::IntoIter<String> {}

fn main() {
    let numbers = vec![1, 2, 2, 3, 3, 4];
    let unique_numbers: Vec<_> = numbers.into_iter().unique().collect();
    println!("{:?}", unique_numbers);

    let words = vec!["hello", "world", "hello"];
    let unique_words: Vec<_> = words.into_iter().unique().collect();
    println!("{:?}", unique_words);
}
