use alloc::boxed::Box;
use alloc::vec::Vec;

use core::slice;

use peek_nth::IteratorExt;

use super::{Lexer, Reader, ReadersBuilder};

pub struct Readers<T, E>(Vec<Box<Reader<T, E>>>);

unsafe impl<T, E> Sync for Readers<T, E>
where
    T: Sync,
    E: Sync,
{
}
unsafe impl<T, E> Send for Readers<T, E>
where
    T: Send,
    E: Send,
{
}

impl<T, E> From<ReadersBuilder<T, E>> for Readers<T, E> {
    #[inline]
    fn from(readers_builder: ReadersBuilder<T, E>) -> Readers<T, E> {
        Readers(readers_builder.0)
    }
}

impl<T, E> Readers<T, E> {
    #[inline]
    pub fn new() -> Self {
        Readers(Vec::new())
    }

    #[inline]
    pub fn add<R: 'static + Reader<T, E>>(&mut self, reader: R) -> &mut Self {
        let index = self.0.iter().position(|r| reader.priority() < r.priority());
        let boxed_reader = Box::new(reader);

        if let Some(index) = index {
            self.0.insert(index, boxed_reader);
        } else {
            self.0.push(boxed_reader);
        }

        self
    }

    #[inline]
    pub fn lexer<I>(&self, input: I) -> Lexer<T, E, I>
    where
        I: IteratorExt<Item = char>,
    {
        Lexer::new(self, input)
    }
}

impl<'a, T, E> Readers<T, E>
where
    T: 'a,
    E: 'a,
{
    #[inline]
    pub fn iter(&'a self) -> ReadersIter<'a, T, E> {
        ReadersIter {
            iter: self.0.iter(),
        }
    }
    #[inline]
    pub fn iter_mut(&'a mut self) -> ReadersIterMut<'a, T, E> {
        ReadersIterMut {
            iter: self.0.iter_mut(),
        }
    }
}

impl<'a, T, E> IntoIterator for &'a Readers<T, E>
where
    T: 'a,
    E: 'a,
{
    type Item = &'a Reader<T, E>;
    type IntoIter = ReadersIter<'a, T, E>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, E> IntoIterator for &'a mut Readers<T, E>
where
    T: 'a,
    E: 'a,
{
    type Item = &'a mut (Reader<T, E> + 'static);
    type IntoIter = ReadersIterMut<'a, T, E>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct ReadersIter<'a, T, E>
where
    T: 'a,
    E: 'a,
{
    iter: slice::Iter<'a, Box<Reader<T, E>>>,
}

impl<'a, T, E> Iterator for ReadersIter<'a, T, E>
where
    T: 'a,
    E: 'a,
{
    type Item = &'a Reader<T, E>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|reader| &**reader)
    }
}

pub struct ReadersIterMut<'a, T, E>
where
    T: 'a,
    E: 'a,
{
    iter: slice::IterMut<'a, Box<Reader<T, E>>>,
}

impl<'a, T, E> Iterator for ReadersIterMut<'a, T, E>
where
    T: 'a,
    E: 'a,
{
    type Item = &'a mut (Reader<T, E> + 'static);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|reader| &mut **reader)
    }
}
