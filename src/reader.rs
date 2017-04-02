use core::hash::Hash;

use super::input::Input;
use super::state::State;
use super::token::Token;


pub trait Reader<T>
    where T: Clone + Eq + PartialEq + Hash
{
    #[inline(always)]
    fn priority(&self) -> usize {
        1000usize
    }

    fn read(&self, &Input, &State, &mut State) -> Option<Token<T>>;
}
