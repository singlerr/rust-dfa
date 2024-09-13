extern crate proc_macro;

use quote::ToTokens;

struct LanguageSet{
    characters: Vec<char>
}
struct DFA{

}


pub fn define(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input = proc_macro2::TokenStream::from(input);

    proc_macro2::TokenTree::
    for token in input {
        match token {
            proc_macro2::TokenTree::Group(t) => {

            },
             => {

            }
        }
    }

}

