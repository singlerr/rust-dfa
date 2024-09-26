use quote::ToTokens;

mod dfa;

/// How to define marco?
///
#[proc_macro]
pub fn define(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    for token in input {
        match token {
            (any) => {
                println!("{:?}", any);
            }
        }
    }

    "println!(\"\")".parse().unwrap()
}
