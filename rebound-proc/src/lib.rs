
use std::ops::Range;

use proc_macro::TokenStream;
use quote::quote;

mod rebound;

#[proc_macro]
pub fn impl_find(input: TokenStream) -> TokenStream {
    let name: syn::Ident = syn::parse(input).unwrap();
    let range: Range<u8> = 0..255;

    quote!(
        #(
        let cur = <Self as ReflectedImpl<#range>>::#name();
        match cur {
            Some(a) => {
                sum.extend(a);
            },
            None => return sum,
        }
        )*
    ).into()
}

#[proc_macro_attribute]
pub fn rebound(input: TokenStream, attrs: TokenStream) -> TokenStream {
    rebound::rebound(input.into(), attrs.into()).into()
}
