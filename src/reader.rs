use std::hash::Hash;

use super::lexer::Lexer;
use super::state::State;
use super::token::Token;


pub trait Reader<T: Clone + Eq + PartialEq + Hash> {
    fn read(&self, &Lexer<T>, &mut State) -> Option<Token<T>>;
    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }
}
