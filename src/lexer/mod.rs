mod string;
mod number;
mod keyword;

use crate::error::LexError;
use crate::token::{SpannedToken, Token, Span};

/// A zero-copy, iterator-based JSON lexer.
///
/// Yield [`SpannedToken`] values with byte-level position information.
///
/// # Examples
///
/// ```rust
/// use json_lex::JSONLexer;
///
/// let lexer = JSONLexer::new(r#"{"key": 42}"#);
/// for result in lexer {
///     match result {
///         Ok(spanned) => { /* use spanned.token, spanned.span */ }
///         Err(e) => eprintln!("{e}"),
///     }
/// }
/// ```
pub struct JSONLexer<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> JSONLexer<'a> {
    /// Create a new lexer from a JSON source string.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.input[self.pos..].chars().next()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn read_char(&mut self) -> Option<(char, usize, usize)> {
        let line = self.line;
        let col = self.column;
        let ch = self.next_char()?;
        Some((ch, line, col))
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.next_char();
                }
                _ => break,
            }
        }
    }

    fn expect_keyword(&mut self, expected: &str, first: char) -> Result<(), LexError> {
        let mut found = String::with_capacity(expected.len());
        found.push(first);
        for _ in expected.chars().skip(1) {
            match self.read_char() {
                Some((c, _, _)) => found.push(c),
                None => break,
            }
        }
        if found == expected {
            Ok(())
        } else {
            Err(LexError::InvalidToken {
                reason: format!("expected \"{}\", found \"{}\"", expected, found),
                line: self.line,
                column: self.column,
            })
        }
    }

    fn ensure_keyword_terminated(&self) -> Result<(), LexError> {
        if let Some(ch) = self.peek_char()
            && (ch.is_ascii_alphanumeric() || ch == '_')
        {
            return Err(LexError::InvalidToken {
                reason: format!(
                    "keyword must be followed by whitespace, EOF, or a JSON delimiter, found '{}'",
                    ch
                ),
                line: self.line,
                column: self.column,
            });
        }
        Ok(())
    }

    /// Advance to the next token, skipping whitespace.
    ///
    /// Returns [`Eof`](Token::Eof) once the input is exhausted.
    /// After an error the lexer position is unspecified; discard and
    /// create a new lexer to continue.
    pub fn next_token(&mut self) -> Result<SpannedToken<'a>, LexError> {
        self.skip_whitespace();
        let start = self.pos;
        let (ch, line, col) = match self.read_char() {
            None => {
                return Ok(SpannedToken {
                    token: Token::Eof,
                    span: Span { start, end: start },
                });
            }
            Some(v) => v,
        };

        let spanned = match ch {
            '{' => Ok(SpannedToken {
                token: Token::LeftBrace,
                span: Span { start, end: self.pos },
            }),
            '}' => Ok(SpannedToken {
                token: Token::RightBrace,
                span: Span { start, end: self.pos },
            }),
            '[' => Ok(SpannedToken {
                token: Token::LeftBracket,
                span: Span { start, end: self.pos },
            }),
            ']' => Ok(SpannedToken {
                token: Token::RightBracket,
                span: Span { start, end: self.pos },
            }),
            ':' => Ok(SpannedToken {
                token: Token::Colon,
                span: Span { start, end: self.pos },
            }),
            ',' => Ok(SpannedToken {
                token: Token::Comma,
                span: Span { start, end: self.pos },
            }),
            '"' => self.lex_string(),
            '0'..='9' | '-' => self.lex_number(ch),
            't' => self.lex_true(),
            'f' => self.lex_false(),
            'n' => self.lex_null(),
            _ => Err(LexError::UnexpectedChar {
                ch,
                line,
                column: col,
            }),
        }?;

        Ok(spanned)
    }
}

impl<'a> Iterator for JSONLexer<'a> {
    type Item = Result<SpannedToken<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(spanned) if spanned.token == Token::Eof => None,
            other => Some(other),
        }
    }
}
