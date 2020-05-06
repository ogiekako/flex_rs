#![feature(proc_macro_hygiene)]
#![feature(todo_macro)]

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{braced, parse::Parse, parse_macro_input, token, Ident, ItemStruct, LitStr, Token};

struct Regex {}

#[derive(Debug)]
struct NameDefinition {
    name: String,
    regex: String,
}

impl Parse for NameDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let regex: LitStr = input.parse()?;
        Ok(NameDefinition {
            name: name.to_string(),
            regex: regex.value(),
        })
    }
}
#[derive(Debug)]
struct StartCond {
    // TODO: impl.
}

#[derive(Debug, Default)]
// Flex's definitions section.
struct Definitions {
    name_defs: Vec<NameDefinition>,
    starts: Vec<StartCond>,
}

impl Parse for Definitions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut res = Definitions::default();

        loop {
            if input.peek(Token![%]) {
                input.parse::<Token![%]>();
                if input.peek(Token![%]) {
                    input.parse::<Token![%]>();
                    return Ok(res);
                }
                let switch: Ident = input.parse()?;
                match switch.to_string().as_ref() {
                    "c" => {
                        // impl start condition.
                        // http://westes.github.io/flex/manual/Start-Conditions.html#Start-Conditions
                        todo!();
                    }
                    "x" => {
                        todo!();
                    }
                    _ => {
                        // emit error.
                        todo!();
                        // switch.span().unwrap().error("unknown parameter").emit();
                    }
                }
            } else {
                res.name_defs.push(input.parse()?);
            }
        }
    }
}

#[derive(Debug)]
struct Rule {
    // Definitions name.
    def: String,
    brace_token: token::Brace,
    content: proc_macro2::TokenStream,
}

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Rule {
            def: input.parse::<Ident>()?.to_string(),
            brace_token: braced!(content in input),
            content: content.parse()?,
        })
    }
}

#[derive(Debug, Default)]
struct Rules {
    rules: Vec<Rule>,
}

impl Parse for Rules {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut res = Rules::default();
        while !input.is_empty() {
            res.rules.push(input.parse()?);
        }
        Ok(res)
    }
}

#[derive(Debug, Default)]
struct Flex {
    definitions: Definitions,
    rules: Rules,
}

impl Parse for Flex {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Flex {
            definitions: input.parse()?,
            rules: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn flex(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Flex);

    let content = &item.rules.rules[0].content;
    quote!(#content).into()
}

////////////////////////////////// sandbox ///////////////////////////////////
#[proc_macro_derive(SelfName)]
pub fn derive_self_name(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let struct_name = item.ident;
    let gen = quote! {
        impl #struct_name {
            pub fn self_name(&self) -> &str {
                stringify!(#struct_name)
            }
        }
    };
    gen.into()
}

struct Hello {
    name: String,
}

impl Parse for Hello {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::LitStr = input.parse()?;
        Ok(Hello { name: name.value() })
    }
}

// Function-like procedural macros
#[proc_macro]
pub fn make_hello(item: TokenStream) -> TokenStream {
    let h = parse_macro_input!(item as Hello);
    let name = h.name;
    quote!(format!("Hello {}", #name)).into()
}
