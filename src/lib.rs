#![feature(alloc)]
#![cfg_attr(feature = "use_std", feature(io))]
#![no_std]


#[cfg(feature = "use_std")] extern crate std;

#[macro_use] extern crate alloc;

extern crate serde;
#[macro_use] extern crate serde_derive;


mod chars;
mod input;
mod lexer;
mod reader;
mod readers_builder;
mod readers;
mod state;
mod token_error;
mod token_meta;
mod token;


pub use self::chars::Chars;
pub use self::input::Input;
pub use self::lexer::Lexer;
pub use self::reader::{Reader, ReaderResult};
pub use self::readers_builder::ReadersBuilder;
pub use self::readers::{Readers, ReadersIter};
pub use self::state::State;
pub use self::token_error::TokenError;
pub use self::token_meta::TokenMeta;
pub use self::token::Token;
