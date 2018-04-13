#![feature(alloc)]
#![no_std]

#[macro_use]
extern crate alloc;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate peek_nth;

mod input;
mod lexer;
mod lines;
mod reader;
mod readers;
mod readers_builder;
mod state;
mod token;
mod token_error;
mod token_meta;

pub use self::input::Input;
pub use self::lexer::Lexer;
pub use self::lines::Lines;
pub use self::reader::{Reader, ReaderResult};
pub use self::readers::{Readers, ReadersIter};
pub use self::readers_builder::ReadersBuilder;
pub use self::state::State;
pub use self::token::Token;
pub use self::token_error::TokenError;
pub use self::token_meta::TokenMeta;
