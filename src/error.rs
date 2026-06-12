use std::fmt;

/// Errors that can occur during JSON lexing.
///
/// Every variant carries the `line` and `column` where the error was detected.
#[derive(Debug)]
pub enum LexError {
    /// A character that cannot start any token.
    UnexpectedChar { ch: char, line: usize, column: usize },
    /// End-of-file reached inside a string literal.
    UnterminatedString { line: usize, column: usize },
    /// A string literal violates JSON rules (e.g. bare control character).
    InvalidString { reason: String, line: usize, column: usize },
    /// A number literal violates JSON grammar (e.g. leading zero, trailing decimal).
    InvalidNumber { reason: String, line: usize, column: usize },
    /// An invalid escape sequence inside a string.
    InvalidEscape { reason: String, line: usize, column: usize },
    /// A keyword-like token that doesn't match `true`/`false`/`null`.
    InvalidToken { reason: String, line: usize, column: usize },
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnexpectedChar { ch, line, column } => {
                write!(f, "unexpected character '{}' at {}:{}", ch, line, column)
            }
            LexError::UnterminatedString { line, column } => {
                write!(f, "unterminated string at {}:{}", line, column)
            }
            LexError::InvalidString { reason, line, column } => {
                write!(f, "invalid string at {}:{}: {}", line, column, reason)
            }
            LexError::InvalidNumber { reason, line, column } => {
                write!(f, "invalid number at {}:{}: {}", line, column, reason)
            }
            LexError::InvalidEscape { reason, line, column } => {
                write!(f, "invalid escape at {}:{}: {}", line, column, reason)
            }
            LexError::InvalidToken { reason, line, column } => {
                write!(f, "invalid token at {}:{}: {}", line, column, reason)
            }
        }
    }
}

impl std::error::Error for LexError {}
