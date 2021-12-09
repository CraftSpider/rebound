#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::quote;

mod rebound;

#[proc_macro]
pub fn impl_find(input: TokenStream) -> TokenStream {
    let name = syn::parse::<syn::Ident>(input).unwrap();
    let range = 0u8..255u8;

    quote!(
        #(
        sum.extend(<Self as ReflectedImpl<#range>>::#name());
        )*
    )
    .into()
}

#[proc_macro]
pub fn extern_items(input: TokenStream) -> TokenStream {
    rebound::extern_items(input.into()).into()
}

#[proc_macro]
pub fn extern_assoc_fns(input: TokenStream) -> TokenStream {
    rebound::extern_assoc_fns(input.into()).into()
}

#[proc_macro]
pub fn extern_assoc_consts(input: TokenStream) -> TokenStream {
    rebound::extern_assoc_consts(input.into()).into()
}

#[proc_macro_attribute]
pub fn post_wire(_: TokenStream, input: TokenStream) -> TokenStream {
    rebound::post_wire(input.into()).into()
}

#[proc_macro_attribute]
pub fn rebound(attrs: TokenStream, item: TokenStream) -> TokenStream {
    rebound::rebound(attrs.into(), item.into()).into()
}
