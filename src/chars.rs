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
            self.peek(state, offset)
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

    use std::io::{Read, Result, ErrorKind};


    impl<R> From<R> for Chars<CharsRead<R>>
        where R: Read,
    {
        #[inline(always)]
        fn from(reader: R) -> Self {
            Chars {
                iter: CharsRead { inner: reader },
                index: 0,
                vec: Vec::new(),
            }
        }
    }

    pub struct CharsRead<R>
        where R: Read,
    {
        inner: R,
    }

    impl<R> Iterator for CharsRead<R>
        where R: Read,
    {
        type Item = char;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            let first_byte = match read_one_byte(&mut self.inner) {
                None => return None,
                Some(Ok(b)) => b,
                Some(Err(_)) => return None, // Some(Err(CharsError::Other(e))),
            };
            let width = ::core::str::utf8_char_width(first_byte);
            if width == 1 {
                return Some(first_byte as char);
            }
            if width == 0 {
                return None; // Some(Err(CharsError::NotUtf8));
            }
            let mut buf = [first_byte, 0, 0, 0];
            {
                let mut start = 1;
                while start < width {
                    match self.inner.read(&mut buf[start..width]) {
                        Ok(0) => return None, // Some(Err(CharsError::NotUtf8)),
                        Ok(n) => start += n,
                        Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                        Err(_) => return None, // Some(Err(CharsError::Other(e))),
                    }
                }
            }
            match ::std::str::from_utf8(&buf[..width]).ok() {
                Some(s) => Some(s.chars().next().unwrap()),
                None => None, // Err(CharsError::NotUtf8),
            }
        }
    }

    #[inline]
    fn read_one_byte(reader: &mut Read) -> Option<Result<u8>> {
        let mut buf = [0];
        loop {
            return match reader.read(&mut buf) {
                Ok(0) => None,
                Ok(..) => Some(Ok(buf[0])),
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => Some(Err(e)),
            };
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
