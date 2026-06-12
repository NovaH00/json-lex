use json_lex::JSONLexer;

fn main() {
    let json = r#"{
        "name": "Alice",
        "age": 30,
        "active": true,
        "data": null
    }"#;

    let lexer = JSONLexer::new(json);
    for result in lexer {
        match result {
            Ok(spanned) => {
                let slice = &json[spanned.span.start..spanned.span.end];
                println!(
                    "{:?}  {:12?}  {:?}",
                    spanned.span, spanned.token, slice
                );
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}
