use json_lex::JSONLexer;

fn main() {
    let json = r#"{
        "numbers": [1, 2, 3, 4, 5],
        "nested": { "a": 1.5, "b": -2e10 }
    }"#;

    let lexer = JSONLexer::new(json);
    let mut counts = std::collections::BTreeMap::new();
    let mut errors = 0;

    for result in lexer {
        match result {
            Ok(spanned) => {
                let label = match spanned.token {
                    json_lex::Token::LeftBrace => "LeftBrace",
                    json_lex::Token::RightBrace => "RightBrace",
                    json_lex::Token::LeftBracket => "LeftBracket",
                    json_lex::Token::RightBracket => "RightBracket",
                    json_lex::Token::Colon => "Colon",
                    json_lex::Token::Comma => "Comma",
                    json_lex::Token::String(_) => "String",
                    json_lex::Token::Number(_) => "Number",
                    json_lex::Token::True | json_lex::Token::False => "Boolean",
                    json_lex::Token::Null => "Null",
                    json_lex::Token::Eof => "Eof",
                };
                *counts.entry(label).or_insert(0) += 1;
            }
            Err(e) => {
                eprintln!("Error: {e}");
                errors += 1;
            }
        }
    }

    println!("Token counts:");
    for (kind, count) in &counts {
        println!("  {kind:>12}: {count}");
    }
    if errors > 0 {
        println!("\n{errors} error(s) encountered");
    }
}
