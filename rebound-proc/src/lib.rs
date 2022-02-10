#![feature(proc_macro_diagnostic)]
#![warn(
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    missing_abi,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    unused_lifetimes,
    clippy::cargo,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::unreadable_literal,
)]

use proc_macro::TokenStream;
use quote::quote;

mod rebound;
mod extension;
mod error;

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
pub fn rebound(attrs: TokenStream, item: TokenStream) -> TokenStream {
    rebound::rebound(attrs.into(), item.into()).into()
}
