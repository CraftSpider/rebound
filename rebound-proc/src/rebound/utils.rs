
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Lifetime;
use syn::spanned::Spanned;

pub fn path_to_string(path: &syn::Path) -> String {
    path.segments.iter()
        .map(
            |seg| seg.to_token_stream().to_string()
        )
        .collect::<Vec<_>>()
        .join("::")
}

pub fn lit_as_str(lit: &syn::Lit) -> Result<String, String> {
    match lit {
        syn::Lit::Str(str) => Ok(str.value()),
        _ => Err("Expected valid identifier for literal".to_string())
    }
}

pub fn static_or_anon(life: &Lifetime) -> Lifetime {
    if life.ident == "static" {
        life.clone()
    } else {
        Lifetime::new("'_", life.span())
    }
}

pub fn item_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    let ty_generics = generics.params
        .iter()
        .map(|param| {
            match param {
                syn::GenericParam::Lifetime(..) => quote!('_),
                syn::GenericParam::Type(param) => quote!(#param),
                syn::GenericParam::Const(param) => quote!(#param)
            }
        })
        .collect::<Vec<_>>();

    quote!(#name<#(#ty_generics,)*>)
}

pub fn item_pattern_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    let ty_generics = generics.params
        .iter()
        .map(|param| {
            match param {
                syn::GenericParam::Lifetime(..) => quote!('_),
                syn::GenericParam::Type(param) => quote!(#param),
                syn::GenericParam::Const(param) => quote!(#param)
            }
        })
        .collect::<Vec<_>>();

    quote!(#name::<#(#ty_generics,)*>)
}

pub fn item_qual_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    let ty_generics = generics.params
        .iter()
        .map(|param| {
            match param {
                syn::GenericParam::Lifetime(_) => TokenStream::new(),
                syn::GenericParam::Type(param) => {
                    let ident = &param.ident;
                    quote!(#ident::name())
                },
                syn::GenericParam::Const(param) => {
                    let ident = &param.ident;
                    quote!(#ident)
                }
            }
        })
        .filter(|param| !param.is_empty())
        .collect::<Vec<_>>();

    let fmt_str;
    if ty_generics.is_empty() {
        fmt_str = "{}::{}".to_string()
    } else {
        fmt_str = format!(
            "{{}}::{{}}<{}>",
            ty_generics.iter().map(|_| "{}").collect::<Vec<_>>().join(", ")
        )
    }

    quote!( format!(#fmt_str, module_path!(), stringify!(#name), #(#ty_generics,)* ) )
}

pub fn sanitized_field_ty(ty: &syn::Type) -> TokenStream {
    match ty {
        syn::Type::Array(ty) => {
            let elem = sanitized_field_ty(&ty.elem);
            let len = &ty.len;

            quote!([#elem; #len])
        }
        syn::Type::Reference(ty) => {
            let lifetime = ty.lifetime.as_ref().map(|life| {
                static_or_anon(life)
            });
            let mutability = &ty.mutability;
            let inner = &ty.elem;

            quote!(& #lifetime #mutability #inner)
        },
        syn::Type::Path(ty) => {
            let segments = ty.path.segments
                .iter()
                .map(|seg| {
                    match &seg.arguments {
                        syn::PathArguments::AngleBracketed(args) => {
                            let ident = &seg.ident;
                            let ty_args = args.args.iter()
                                .map(|arg| {
                                    match arg {
                                        syn::GenericArgument::Lifetime(life) => {
                                            let life = static_or_anon(life);
                                            quote!(#life)
                                        }
                                        syn::GenericArgument::Type(ty) => {
                                            sanitized_field_ty(ty)
                                        }
                                        _ => quote!(#arg)
                                    }
                                })
                                .collect::<Vec<_>>();

                            quote!( #ident<#(#ty_args,)*> )
                        }
                        syn::PathArguments::Parenthesized(_args) => {
                            todo!()
                        }
                        syn::PathArguments::None => {
                            quote!(#seg)
                        }
                    }
                })
                .collect::<Vec<_>>();

            quote!( #(#segments)::* )
        }
        _ => quote!(#ty)
    }
}

pub fn impl_bounds(generics: &syn::Generics) -> TokenStream {
    let impl_bounds = generics.params
        .iter()
        .map(|param| {
            match param {
                syn::GenericParam::Lifetime(..) => TokenStream::new(),
                syn::GenericParam::Type(param) => quote!( #param: rebound::Reflected ),
                syn::GenericParam::Const(param) => quote!( #param ),
            }
        })
        .filter(|ts| !ts.is_empty())
        .collect::<Vec<_>>();

    quote!(<#(#impl_bounds,)*>)
}
