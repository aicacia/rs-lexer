use std::hash::Hash;

use super::lexical_reader::LexicalReader;
use super::state::State;
use super::token::Token;


pub trait Reader<T> 
    where T: Clone + Eq + PartialEq + Hash
{
    fn read(&self, &LexicalReader<T>, &mut State) -> Option<Token<T>>;
    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }
}
