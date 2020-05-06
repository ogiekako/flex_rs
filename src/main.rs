#![feature(proc_macro_hygiene)]

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

fn main() {
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
