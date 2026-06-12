use std::borrow::Cow;

/// A byte range in the source input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    /// Byte offset of the first character (0-indexed).
    pub start: usize,
    /// Byte offset of the first character *after* the token (exclusive).
    pub end: usize,
}

/// A [`Token`] paired with its source [`Span`].
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken<'a> {
    pub token: Token<'a>,
    pub span: Span,
}

/// A single token from a JSON source string.
///
/// Tokens are zero-copy where possible — [`String`](Token::String) borrows
/// unescaped text, [`Number`](Token::Number) borrows the raw digit slice,
/// and keywords are unit variants.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// A string value. Borrows the input when no escape sequences are present,
    /// otherwise allocates an owned [`String`] with escapes resolved.
    String(Cow<'a, str>),
    /// A numeric value (integer or floating-point). Borrows the raw source slice.
    Number(&'a str),

    /// The keyword `true`
    True,
    /// The keyword `false`
    False,
    /// The keyword `null`
    Null,

    /// End of input — emitted once before the iterator returns `None`.
    Eof,
}
