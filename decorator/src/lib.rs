extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, token::Token, ItemFn};

#[proc_macro_attribute]
pub fn state_safe(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(input as ItemFn);

    let check_owner = quote! {
            assert!(self.owner == String::from("bob"), "Can only be called by stream owner.");
    };

    function
        .block
        .stmts
        .insert(0, syn::parse2(check_owner).unwrap());

    TokenStream::from(quote! { #function })
}
