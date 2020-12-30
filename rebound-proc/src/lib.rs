#![feature(once_cell, proc_macro_diagnostic)]

use core::ops::Range;

use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};

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
pub fn reflect_prims(input: TokenStream) -> TokenStream {
    let info = <fn(syn::parse::ParseStream) -> syn::Result<syn::punctuated::Punctuated<syn::Type, syn::Token![,]>> as syn::parse::Parser>::parse2(
        syn::punctuated::Punctuated::<_, _>::parse_terminated,
        input.into()
    )
        .unwrap();

    let mut out = proc_macro2::TokenStream::new();

    for i in info {
        out.append_all(quote!(
            impl Reflected for #i {
                fn name() -> String {
                    stringify!(#i).into()
                }

                fn assemble(meta: Self::Meta, ptr: *mut ()) -> *mut Self {
                    ptr as _
                }

                fn disassemble(&self) -> (Self::Meta, *mut ()) {
                    ((), self as *const Self as _)
                }

                unsafe fn init() {
                    Type::new_prim::<#i>()
                }
            }
        ))
    }

    out.into()
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
