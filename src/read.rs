use super::{Input, ReaderResult, Readers, State};

#[inline]
pub fn read<T, E>(
  readers: &Readers<T, E>,
  input: &mut dyn Input,
  state: &mut State,
) -> Option<Result<T, E>> {
  if input.is_done(state) {
    None
  } else {
    let mut token = None;
    let mut new_state = None;
    let mut is_empty = false;
    let orig_state = state.clone();

    for reader in readers.iter() {
      let mut next_state = orig_state.clone();

      match reader.read(readers, input, &orig_state, &mut next_state) {
        ReaderResult::Some(t) => {
          token = Some(Ok(t));
          new_state = Some(next_state);
          break;
        }
        ReaderResult::Err(e) => {
          return Some(Err(e));
        }
        ReaderResult::Empty => {
          new_state = Some(next_state);
          is_empty = true;
          break;
        }
        ReaderResult::None => (),
      }
    }

    if let Some(s) = new_state {
      state.clone_from(&s);
    }

    if is_empty {
      read(readers, input, state)
    } else {
      debug_assert!(
        orig_state.index() != state.index() || input.is_done(&state),
        "No reader was able to read at {:?}",
        orig_state
      );
      token
    }
  }
}
