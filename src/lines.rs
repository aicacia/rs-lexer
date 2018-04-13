use alloc::string::String;

use super::{Input, State};

pub struct Lines<'a, I>
where
    I: 'a + Input,
{
    state: &'a mut State,
    input: &'a mut I,
}

impl<'a, I> Lines<'a, I>
where
    I: 'a + Input,
{
    #[inline(always)]
    pub fn new(input: &'a mut I, state: &'a mut State) -> Self {
        Lines {
            state: state,
            input: input,
        }
    }
}

impl<'a, I> Iterator for Lines<'a, I>
where
    I: 'a + Input,
{
    type Item = String;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.input.read_line(self.state)
    }
}

#[cfg(test)]
mod test {
    use super::super::Input;
    use super::*;
    use peek_nth::IteratorExt;

    #[test]
    fn test_lines() {
        let mut input = "abc\ndef\nghi\n".chars().peekable_nth();
        let mut state = State::new();
        let mut lines = input.lines(&mut state);

        assert_eq!(lines.next(), Some(String::from("abc")));
        assert_eq!(lines.next(), Some(String::from("def")));
        assert_eq!(lines.next(), Some(String::from("ghi")));
        assert_eq!(lines.next(), None);
    }
}
