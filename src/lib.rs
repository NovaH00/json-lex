#![forbid(unsafe_code)]

mod lexer;
mod error;
mod token;

pub use lexer::JSONLexer;
pub use error::LexError;
pub use token::{Token, Span, SpannedToken};

#[cfg(test)]
mod tests;
