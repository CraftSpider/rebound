#![feature(once_cell, proc_macro_diagnostic)]

use std::ops::Range;

use proc_macro::TokenStream;
use quote::quote;

mod rebound;

#[proc_macro]
pub fn impl_find(input: TokenStream) -> TokenStream {
    let name: syn::Ident = syn::parse(input).unwrap();
    let range: Range<u8> = 0u8..255u8;

    quote!(
        #(
        sum.extend(<Self as ReflectedImpl<#range>>::#name());
        )*
    )
    .into()
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
pub fn rebound(attrs: TokenStream, item: TokenStream) -> TokenStream {
    rebound::rebound(attrs.into(), item.into()).into()
}
