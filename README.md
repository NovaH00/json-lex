# json-lex

A zero-copy, iterator-based JSON lexer for Rust.

```rust
use json_lex::JSONLexer;

let lexer = JSONLexer::new(r#"{"key": 42}"#);
for result in lexer {
    match result {
        Ok(spanned) => {
            // spanned.token  — Token enum variant
            // spanned.span   — byte Span { start, end } into the source
        }
        Err(e) => eprintln!("{e}"),
    }
}
```

## Features

- **Zero-copy** — strings borrow the input when unescaped; numbers borrow the raw source slice
- **Iterator API** — works with `for`, `.map()`, `.collect()`, adapters
- **Span tracking** — every token carries `Span { start, end }` byte offsets
- **Error reporting** — all errors carry line and column numbers
- **No unsafe code** — `#![forbid(unsafe_code)]`
- **No dependencies** — pure Rust standard library

## Token types

| Token | Example |
|---|---|
| `LeftBrace` / `RightBrace` | `{` `}` |
| `LeftBracket` / `RightBracket` | `[` `]` |
| `Colon` / `Comma` | `:` `,` |
| `String(Cow<'a, str>)` | `"hello"` |
| `Number(&'a str)` | `42`, `-3.14e10` |
| `True` / `False` / `Null` | keywords |
| `Eof` | end of input |

## Examples

```sh
cargo run --example print_tokens
cargo run --example token_count
```

## License

MIT
