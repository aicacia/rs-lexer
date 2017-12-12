use alloc::boxed::Box;
use alloc::vec::Vec;

use core::slice;

use super::{Lexer, Input, Reader, ReadersBuilder};


pub struct Readers<T, E> {
    vec: Vec<Box<Reader<T, E>>>,
}

impl<T, E> From<ReadersBuilder<T, E>> for Readers<T, E> {

    #[inline(always)]
    fn from(readers_builder: ReadersBuilder<T, E>) -> Readers<T, E> {
        Readers {
            vec: readers_builder.vec,
        }
    }
}

impl<T, E> Readers<T, E> {

    #[inline(always)]
    pub fn new() -> Self {
        Readers {
            vec: Vec::new(),
        }
    }

    #[inline]
    pub fn add<R: 'static + Reader<T, E>>(&mut self, reader: R) -> &mut Self {
        self.no_sort_add(reader).sort()
    }

    #[inline]
    pub fn no_sort_add<R: 'static + Reader<T, E>>(&mut self, reader: R) -> &mut Self {
        self.vec.push(Box::new(reader));
        self
    }

    #[inline]
    fn sort(&mut self) -> &mut Self {
        self.vec.sort_by(|a, b| a.priority().cmp(&b.priority()));
        self
    }

    #[inline(always)]
    pub fn lexer<I>(&self, input: I) -> Lexer<T, E, I>
        where I: Input,
    {
        Lexer::new(self, input)
    }
}

impl<'a, T, E> Readers<T, E>
    where T: 'a,
          E: 'a,
{
    #[inline(always)]
    pub fn iter(&'a self) -> ReadersIter<'a, T, E> {
        ReadersIter {
            iter: self.vec.iter(),
        }
    }
    #[inline(always)]
    pub fn iter_mut(&'a mut self) -> ReadersIterMut<'a, T, E> {
        ReadersIterMut {
            iter: self.vec.iter_mut(),
        }
    }
}

impl<'a, T, E> IntoIterator for &'a Readers<T, E>
    where T: 'a,
          E: 'a,
{
    type Item = &'a Reader<T, E>;
    type IntoIter = ReadersIter<'a, T, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, E> IntoIterator for &'a mut Readers<T, E>
    where T: 'a,
          E: 'a,
{
    type Item = &'a mut (Reader<T, E> + 'static);
    type IntoIter = ReadersIterMut<'a, T, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


pub struct ReadersIter<'a, T, E>
    where T: 'a,
          E: 'a,
{
    iter: slice::Iter<'a, Box<Reader<T, E>>>,
}

impl<'a, T, E> Iterator for ReadersIter<'a, T, E>
    where T: 'a,
          E: 'a,
{
    type Item = &'a Reader<T, E>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|reader| &**reader)
    }
}


pub struct ReadersIterMut<'a, T, E>
    where T: 'a,
          E: 'a,
{
    iter: slice::IterMut<'a, Box<Reader<T, E>>>,
}

impl<'a, T, E> Iterator for ReadersIterMut<'a, T, E>
    where T: 'a,
          E: 'a,
{
    type Item = &'a mut (Reader<T, E> + 'static);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|reader| &mut **reader)
    }
}
