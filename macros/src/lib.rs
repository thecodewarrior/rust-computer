#![feature(proc_macro_hygiene)]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
#[macro_use] extern crate quote;
#[macro_use] extern crate syn;

use proc_macro::TokenStream;
use syn::{Expr, LitStr, LitInt};
use syn::parse::{Parse, ParseStream, Result};

struct TestInput {
    expression: Expr,
    pattern: LitStr
}

impl Parse for TestInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let expression: Expr = input.parse()?;
        input.parse::<Token![;]>()?;
        let pattern: LitStr = input.parse()?;
        Ok(TestInput {
            expression,
            pattern
        })
    }
}

#[proc_macro]
pub fn bits(input: TokenStream) -> TokenStream {
    let TestInput { expression, pattern } = parse_macro_input!(input as TestInput);

    let pattern_string: String = pattern.value();
    if !pattern_string.chars().all(|c| c == '0' || c == '1' || c == '_' || c == 'x') {
        pattern.span().unwrap()
        .error("Pattern strings can only contain the characters 0, 1, _, and x.")
        .emit();
        return TokenStream::new();
    }

    let mask_string = pattern_string.as_str().replace("0", "1").replace("x", "0");
    let test_string = pattern_string.as_str().replace("x", "0");
    let mask = LitInt::new(format!("0b{}", mask_string).as_str(), pattern.span());
    let test = LitInt::new(format!("0b{}", test_string).as_str(), pattern.span());

    let result = quote! {
        #expression & #mask == #test
    };

    TokenStream::from(result)
}