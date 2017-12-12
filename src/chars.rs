use alloc::vec::Vec;

use super::{Input, State};


pub struct Chars<I>
    where I: Iterator<Item = char>
{
    iter: I,
    index: usize,
    vec: Vec<char>,
}

impl<I> Input for Chars<I>
    where I: Iterator<Item = char>
{
    #[inline]
    fn peek(&mut self, state: &State, offset: usize) -> Option<char> {
        let index = state.index() + offset;

        if index < self.index {
            Some(self.vec[index])
        } else if self.next().is_some() {
            self.char_at(index)
        } else {
            None
        }
    }
}

impl<I> From<I> for Chars<I>
    where I: Iterator<Item = char>
{
    #[inline(always)]
    fn from(iter: I) -> Self {
        Chars {
            iter: iter,
            index: 0,
            vec: Vec::new(),
        }
    }
}

impl<I> Chars<I>
    where I: Iterator<Item = char>
{
    #[inline(always)]
    pub fn new(iter: I) -> Self { From::from(iter) }

    #[inline]
    pub fn char_at(&mut self, index: usize) -> Option<char> {
        if index < self.index {
            Some(self.vec[index])
        } else if self.next().is_some() {
            self.char_at(index)
        } else {
            None
        }
    }
}

impl<I> Iterator for Chars<I>
    where I: Iterator<Item = char>
{
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let ch = self.vec[self.index];
            self.index += 1;
            Some(ch)
        } else if let Some(ch) = self.iter.next() {
            self.vec.push(ch);
            self.index += 1;
            Some(ch)
        } else {
            None
        }
    }
}


#[cfg(feature = "use_std")]
mod __use_std {
    use super::*;

    use std::io;


    impl<R> From<R> for Chars<CharsRead<R>>
        where R: io::Read,
    {
        #[inline(always)]
        fn from(reader: R) -> Self {
            Chars {
                iter: CharsRead::from(reader),
                index: 0,
                vec: Vec::new(),
            }
        }
    }

    pub struct CharsRead<R>
        where R: io::Read,
    {
        chars: io::Chars<R>,
    }

    impl<R> From<R> for CharsRead<R>
        where R: io::Read,
    {
        #[inline(always)]
        fn from(reader: R) -> Self {
            CharsRead {
                chars: reader.chars(),
            }
        }
    }

    impl<R> Iterator for CharsRead<R>
        where R: io::Read,
    {
        type Item = char;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            match self.chars.next() {
                Some(Ok(ch)) => Some(ch),
                _ => None,
            }
        }
    }
}

#[cfg(feature = "use_std")]
pub use self::__use_std::*;


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_chars_iter() {
        let chars = Chars::new("abcdefg".chars());
        assert_eq!(chars.collect::<Vec<char>>(), ['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    }

    #[test]
    fn test_chars_input() {
        let mut state = State::new();
        let mut chars = Chars::new("abcdefg".chars());

        assert_eq!(chars.peek(&state, 0), Some('a'));
        assert_eq!(chars.peek(&state, 1), Some('b'));
        assert_eq!(chars.peek(&state, 2), Some('c'));

        chars.read(&mut state);
        chars.read(&mut state);
        chars.read(&mut state);

        assert_eq!(chars.read(&mut state), Some('d'));
        assert_eq!(chars.read(&mut state), Some('e'));
        assert_eq!(chars.read(&mut state), Some('f'));

        assert_eq!(chars.peek(&state, 0), Some('g'));
        assert_eq!(chars.read(&mut state), Some('g'));

        assert_eq!(chars.read(&mut state), None);
    }
}
