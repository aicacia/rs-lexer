use super::input::Input;
use super::state::State;


pub trait Reader<T> {

    #[inline(always)]
    fn priority(&self) -> usize {
        1000usize
    }

    fn read(&self, &Input, &State, &mut State) -> Option<T>;
}
