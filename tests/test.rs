#![feature(proc_macro_hygiene)]
use flex_rs::SelfName;

#[derive(SelfName)]
struct Hoge {}
#[test]
fn f() {
    let hoge = Hoge {};
    assert_eq!(hoge.self_name(), "Hoge");
}

#[test]
fn hello_test() {
    assert_eq!(flex_rs::make_hello!("hoge"), "Hello hoge");
}

// #[test]
// fn flex_test() {
//     myflex::flex! {
//         Class "c"
//         %%
//     };
//     myflex::flex! {
//         Class "c"
//         %c
//     };
//     myflex::flex! {
//         Class "c"
//         NL "\n"
//         %c
//     };
// }
