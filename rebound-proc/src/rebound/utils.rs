use super::Config;

use std::iter::FromIterator;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Lifetime;
use syn::Token;

pub fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.to_token_stream().to_string())
        .collect::<Vec<_>>()
        .join("::")
}

pub fn lit_as_str(lit: &syn::Lit) -> Result<String, String> {
    match lit {
        syn::Lit::Str(str) => Ok(str.value()),
        _ => Err("Expected valid identifier for literal".to_string()),
    }
}

pub fn static_or_anon(life: &Lifetime) -> Lifetime {
    if life.ident == "static" {
        life.clone()
    } else {
        Lifetime::new("'_", life.span())
    }
}

fn build_name<F>(name: &syn::Ident, generics: &syn::Generics, f: F) -> TokenStream
where
    F: FnMut(&syn::GenericParam) -> TokenStream,
{
    let ty_generics = generics.params.iter().map(f).collect::<Vec<_>>();

    quote!(#name::<#(#ty_generics,)*>)
}

pub fn item_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    build_name(name, generics, |param| match param {
        syn::GenericParam::Lifetime(..) => quote!('_),
        syn::GenericParam::Type(syn::TypeParam { ident, .. })
        | syn::GenericParam::Const(syn::ConstParam { ident, .. }) => quote!(#ident),
    })
}

pub fn item_static_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    build_name(name, generics, |param| match param {
        syn::GenericParam::Lifetime(..) => quote!('static),
        syn::GenericParam::Type(syn::TypeParam { ident, .. }) => quote!(#ident::Key),
        syn::GenericParam::Const(syn::ConstParam { ident, .. }) => quote!(#ident),
    })
}

pub fn item_qual_name(cfg: &Config, name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    let ty_generics = generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(_) => TokenStream::new(),
            syn::GenericParam::Type(param) => {
                let ident = &param.ident;
                quote!(#ident::name())
            }
            syn::GenericParam::Const(param) => {
                let ident = &param.ident;
                quote!(#ident)
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
            ty_generics
                .iter()
                .map(|_| "{}")
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    let module_path = if let Some((mat, rep)) = &cfg.name_replace {
        quote!({ module_path!().replace(#mat, #rep) })
    } else {
        quote!(module_path!())
    };

    quote!(format!(#fmt_str, #module_path, stringify!(#name), #(#ty_generics,)* ))
}

pub fn sanitized_field_ty(ty: &syn::Type) -> TokenStream {
    match ty {
        syn::Type::Array(ty) => {
            let elem = sanitized_field_ty(&ty.elem);
            let len = &ty.len;

            quote!([#elem; #len])
        }
        syn::Type::Reference(ty) => {
            let lifetime = ty.lifetime.as_ref().map(|life| static_or_anon(life));
            let mutability = &ty.mutability;
            let inner = &ty.elem;

            quote!(& #lifetime #mutability #inner)
        }
        syn::Type::Path(ty) => {
            let segments = ty
                .path
                .segments
                .iter()
                .map(|seg| match &seg.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        let ident = &seg.ident;
                        let ty_args = args
                            .args
                            .iter()
                            .map(|arg| match arg {
                                syn::GenericArgument::Lifetime(life) => {
                                    let life = static_or_anon(life);
                                    quote!(#life)
                                }
                                syn::GenericArgument::Type(ty) => sanitized_field_ty(ty),
                                _ => quote!(#arg),
                            })
                            .collect::<Vec<_>>();

                        quote!( #ident<#(#ty_args,)*> )
                    }
                    syn::PathArguments::Parenthesized(args) => {
                        let ident = &seg.ident;
                        let in_args = args
                            .inputs
                            .iter()
                            .map(sanitized_field_ty)
                            .collect::<Vec<_>>();
                        let out_arg = &args.output;

                        quote!( #ident(#(#in_args),*) #out_arg)
                    }
                    syn::PathArguments::None => {
                        quote!(#seg)
                    }
                })
                .collect::<Vec<_>>();

            quote!( #(#segments)::* )
        }
        _ => quote!(#ty),
    }
}

fn is_maybe(bound: &syn::TypeParamBound) -> bool {
    if let syn::TypeParamBound::Trait(bound) = bound {
        matches!(bound.modifier, syn::TraitBoundModifier::Maybe(_))
    } else {
        false
    }
}

pub fn impl_bounds(cfg: &Config, generics: &syn::Generics) -> (TokenStream, TokenStream) {
    let reflected_bound = syn::TypeParamBound::Trait(syn::TraitBound {
        paren_token: None,
        modifier: syn::TraitBoundModifier::None,
        lifetimes: None,
        path: syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(vec![
                syn::PathSegment::from(cfg.crate_name.clone()),
                syn::PathSegment::from(syn::Ident::new("Reflected", Span::call_site())),
            ]),
        },
    });

    let mut impl_bounds: Punctuated<syn::GenericParam, Token![,]> = Punctuated::new();
    let mut clauses: Punctuated<syn::WherePredicate, Token![,]> = Punctuated::new();

    if let Some(clause) = &generics.where_clause {
        clauses.extend(clause.predicates.iter().cloned());
    }

    generics.params.iter().for_each(|param| match param {
        syn::GenericParam::Lifetime(param) => {
            let found = clauses.iter_mut().any(|clause| {
                if let syn::WherePredicate::Lifetime(life) = clause {
                    if life.lifetime == param.lifetime {
                        life.bounds.extend(param.bounds.iter().cloned());
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            if !found {
                clauses.push(syn::WherePredicate::Lifetime(syn::PredicateLifetime {
                    lifetime: param.lifetime.clone(),
                    colon_token: syn::token::Colon(Span::call_site()),
                    bounds: param.bounds.clone(),
                }));
            }

            impl_bounds.push(syn::GenericParam::Lifetime(syn::LifetimeDef {
                attrs: vec![],
                lifetime: param.lifetime.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
            }));
        }
        syn::GenericParam::Type(param) => {
            let found = clauses.iter_mut().any(|clause| {
                if let syn::WherePredicate::Type(pred_ty) = clause {
                    if let syn::Type::Path(path) = &pred_ty.bounded_ty {
                        if path
                            .path
                            .get_ident()
                            .map_or(false, |ident| param.ident == *ident)
                        {
                            pred_ty.bounds.extend(param.bounds.iter().cloned());
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            if !found {
                clauses.push(syn::WherePredicate::Type(syn::PredicateType {
                    lifetimes: None,
                    bounded_ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path::from(param.ident.clone()),
                    }),
                    colon_token: syn::token::Colon(Span::call_site()),
                    bounds: param
                        .bounds
                        .iter()
                        .cloned()
                        .filter(|bound| !is_maybe(bound))
                        .collect(),
                }))
            }

            let bounds: Punctuated<syn::TypeParamBound, Token![+]> =
                param.bounds.iter().cloned().filter(is_maybe).collect();
            let colon_token = if bounds.is_empty() {
                None
            } else {
                Some(syn::token::Colon(Span::call_site()))
            };

            impl_bounds.push(syn::GenericParam::Type(syn::TypeParam {
                attrs: vec![],
                ident: param.ident.clone(),
                colon_token,
                bounds,
                eq_token: None,
                default: None,
            }))
        }
        syn::GenericParam::Const(_) => impl_bounds.push(param.clone()),
    });

    let mut key_bounds: Vec<syn::WherePredicate> = Vec::new();

    clauses.iter_mut().for_each(|clause| {
        if let syn::WherePredicate::Type(pred_ty) = clause {
            pred_ty.bounds.push(reflected_bound.clone());

            if let syn::Type::Path(path) = &pred_ty.bounded_ty {
                let mut new_path = path.path.segments.clone();
                new_path.push(syn::PathSegment::from(syn::Ident::new(
                    "Key",
                    Span::call_site(),
                )));

                let is_unsized = impl_bounds.iter().any(|param| {
                    if let syn::GenericParam::Type(ty) = param {
                        ty.ident == *path.path.get_ident().unwrap()
                            && ty.bounds.iter().any(is_maybe)
                    } else {
                        false
                    }
                });

                let mut bounds = pred_ty.bounds.clone();
                if !is_unsized {
                    bounds.push(syn::TypeParamBound::Trait(syn::TraitBound {
                        paren_token: None,
                        modifier: syn::TraitBoundModifier::None,
                        lifetimes: None,
                        path: syn::Path::from(syn::Ident::new("Sized", Span::call_site())),
                    }));
                }

                key_bounds.push(syn::WherePredicate::Type(syn::PredicateType {
                    lifetimes: None,
                    bounded_ty: syn::Type::Path(syn::TypePath {
                        path: syn::Path {
                            leading_colon: None,
                            segments: new_path,
                        },
                        qself: None,
                    }),
                    colon_token: syn::token::Colon(Span::call_site()),
                    bounds,
                }));
            }
        }
    });

    clauses.extend(key_bounds);

    (quote!(<#impl_bounds>), quote!(where #clauses))
}

pub fn ty_id(ty: &syn::Type) -> Result<String, String> {
    match ty {
        syn::Type::Tuple(ty) => Ok(format!(
            "({})",
            ty.elems
                .iter()
                .map(|ty| ty_id(ty))
                .collect::<Result<Vec<_>, String>>()?
                .join(", ")
        )),
        syn::Type::Path(ty) => Ok(ty.to_token_stream().to_string()),
        _ => Err("Unrecognized / unsupported type for impl block".to_string()),
    }
}
