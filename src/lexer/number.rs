use crate::error::LexError;
use crate::token::{Token, Span, SpannedToken};
use super::JSONLexer;

impl<'a> JSONLexer<'a> {
    pub fn lex_number(&mut self, first: char) -> Result<SpannedToken<'a>, LexError> {
        let start = self.pos - first.len_utf8();

        //
        // Sign
        //
        let first_digit = if first == '-' {
            match self.peek_char() {
                Some(ch @ '0'..='9') => {
                    self.next_char();
                    ch
                }
                _ => {
                    return Err(LexError::InvalidNumber {
                        reason: "expected digit after '-'".to_string(),
                        line: self.line,
                        column: self.column,
                    });
                }
            }
        } else {
            first
        };

        //
        // Integer part
        //
        if first_digit == '0' {
            if let Some('0'..='9') = self.peek_char() {
                return Err(LexError::InvalidNumber {
                    reason: "leading zeros are not allowed".to_string(),
                    line: self.line,
                    column: self.column,
                });
            }
        } else {
            while let Some('0'..='9') = self.peek_char() {
                self.next_char();
            }
        }

        //
        // Fraction
        //
        if let Some('.') = self.peek_char() {
            self.next_char();

            match self.peek_char() {
                Some('0'..='9') => {
                    self.next_char();
                }
                _ => {
                    return Err(LexError::InvalidNumber {
                        reason: "expected digit after decimal point".to_string(),
                        line: self.line,
                        column: self.column,
                    });
                }
            };

            while let Some('0'..='9') = self.peek_char() {
                self.next_char();
            }
        }

        //
        // Exponent
        //
        if let Some('e' | 'E') = self.peek_char() {
            self.next_char();

            if let Some('+' | '-') = self.peek_char() {
                self.next_char();
            }

            match self.peek_char() {
                Some('0'..='9') => {
                    self.next_char();
                }
                _ => {
                    return Err(LexError::InvalidNumber {
                        reason: "expected digit in exponent".to_string(),
                        line: self.line,
                        column: self.column,
                    });
                }
            }

            while let Some('0'..='9') = self.peek_char() {
                self.next_char();
            }
        }

        //
        // Number terminator
        //
        if let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' | '\n' | '\r'
                | ',' | ']' | '}' | ':' => {}

                _ => {
                    return Err(LexError::InvalidNumber {
                        reason: format!("invalid character after number: '{}'", ch),
                        line: self.line,
                        column: self.column,
                    });
                }
            }
        }

        let end = self.pos;
        Ok(SpannedToken {
            token: Token::Number(&self.input[start..end]),
            span: Span { start, end },
        })
    }
}
