use alloc::boxed::Box;
use alloc::vec::Vec;

use core::slice;

use super::reader::Reader;


pub struct Readers<T> {
    readers: Vec<Box<Reader<T>>>,
}

impl<T> Readers<T> {

    #[inline(always)]
    pub fn new() -> Self {
        Readers {
            readers: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn add<R: 'static + Reader<T>>(&mut self, reader: R) -> &mut Self {
        self.readers.push(Box::new(reader));
        self
    }

    #[inline(always)]
    pub fn len(&mut self) -> usize {
        self.readers.len()
    }

    #[inline(always)]
    pub fn sort(&mut self) -> &mut Self {
        self.readers.sort_by(|a, b| a.priority().cmp(&b.priority()));
        self
    }

    #[inline(always)]
    pub fn iter(&mut self) -> slice::Iter<Box<Reader<T>>> {
        self.readers.iter()
    }
}
