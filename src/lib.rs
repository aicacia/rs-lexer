#![feature(alloc)]
#![no_std]


#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate alloc;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate chars_input;


mod lexer;
mod reader;
mod readers_builder;
mod readers;
mod token_error;
mod token_meta;
mod token;


pub use self::lexer::Lexer;
pub use self::reader::{Reader, ReaderResult};
pub use self::readers_builder::ReadersBuilder;
pub use self::readers::{Readers, ReadersIter};
pub use self::token_error::TokenError;
pub use self::token_meta::TokenMeta;
pub use self::token::Token;
pub use chars_input::{State, Chars, Input, Lines};
