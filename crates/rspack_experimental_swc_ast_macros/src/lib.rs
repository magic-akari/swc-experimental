use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, parse_macro_input};

#[proc_macro_attribute]
pub fn ast(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    let expanded = quote! { #input };
    TokenStream::from(expanded)
}
