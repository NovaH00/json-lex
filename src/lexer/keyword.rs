use crate::error::LexError;
use crate::token::{Token, Span, SpannedToken};
use super::JSONLexer;

impl<'a> JSONLexer<'a> {
    pub fn lex_true(&mut self) -> Result<SpannedToken<'a>, LexError> {
        let start = self.pos - 1;
        self.expect_keyword("true", 't')?;
        self.ensure_keyword_terminated()?;

        Ok(SpannedToken {
            token: Token::True,
            span: Span { start, end: self.pos },
        })
    }

    pub fn lex_false(&mut self) -> Result<SpannedToken<'a>, LexError> {
        let start = self.pos - 1;
        self.expect_keyword("false", 'f')?;
        self.ensure_keyword_terminated()?;

        Ok(SpannedToken {
            token: Token::False,
            span: Span { start, end: self.pos },
        })
    }

    pub fn lex_null(&mut self) -> Result<SpannedToken<'a>, LexError> {
        let start = self.pos - 1;
        self.expect_keyword("null", 'n')?;
        self.ensure_keyword_terminated()?;

        Ok(SpannedToken {
            token: Token::Null,
            span: Span { start, end: self.pos },
        })
    }
}
