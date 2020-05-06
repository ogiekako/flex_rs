#![feature(proc_macro_hygiene)]
use std::io;

struct Lexer {}

impl Lexer {
    flex_rs::flex! {
        NL "\n"
        CLASS "class"
        %%

        NL {
            fn new() -> i32 {
                42
            }
        }
    }
}

fn lex(mut r: impl io::Read) -> Result<(), io::Error> {
    let mut b = [0; 1];
    r.read(&mut b)?;
    println!("{}", String::from_utf8_lossy(&b));
    Ok(())
}

fn main() {
    lex(std::io::stdin()).unwrap();
    flex_rs::flex!(
        NL "\n"
        CLASS "class"
        %%

        NL {
            let x = "foo";
            println!("{} {}", x, Lexer::new());
        }
    );
}
