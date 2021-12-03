use super::Config;

use std::iter::FromIterator;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Lifetime, Token};

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

fn static_or_anon(life: &Lifetime) -> Lifetime {
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

fn item_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    build_name(name, generics, |param| match param {
        syn::GenericParam::Lifetime(..) => quote!('_),
        syn::GenericParam::Type(syn::TypeParam { ident, .. })
        | syn::GenericParam::Const(syn::ConstParam { ident, .. }) => quote!(#ident),
    })
}

fn item_lifetime_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    build_name(name, generics, |param| match param {
        syn::GenericParam::Lifetime(lifetime) => quote!(#lifetime),
        syn::GenericParam::Type(syn::TypeParam { ident, .. })
        | syn::GenericParam::Const(syn::ConstParam { ident, .. }) => quote!(#ident),
    })
}

fn item_static_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
    build_name(name, generics, |param| match param {
        syn::GenericParam::Lifetime(..) => quote!('static),
        syn::GenericParam::Type(syn::TypeParam { ident, .. }) => quote!(#ident::Key),
        syn::GenericParam::Const(syn::ConstParam { ident, .. }) => quote!(#ident),
    })
}

fn item_qual_name(cfg: &Config, name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
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

    quote!(::std::format!(#fmt_str, #module_path, stringify!(#name), #(#ty_generics,)* ))
}

#[derive(Copy, Clone)]
pub enum NameTy {
    /// This item's identifier
    Ident,
    /// This item's code path
    Path,
    /// This item's path with generic lifetimes
    LifetimePath,
    /// This item's full path with static lifetimes
    StaticPath,
    /// This item's full rebound name path
    ReboundName,
}

pub trait OutputHelpers {
    /// Get the impl and where bounds to use for the `impl {Reflected, ReflectedX}` blocks
    fn reflect_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream);
    /// Get the impl and where bounds to use for the `impl NotOutlives` block
    fn outlives_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream);

    /// Get the fn name to use in the initialize implementation, generally an unsafe method
    /// on `Type`
    fn new_fn_name(&self) -> syn::Ident;
    /// Get a name for this type. The ident, or some formatting of the path
    fn name(&self, cfg: &Config, ty: NameTy) -> TokenStream;
}

impl OutputHelpers for syn::Item {
    fn reflect_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream) {
        match self {
            syn::Item::Struct(is) => is.reflect_bounds(cfg),
            syn::Item::Enum(ie) => ie.reflect_bounds(cfg),
            syn::Item::Union(iu) => iu.reflect_bounds(cfg),
            _ => unreachable!(),
        }
    }

    fn outlives_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream) {
        match self {
            syn::Item::Struct(is) => is.outlives_bounds(cfg),
            syn::Item::Enum(ie) => ie.outlives_bounds(cfg),
            syn::Item::Union(iu) => iu.outlives_bounds(cfg),
            _ => unreachable!(),
        }
    }

    fn new_fn_name(&self) -> syn::Ident {
        match self {
            syn::Item::Struct(is) => is.new_fn_name(),
            syn::Item::Enum(ie) => ie.new_fn_name(),
            syn::Item::Union(iu) => iu.new_fn_name(),
            _ => unreachable!(),
        }
    }

    fn name(&self, cfg: &Config, ty: NameTy) -> TokenStream {
        match self {
            syn::Item::Struct(is) => is.name(cfg, ty),
            syn::Item::Enum(ie) => ie.name(cfg, ty),
            syn::Item::Union(iu) => iu.name(cfg, ty),
            _ => unreachable!(),
        }
    }
}

impl OutputHelpers for syn::ItemStruct {
    fn reflect_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream) {
        reflect_bounds(cfg, &self.generics)
    }

    fn outlives_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream) {
        outlives_bounds(cfg, &self.generics)
    }

    fn new_fn_name(&self) -> syn::Ident {
        let name = match self.fields {
            syn::Fields::Named(_) => "new_struct",
            syn::Fields::Unnamed(_) => "new_tuple_struct",
            syn::Fields::Unit => "new_unit_struct",
        };
        syn::Ident::new(name, Span::call_site())
    }

    fn name(&self, cfg: &Config, ty: NameTy) -> TokenStream {
        match ty {
            NameTy::Ident => {
                let ident = &self.ident;
                quote!(#ident)
            }
            NameTy::Path => item_name(&self.ident, &self.generics),
            NameTy::LifetimePath => item_lifetime_name(&self.ident, &self.generics),
            NameTy::StaticPath => item_static_name(&self.ident, &self.generics),
            NameTy::ReboundName => item_qual_name(cfg, &self.ident, &self.generics),
        }
    }
}

impl OutputHelpers for syn::ItemEnum {
    fn reflect_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream) {
        reflect_bounds(cfg, &self.generics)
    }

    fn outlives_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream) {
        outlives_bounds(cfg, &self.generics)
    }

    fn new_fn_name(&self) -> syn::Ident {
        syn::Ident::new("new_enum", Span::call_site())
    }

    fn name(&self, cfg: &Config, ty: NameTy) -> TokenStream {
        match ty {
            NameTy::Ident => {
                let ident = &self.ident;
                quote!(#ident)
            }
            NameTy::Path => item_name(&self.ident, &self.generics),
            NameTy::LifetimePath => item_lifetime_name(&self.ident, &self.generics),
            NameTy::StaticPath => item_static_name(&self.ident, &self.generics),
            NameTy::ReboundName => item_qual_name(cfg, &self.ident, &self.generics),
        }
    }
}

impl OutputHelpers for syn::ItemUnion {
    fn reflect_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream) {
        reflect_bounds(cfg, &self.generics)
    }

    fn outlives_bounds(&self, cfg: &Config) -> (TokenStream, TokenStream) {
        outlives_bounds(cfg, &self.generics)
    }

    fn new_fn_name(&self) -> syn::Ident {
        syn::Ident::new("new_union", Span::call_site())
    }

    fn name(&self, cfg: &Config, ty: NameTy) -> TokenStream {
        match ty {
            NameTy::Ident => {
                let ident = &self.ident;
                quote!(#ident)
            }
            NameTy::Path => item_name(&self.ident, &self.generics),
            NameTy::LifetimePath => item_lifetime_name(&self.ident, &self.generics),
            NameTy::StaticPath => item_static_name(&self.ident, &self.generics),
            NameTy::ReboundName => item_qual_name(cfg, &self.ident, &self.generics),
        }
    }
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

fn reflect_bound(cfg: &Config) -> syn::TypeParamBound {
    syn::TypeParamBound::Trait(syn::TraitBound {
        paren_token: None,
        modifier: syn::TraitBoundModifier::None,
        lifetimes: None,
        path: syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(
                vec![
                    cfg.crate_name.clone(),
                    syn::Ident::new("Reflected", Span::call_site()),
                ]
                .into_iter()
                .map(syn::PathSegment::from),
            ),
        },
    })
}

fn reflect_bounds(cfg: &Config, generics: &syn::Generics) -> (TokenStream, TokenStream) {
    let reflected_bound = reflect_bound(cfg);

    let mut impl_bounds: Punctuated<syn::GenericParam, Token![,]> = Punctuated::new();
    let mut clauses: Punctuated<syn::WherePredicate, Token![,]> = Punctuated::new();

    if let Some(clause) = &generics.where_clause {
        clauses.extend(clause.predicates.iter().cloned());
    }

    // For each param and bound in <>, if there's an existing clause, append the bounds to it.
    // Otherwise, add a new bound
    generics.params.iter().for_each(|param| match param {
        syn::GenericParam::Lifetime(param) => {
            let found = clauses.iter_mut().any(|clause| {
                if let syn::WherePredicate::Lifetime(life) = clause {
                    if life.lifetime == param.lifetime {
                        life.bounds.extend(param.bounds.iter().cloned());
                        return true;
                    }
                }
                false
            });

            if !found {
                clauses.push(syn::WherePredicate::Lifetime(syn::PredicateLifetime {
                    lifetime: param.lifetime.clone(),
                    colon_token: syn::token::Colon::default(),
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
                            return true;
                        }
                    }
                }
                false
            });

            if !found {
                clauses.push(syn::WherePredicate::Type(syn::PredicateType {
                    lifetimes: None,
                    bounded_ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path::from(param.ident.clone()),
                    }),
                    colon_token: syn::token::Colon::default(),
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
                Some(syn::token::Colon::default())
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
                    colon_token: syn::token::Colon::default(),
                    bounds,
                }));
            }
        }
    });

    clauses.extend(key_bounds);

    (quote!(<#impl_bounds>), quote!(where #clauses))
}

fn not_outlives_bound(cfg: &Config, lifetime: Lifetime) -> syn::TypeParamBound {
    syn::TypeParamBound::Trait(syn::TraitBound {
        paren_token: None,
        modifier: syn::TraitBoundModifier::None,
        lifetimes: None,
        path: syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(vec![
                cfg.crate_name.clone().into(),
                syn::Ident::new("value", Span::call_site()).into(),
                syn::PathSegment {
                    ident: syn::Ident::new("NotOutlives", Span::call_site()),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: Some(syn::token::Colon2::default()),
                            lt_token: syn::token::Lt::default(),
                            args: Punctuated::from_iter(
                                vec![syn::GenericArgument::Lifetime(lifetime)].into_iter(),
                            ),
                            gt_token: syn::token::Gt::default(),
                        },
                    ),
                },
            ]),
        },
    })
}

fn outlives_bounds(cfg: &Config, generics: &syn::Generics) -> (TokenStream, TokenStream) {
    let mut impl_bounds: Vec<syn::GenericParam> = Vec::new();
    let mut clauses: Punctuated<syn::WherePredicate, Token![,]> = Punctuated::new();
    let mut lifetimes: Vec<syn::Lifetime> = Vec::new();

    if let Some(clause) = &generics.where_clause {
        clauses.extend(clause.predicates.iter().cloned());
    }

    let mut count = 0;
    let base_lifetime = Lifetime::new("'no", Span::call_site());
    impl_bounds.push(syn::GenericParam::Lifetime(syn::LifetimeDef {
        attrs: vec![],
        lifetime: base_lifetime.clone(),
        colon_token: None,
        bounds: Punctuated::new(),
    }));

    generics.params.iter().for_each(|param| match param {
        syn::GenericParam::Lifetime(param) => {
            clauses.iter_mut().for_each(|clause| {
                if let syn::WherePredicate::Lifetime(life) = clause {
                    if life.lifetime == base_lifetime {
                        life.bounds.push(param.lifetime.clone())
                    }
                }
            });

            // If there's an existing where clause for this lifetime, use it.
            // Otherwise, add one
            let found = clauses.iter_mut().any(|clause| {
                if let syn::WherePredicate::Lifetime(life) = clause {
                    if life.lifetime == param.lifetime {
                        life.bounds.extend(param.bounds.iter().cloned());
                        return true;
                    }
                }
                false
            });

            if !found {
                clauses.push(syn::WherePredicate::Lifetime(syn::PredicateLifetime {
                    lifetime: param.lifetime.clone(),
                    colon_token: syn::token::Colon::default(),
                    bounds: param.bounds.clone(),
                }));
            }

            lifetimes.push(param.lifetime.clone());

            impl_bounds.push(syn::GenericParam::Lifetime(syn::LifetimeDef {
                attrs: vec![],
                lifetime: param.lifetime.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
            }));
        }
        syn::GenericParam::Type(param) => {
            let new_lifetime = syn::Lifetime::new(&format!("'a{}", count), Span::call_site());
            impl_bounds.push(syn::GenericParam::Lifetime(syn::LifetimeDef {
                attrs: vec![],
                lifetime: new_lifetime.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
            }));
            count += 1;

            let found = clauses.iter_mut().any(|clause| {
                if let syn::WherePredicate::Type(pred_ty) = clause {
                    if let syn::Type::Path(path) = &pred_ty.bounded_ty {
                        if path
                            .path
                            .get_ident()
                            .map_or(false, |ident| param.ident == *ident)
                        {
                            pred_ty.bounds.extend(param.bounds.iter().cloned());
                            pred_ty
                                .bounds
                                .push(not_outlives_bound(cfg, new_lifetime.clone()));
                            return true;
                        }
                    }
                }
                false
            });

            if !found {
                let mut bounds: Punctuated<_, _> = param
                    .bounds
                    .iter()
                    .cloned()
                    .filter(|bound| !is_maybe(bound))
                    .collect();

                bounds.push(not_outlives_bound(cfg, new_lifetime));

                clauses.push(syn::WherePredicate::Type(syn::PredicateType {
                    lifetimes: None,
                    bounded_ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path::from(param.ident.clone()),
                    }),
                    colon_token: syn::token::Colon::default(),
                    bounds,
                }))
            }

            let bounds: Punctuated<syn::TypeParamBound, Token![+]> =
                param.bounds.iter().cloned().filter(is_maybe).collect();
            let colon_token = if bounds.is_empty() {
                None
            } else {
                Some(syn::token::Colon::default())
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

    impl_bounds.sort_unstable_by(|left, right| {
        let left = match left {
            syn::GenericParam::Lifetime(_) => -1,
            syn::GenericParam::Type(_) => 0,
            syn::GenericParam::Const(_) => 1,
        };
        let right = match right {
            syn::GenericParam::Lifetime(_) => -1,
            syn::GenericParam::Type(_) => 0,
            syn::GenericParam::Const(_) => 1,
        };
        return i32::cmp(&left, &right);
    });
    let impl_bounds: Punctuated<syn::GenericParam, Token![,]> = impl_bounds.into_iter().collect();

    lifetimes
        .extend((0..count).map(|idx| syn::Lifetime::new(&format!("'a{}", idx), Span::call_site())));
    clauses.push(syn::WherePredicate::Lifetime(syn::PredicateLifetime {
        lifetime: base_lifetime,
        colon_token: syn::token::Colon::default(),
        bounds: lifetimes.into_iter().collect(),
    }));

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
