use super::{Input, Line, State};

pub struct Lines<'a> {
  state: &'a mut State,
  input: &'a mut dyn Input,
}

impl<'a> Lines<'a> {
  #[inline(always)]
  pub fn new(input: &'a mut dyn Input, state: &'a mut State) -> Self {
    Lines {
      state: state,
      input: input,
    }
  }

  #[inline]
  pub fn peek_line(&mut self) -> Option<Line> {
    self.input.peek_line(self.state)
  }

  #[inline]
  pub fn skip_line(&mut self) {
    self.input.skip_line(self.state);
  }
}

impl<'a> Iterator for Lines<'a> {
  type Item = Line;

  #[inline]
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

    assert_eq!(lines.next(), Some(Line::from("abc")));
    assert_eq!(lines.next(), Some(Line::from("def")));
    assert_eq!(lines.next(), Some(Line::from("ghi")));
    assert_eq!(lines.next(), None);
  }
}
