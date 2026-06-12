use crate::error::LexError;
use crate::token::{SpannedToken, Token, Span};
use super::JSONLexer;

impl<'a> JSONLexer<'a> {
    fn read_hex4(&mut self) -> Result<u16, LexError> {
        let mut value: u16 = 0;

        for _ in 0..4 {
            let (ch, line, col) = match self.read_char() {
                Some(v) => v,
                None => {
                    return Err(LexError::InvalidEscape {
                        reason: "unexpected EOF in unicode escape".to_string(),
                        line: self.line,
                        column: self.column,
                    });
                }
            };

            let digit = match ch.to_digit(16) {
                Some(v) => v as u16,
                None => {
                    return Err(LexError::InvalidEscape {
                        reason: format!(
                            "expected hexadecimal digit, found '{}'",
                            ch
                        ),
                        line,
                        column: col,
                    });
                }
            };

            value = (value << 4) | digit;
        }

        Ok(value)
    }

    pub fn lex_string(&mut self) -> Result<SpannedToken<'a>, LexError> {
        let start = self.pos;
        let mut processed: Option<String> = None;

        loop {
            let (ch, line, col) = match self.read_char() {
                Some(v) => v,
                None => {
                    return Err(LexError::UnterminatedString {
                        line: self.line,
                        column: self.column,
                    });
                }
            };

            match ch {
                '"' => {
                    let token = match processed {
                        Some(s) => Token::String(s.into()),
                        None => Token::String((&self.input[start..self.pos - 1]).into()),
                    };
                    return Ok(SpannedToken {
                        token,
                        span: Span { start, end: self.pos },
                    });
                }

                '\\' => {
                    let acc = processed.get_or_insert_with(|| {
                        self.input[start..self.pos - 1].to_string()
                    });

                    let (escaped, eline, ecol) = match self.read_char() {
                        Some(v) => v,
                        None => {
                            return Err(LexError::UnterminatedString {
                                line: self.line,
                                column: self.column,
                            });
                        }
                    };

                    match escaped {
                        '"' => acc.push('"'),
                        '\\' => acc.push('\\'),
                        '/' => acc.push('/'),
                        'n' => acc.push('\n'),
                        't' => acc.push('\t'),
                        'r' => acc.push('\r'),
                        'b' => acc.push('\x08'),
                        'f' => acc.push('\x0C'),

                        'u' => {
                            let first = self.read_hex4()?;

                            if (0xD800..=0xDBFF).contains(&first) {
                                match self.read_char() {
                                    Some(('\\', _, _)) => {}
                                    Some((_, l, ccol)) => {
                                        return Err(LexError::InvalidEscape {
                                            reason: "expected second surrogate pair".to_string(),
                                            line: l,
                                            column: ccol,
                                        });
                                    }
                                    None => {
                                        return Err(LexError::InvalidEscape {
                                            reason: "expected second surrogate pair".to_string(),
                                            line: self.line,
                                            column: self.column,
                                        });
                                    }
                                }

                                match self.read_char() {
                                    Some(('u', _, _)) => {}
                                    Some((_, l, ccol)) => {
                                        return Err(LexError::InvalidEscape {
                                            reason: "expected '\\u' for second surrogate pair".to_string(),
                                            line: l,
                                            column: ccol,
                                        });
                                    }
                                    None => {
                                        return Err(LexError::InvalidEscape {
                                            reason: "expected '\\u' for second surrogate pair".to_string(),
                                            line: self.line,
                                            column: self.column,
                                        });
                                    }
                                }

                                let second = self.read_hex4()?;

                                if !(0xDC00..=0xDFFF).contains(&second) {
                                    return Err(LexError::InvalidEscape {
                                        reason: "invalid low surrogate".to_string(),
                                        line: self.line,
                                        column: self.column,
                                    });
                                }

                                let codepoint =
                                    0x10000
                                    + (((first as u32 - 0xD800) << 10)
                                    | (second as u32 - 0xDC00));

                                match char::from_u32(codepoint) {
                                    Some(c) => acc.push(c),
                                    None => {
                                        return Err(LexError::InvalidEscape {
                                            reason: "invalid unicode codepoint".to_string(),
                                            line: self.line,
                                            column: self.column,
                                        });
                                    }
                                }
                            }
                            else if (0xDC00..=0xDFFF).contains(&first) {
                                return Err(LexError::InvalidEscape {
                                    reason: "unexpected low surrogate".to_string(),
                                    line: self.line,
                                    column: self.column,
                                });
                            }
                            else {
                                match char::from_u32(first as u32) {
                                    Some(c) => acc.push(c),
                                    None => {
                                        return Err(LexError::InvalidEscape {
                                            reason: "invalid unicode codepoint".to_string(),
                                            line: self.line,
                                            column: self.column,
                                        });
                                    }
                                }
                            }
                        }

                        _ => {
                            return Err(LexError::InvalidEscape {
                                reason: format!(
                                    "invalid escape sequence '\\{}'",
                                    escaped
                                ),
                                line: eline,
                                column: ecol,
                            });
                        }
                    }
                }

                c if c.is_control() => {
                    return Err(LexError::InvalidString {
                        reason: "control characters are not allowed".to_string(),
                        line,
                        column: col,
                    });
                }

                _ => {
                    if let Some(acc) = &mut processed {
                        acc.push(ch);
                    }
                }
            }
        }
    }
}
