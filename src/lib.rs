use proc_macro::TokenStream;
use quote::quote;
use syn;
use crate::spec_macros::impl_spec_macro;
mod spec_macros;

#[proc_macro_derive(FromSpec)]
pub fn from_spec_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_spec_macro(&ast)
}