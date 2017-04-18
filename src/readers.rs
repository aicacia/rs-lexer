use alloc::boxed::Box;

use collections::vec::Vec;

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

    #[inline]
    pub fn add<R: 'static + Reader<T>>(&mut self, reader: R) -> &mut Self {
        self.readers.push(Box::new(reader));
        self
    }

    #[inline]
    pub fn sort(&mut self) -> &mut Self {
        self.readers.sort_by(|a, b| a.priority().cmp(&b.priority()));
        self
    }

    #[inline]
    pub fn iter(&mut self) -> slice::Iter<Box<Reader<T>>> {
        self.readers.iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<Box<Reader<T>>> {
        self.readers.iter_mut()
    }
}
