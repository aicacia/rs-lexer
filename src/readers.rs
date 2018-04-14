use alloc::boxed::Box;
use alloc::vec::Vec;

use core::slice;

use peek_nth::IteratorExt;

use super::{Lexer, Reader, ReadersBuilder};

pub struct Readers<T, E, D = ()>(Vec<Box<Reader<T, E, D>>>);

unsafe impl<T, E, D> Sync for Readers<T, E, D>
where
    T: Sync,
    E: Sync,
    D: Sync,
{
}
unsafe impl<T, E, D> Send for Readers<T, E, D>
where
    T: Send,
    E: Send,
    D: Send,
{
}

impl<T, E, D> From<ReadersBuilder<T, E, D>> for Readers<T, E, D> {
    #[inline]
    fn from(readers_builder: ReadersBuilder<T, E, D>) -> Readers<T, E, D> {
        Readers(readers_builder.0)
    }
}

impl<T, E, D> Readers<T, E, D> {
    #[inline]
    pub fn new() -> Self {
        Readers(Vec::new())
    }

    #[inline]
    pub fn add<R: 'static + Reader<T, E, D>>(&mut self, reader: R) -> &mut Self {
        let index = self.0
            .iter()
            .position(|r| reader.priority() <= r.priority());
        let boxed_reader = Box::new(reader);

        if let Some(index) = index {
            self.0.insert(index, boxed_reader);
        } else {
            self.0.push(boxed_reader);
        }

        self
    }

    #[inline]
    pub fn lexer<'a, I>(&'a self, input: I, data: &'a mut D) -> Lexer<'a, T, E, I, D>
    where
        I: IteratorExt<Item = char>,
    {
        Lexer::new(self, input, data)
    }
}

impl<'a, T, E, D> Readers<T, E, D>
where
    T: 'a,
    E: 'a,
    D: 'a,
{
    #[inline]
    pub fn iter(&'a self) -> ReadersIter<'a, T, E, D> {
        ReadersIter {
            iter: self.0.iter(),
        }
    }
    #[inline]
    pub fn iter_mut(&'a mut self) -> ReadersIterMut<'a, T, E, D> {
        ReadersIterMut {
            iter: self.0.iter_mut(),
        }
    }
}

impl<'a, T, E, D> IntoIterator for &'a Readers<T, E, D>
where
    T: 'a,
    E: 'a,
    D: 'a,
{
    type Item = &'a Reader<T, E, D>;
    type IntoIter = ReadersIter<'a, T, E, D>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, E, D> IntoIterator for &'a mut Readers<T, E, D>
where
    T: 'a,
    E: 'a,
    D: 'a,
{
    type Item = &'a mut (Reader<T, E, D> + 'static);
    type IntoIter = ReadersIterMut<'a, T, E, D>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct ReadersIter<'a, T, E, D>
where
    T: 'a,
    E: 'a,
    D: 'a,
{
    iter: slice::Iter<'a, Box<Reader<T, E, D>>>,
}

impl<'a, T, E, D> Iterator for ReadersIter<'a, T, E, D>
where
    T: 'a,
    E: 'a,
    D: 'a,
{
    type Item = &'a Reader<T, E, D>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|reader| &**reader)
    }
}

pub struct ReadersIterMut<'a, T, E, D>
where
    T: 'a,
    E: 'a,
    D: 'a,
{
    iter: slice::IterMut<'a, Box<Reader<T, E, D>>>,
}

impl<'a, T, E, D> Iterator for ReadersIterMut<'a, T, E, D>
where
    T: 'a,
    E: 'a,
    D: 'a,
{
    type Item = &'a mut (Reader<T, E, D> + 'static);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|reader| &mut **reader)
    }
}
