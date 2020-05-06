# flex_rs

Flex implementation written in Rust.

Only supports enough functionalities to create a lexer for Cool programming language.

## Usage example (currently it doesn't work)

```rust
#[derive(Default)]
struct Lexer {
    count: i32,
}

impl Lexer {
    flex_rs::flex!{
        A "a"
        
        %%

        A { this.count++; }
    }
}

fn main() {
    let mut lexer = Lexer::default();
    lexer.lex(std::io::stdin()).unwrap();
    println!("{}", lexer.count);
}
```
