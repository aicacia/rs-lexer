use alloc::boxed::Box;
use alloc::vec::Vec;

use core::slice;

use super::{Lexer, Input, Reader, ReadersBuilder};


pub struct Readers<T> {
    vec: Vec<Box<Reader<T>>>,
}

impl<T> From<ReadersBuilder<T>> for Readers<T> {

    #[inline(always)]
    fn from(readers_builder: ReadersBuilder<T>) -> Readers<T> {
        Readers {
            vec: readers_builder.vec,
        }
    }
}

impl<T> Readers<T> {

    #[inline(always)]
    pub fn new() -> Self {
        Readers {
            vec: Vec::new(),
        }
    }

    #[inline]
    pub fn add<R: 'static + Reader<T>>(&mut self, reader: R) -> &mut Self {
        self.no_sort_add(reader).sort()
    }

    #[inline]
    pub fn no_sort_add<R: 'static + Reader<T>>(&mut self, reader: R) -> &mut Self {
        self.vec.push(Box::new(reader));
        self
    }

    #[inline]
    fn sort(&mut self) -> &mut Self {
        self.vec.sort_by(|a, b| a.priority().cmp(&b.priority()));
        self
    }

    #[inline(always)]
    pub fn lexer<I>(&self, input: I) -> Lexer<T, I>
        where I: Input,
    {
        Lexer::new(self, input)
    }
}

impl<'a, T> Readers<T>
    where T: 'a,
{
    #[inline(always)]
    pub fn iter(&'a self) -> ReadersIter<'a, T> {
        ReadersIter {
            iter: self.vec.iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a Readers<T>
    where T: 'a,
{
    type Item = &'a Reader<T>;
    type IntoIter = ReadersIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


pub struct ReadersIter<'a, T>
    where T: 'a,
{
    iter: slice::Iter<'a, Box<Reader<T>>>,
}

impl<'a, T> Iterator for ReadersIter<'a, T>
    where T: 'a,
{
    type Item = &'a Reader<T>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|reader| &**reader)
    }
}
