use crate::{JSONLexer, Token};

#[test]
fn lex_integer() {
    let mut lexer = JSONLexer::new("123");
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::Number("123"));
}

#[test]
fn lex_negative_integer() {
    let mut lexer = JSONLexer::new("-123");
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::Number("-123"));
}

#[test]
fn lex_decimal() {
    let mut lexer = JSONLexer::new("123.456");
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::Number("123.456"));
}

#[test]
fn lex_exponent() {
    let mut lexer = JSONLexer::new("1.23e-10");
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::Number("1.23e-10"));
}

#[test]
fn reject_leading_zero() {
    let mut lexer = JSONLexer::new("01");
    assert!(lexer.next_token().is_err());
}

#[test]
fn reject_trailing_decimal() {
    let mut lexer = JSONLexer::new("1.");
    assert!(lexer.next_token().is_err());
}

#[test]
fn reject_incomplete_exponent() {
    let mut lexer = JSONLexer::new("1e");
    assert!(lexer.next_token().is_err());
}

#[test]
fn reject_incomplete_signed_exponent() {
    let mut lexer = JSONLexer::new("1e+");
    assert!(lexer.next_token().is_err());
}

#[test]
fn reject_lone_minus() {
    let mut lexer = JSONLexer::new("-");
    assert!(lexer.next_token().is_err());
}

#[test]
fn lex_string() {
    let mut lexer = JSONLexer::new(r#""hello""#);
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::String("hello".into()));
}

#[test]
fn lex_escape_sequences() {
    let mut lexer = JSONLexer::new(r#""\n\t\r""#);
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::String("\n\t\r".into()));
}

#[test]
fn lex_unicode_escape() {
    let mut lexer = JSONLexer::new(r#""\u0041""#);
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::String("A".into()));
}

#[test]
fn lex_surrogate_pair() {
    let mut lexer = JSONLexer::new(r#""\uD83D\uDE00""#);
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::String("😀".into()));
}

#[test]
fn reject_invalid_escape() {
    let mut lexer = JSONLexer::new(r#""\q""#);
    assert!(lexer.next_token().is_err());
}

#[test]
fn reject_unterminated_string() {
    let mut lexer = JSONLexer::new(r#""hello"#);
    assert!(lexer.next_token().is_err());
}

#[test]
fn reject_invalid_unicode_escape() {
    let mut lexer = JSONLexer::new(r#""\u12G4""#);
    assert!(lexer.next_token().is_err());
}

#[test]
fn lex_true() {
    let mut lexer = JSONLexer::new("true");
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::True);
}

#[test]
fn lex_false() {
    let mut lexer = JSONLexer::new("false");
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::False);
}

#[test]
fn lex_null() {
    let mut lexer = JSONLexer::new("null");
    let result = lexer.next_token().unwrap();
    assert_eq!(result.token, Token::Null);
}

#[test]
fn reject_true_identifier() {
    let mut lexer = JSONLexer::new("trueabc");
    assert!(lexer.next_token().is_err());
}

#[test]
fn iterator() {
    let lexer = JSONLexer::new("true false null");
    let tokens: Vec<Token> = lexer.map(|r| r.unwrap().token).collect();
    assert_eq!(tokens, vec![Token::True, Token::False, Token::Null]);
}

#[test]
fn eof_token() {
    let mut lexer = JSONLexer::new("42");
    assert!(lexer.next_token().is_ok());
    let eof = lexer.next_token().unwrap();
    assert_eq!(eof.token, Token::Eof);
}

#[test]
fn span_positions() {
    let mut lexer = JSONLexer::new(r#"{"key": 42}"#);
    let spanned = lexer.next_token().unwrap();
    assert_eq!(spanned.span.start, 0);
    assert_eq!(spanned.span.end, 1);
    assert_eq!(spanned.token, Token::LeftBrace);

    let spanned = lexer.next_token().unwrap();
    assert_eq!(spanned.token, Token::String("key".into()));
}

#[test]
fn left_brace() {
    let mut lexer = JSONLexer::new("{");
    assert_eq!(lexer.next_token().unwrap().token, Token::LeftBrace);
}

#[test]
fn right_brace() {
    let mut lexer = JSONLexer::new("}");
    assert_eq!(lexer.next_token().unwrap().token, Token::RightBrace);
}

#[test]
fn left_bracket() {
    let mut lexer = JSONLexer::new("[");
    assert_eq!(lexer.next_token().unwrap().token, Token::LeftBracket);
}

#[test]
fn right_bracket() {
    let mut lexer = JSONLexer::new("]");
    assert_eq!(lexer.next_token().unwrap().token, Token::RightBracket);
}

#[test]
fn colon() {
    let mut lexer = JSONLexer::new(":");
    assert_eq!(lexer.next_token().unwrap().token, Token::Colon);
}

#[test]
fn comma() {
    let mut lexer = JSONLexer::new(",");
    assert_eq!(lexer.next_token().unwrap().token, Token::Comma);
}

#[test]
fn multiple_symbols() {
    let lexer = JSONLexer::new("{}[],:");
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().token).collect();
    assert_eq!(
        tokens,
        vec![
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Comma,
            Token::Colon,
        ]
    );
}

#[test]
fn zero_copy_number() {
    let mut lexer = JSONLexer::new("42");
    let result = lexer.next_token().unwrap();
    if let Token::Number(s) = result.token {
        assert_eq!(s, "42");
    } else {
        panic!("expected Number token");
    }
}
