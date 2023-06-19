use super::Config;
use std::iter;

use crate::extension::*;
use proc_macro2::Span;
use quasi_quote::{quote_expr, quote_generic_arg, quote_path};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Lifetime, Token};

fn static_or_anon(life: &Lifetime) -> Lifetime {
    if life.ident == "static" {
        life.clone()
    } else {
        Lifetime::anon(life.span())
    }
}

fn build_name<F>(name: &syn::Ident, generics: &syn::Generics, f: F) -> syn::Path
where
    F: FnMut(&syn::GenericParam) -> syn::GenericArgument,
{
    let ty_generics = generics.params.iter().map(f);
    let seg = syn::PathSegment::from(name.clone()).with_all_bracketed(ty_generics);
    syn::Path::new([seg])
}

fn item_name(name: &syn::Ident, generics: &syn::Generics) -> syn::Path {
    build_name(name, generics, |param| match param {
        syn::GenericParam::Lifetime(..) => {
            syn::GenericArgument::Lifetime(syn::Lifetime::anon(param.span()))
        }
        syn::GenericParam::Type(syn::TypeParam { ident, .. })
        | syn::GenericParam::Const(syn::ConstParam { ident, .. }) => {
            quote_generic_arg!(#{ident.clone()})
        }
    })
}

fn item_lifetime_name(name: &syn::Ident, generics: &syn::Generics) -> syn::Path {
    build_name(name, generics, |param| match param {
        syn::GenericParam::Lifetime(lifetime) => quote_generic_arg!(#{lifetime.lifetime.clone()}),
        syn::GenericParam::Type(syn::TypeParam { ident, .. })
        | syn::GenericParam::Const(syn::ConstParam { ident, .. }) => {
            quote_generic_arg!(#{ident.clone()})
        }
    })
}

fn item_static_name(name: &syn::Ident, generics: &syn::Generics) -> syn::Path {
    build_name(name, generics, |param| match param {
        syn::GenericParam::Lifetime(..) => {
            syn::GenericArgument::Lifetime(syn::Lifetime::static_(param.span()))
        }
        syn::GenericParam::Type(syn::TypeParam { ident, .. }) => {
            quote_generic_arg!(#{ident.clone()}::Key)
        }
        syn::GenericParam::Const(syn::ConstParam { ident, .. }) => {
            quote_generic_arg!(#{ident.clone()})
        }
    })
}

fn item_qual_name(cfg: &Config, name: &syn::Ident, generics: &syn::Generics) -> syn::Expr {
    let ty_generics = generics
        .params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Lifetime(_) => None::<syn::Expr>,
            syn::GenericParam::Type(param) => Some(quote_expr!(#{param.ident.clone()}::name())),
            syn::GenericParam::Const(param) => Some(quote_expr!(#{param.ident.clone()})),
        })
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

    let path = quote_path!(::std::format);

    syn::Expr::Macro(syn::ExprMacro::new(syn::Macro::new(path).with_tokens(
        quote!(#fmt_str, #module_path, stringify!(#name), #(#ty_generics,)*),
    )))
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
}

pub struct Bounds {
    pub impl_bounds: Punctuated<syn::GenericParam, Token![,]>,
    pub where_bounds: Punctuated<syn::WherePredicate, Token![,]>,
}

pub trait OutputHelpers {
    /// Get the impl and where bounds to use for the `impl {Reflected, ReflectedX}` blocks
    fn reflect_bounds(&self, cfg: &Config) -> Bounds;
    /// Get the impl and where bounds to use for the `impl NotOutlives` block
    fn outlives_bounds(&self, cfg: &Config) -> Bounds;

    /// Get the fn name to use in the initialize implementation, generally an unsafe method
    /// on `Type`
    fn new_fn_name(&self) -> syn::Ident;
    /// Get a name for this type. The ident, or some formatting of the path
    fn name(&self, ty: NameTy) -> syn::Path;
    /// Get the rebound runtime name for this type.
    fn rebound_name(&self, cfg: &Config) -> syn::Expr;
}

impl OutputHelpers for syn::Item {
    fn reflect_bounds(&self, cfg: &Config) -> Bounds {
        match self {
            syn::Item::Struct(is) => is.reflect_bounds(cfg),
            syn::Item::Enum(ie) => ie.reflect_bounds(cfg),
            syn::Item::Union(iu) => iu.reflect_bounds(cfg),
            _ => unreachable!(),
        }
    }

    fn outlives_bounds(&self, cfg: &Config) -> Bounds {
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

    fn name(&self, ty: NameTy) -> syn::Path {
        match self {
            syn::Item::Struct(is) => is.name(ty),
            syn::Item::Enum(ie) => ie.name(ty),
            syn::Item::Union(iu) => iu.name(ty),
            _ => unreachable!(),
        }
    }

    fn rebound_name(&self, cfg: &Config) -> syn::Expr {
        match self {
            syn::Item::Struct(is) => is.rebound_name(cfg),
            syn::Item::Enum(ie) => ie.rebound_name(cfg),
            syn::Item::Union(iu) => iu.rebound_name(cfg),
            _ => unreachable!(),
        }
    }
}

impl OutputHelpers for syn::ItemStruct {
    fn reflect_bounds(&self, cfg: &Config) -> Bounds {
        reflect_bounds(cfg, &self.generics)
    }

    fn outlives_bounds(&self, cfg: &Config) -> Bounds {
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

    fn name(&self, ty: NameTy) -> syn::Path {
        match ty {
            NameTy::Ident => quote_path!(#{self.ident.clone()}),
            NameTy::Path => item_name(&self.ident, &self.generics),
            NameTy::LifetimePath => item_lifetime_name(&self.ident, &self.generics),
            NameTy::StaticPath => item_static_name(&self.ident, &self.generics),
        }
    }

    fn rebound_name(&self, cfg: &Config) -> syn::Expr {
        item_qual_name(cfg, &self.ident, &self.generics)
    }
}

impl OutputHelpers for syn::ItemEnum {
    fn reflect_bounds(&self, cfg: &Config) -> Bounds {
        reflect_bounds(cfg, &self.generics)
    }

    fn outlives_bounds(&self, cfg: &Config) -> Bounds {
        outlives_bounds(cfg, &self.generics)
    }

    fn new_fn_name(&self) -> syn::Ident {
        syn::Ident::new("new_enum", Span::call_site())
    }

    fn name(&self, ty: NameTy) -> syn::Path {
        match ty {
            NameTy::Ident => quote_path!(#{self.ident.clone()}),
            NameTy::Path => item_name(&self.ident, &self.generics),
            NameTy::LifetimePath => item_lifetime_name(&self.ident, &self.generics),
            NameTy::StaticPath => item_static_name(&self.ident, &self.generics),
        }
    }

    fn rebound_name(&self, cfg: &Config) -> syn::Expr {
        item_qual_name(cfg, &self.ident, &self.generics)
    }
}

impl OutputHelpers for syn::ItemUnion {
    fn reflect_bounds(&self, cfg: &Config) -> Bounds {
        reflect_bounds(cfg, &self.generics)
    }

    fn outlives_bounds(&self, cfg: &Config) -> Bounds {
        outlives_bounds(cfg, &self.generics)
    }

    fn new_fn_name(&self) -> syn::Ident {
        syn::Ident::new("new_union", Span::call_site())
    }

    fn name(&self, ty: NameTy) -> syn::Path {
        match ty {
            NameTy::Ident => quote_path!(#{self.ident.clone()}),
            NameTy::Path => item_name(&self.ident, &self.generics),
            NameTy::LifetimePath => item_lifetime_name(&self.ident, &self.generics),
            NameTy::StaticPath => item_static_name(&self.ident, &self.generics),
        }
    }

    fn rebound_name(&self, cfg: &Config) -> syn::Expr {
        item_qual_name(cfg, &self.ident, &self.generics)
    }
}

pub fn sanitized_field_ty(ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Array(ty) => {
            syn::TypeArray::new(sanitized_field_ty(&ty.elem), ty.len.clone()).into()
        }
        syn::Type::Reference(ty) => syn::TypeReference::new(sanitized_field_ty(&ty.elem))
            .with_lifetime(ty.lifetime.as_ref().map(static_or_anon))
            .with_mutability(ty.mutability.clone())
            .into(),
        syn::Type::Path(ty) => {
            let segments = ty.path.segments.iter().map(|seg| {
                let mut out = syn::PathSegment::from(seg.ident.clone());
                match &seg.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        let ty_args = args
                            .args
                            .iter()
                            .map(|arg| match arg {
                                syn::GenericArgument::Lifetime(life) => {
                                    syn::GenericArgument::Lifetime(static_or_anon(life))
                                }
                                syn::GenericArgument::Type(ty) => {
                                    syn::GenericArgument::Type(sanitized_field_ty(ty))
                                }
                                _ => arg.clone(),
                            })
                            .collect::<Vec<_>>();

                        out = out.with_all_bracketed(ty_args);
                    }
                    syn::PathArguments::Parenthesized(args) => {
                        let in_args = args.inputs.iter().map(sanitized_field_ty);
                        let out_arg = match &args.output {
                            syn::ReturnType::Default => syn::ReturnType::Default,
                            syn::ReturnType::Type(arrow, ty) => syn::ReturnType::Type(
                                arrow.clone(),
                                Box::new(sanitized_field_ty(ty)),
                            ),
                        };

                        out = out.with_all_paren_args(in_args).with_paren_ret(out_arg);
                    }
                    syn::PathArguments::None => (),
                }
                out
            });
            syn::TypePath::new(syn::Path::new(segments)).into()
        }
        ty => ty.clone(),
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
    syn::TypeParamBound::Trait(syn::TraitBound::new(quote_path!(#{&cfg.crate_name}::Reflected)))
}

fn reflect_bounds(cfg: &Config, generics: &syn::Generics) -> Bounds {
    let reflected_bound = reflect_bound(cfg);

    let mut impl_bounds: Punctuated<syn::GenericParam, Token![,]> = Punctuated::new();
    let mut where_bounds: Punctuated<syn::WherePredicate, Token![,]> = Punctuated::new();

    if let Some(clause) = &generics.where_clause {
        where_bounds.extend(clause.predicates.iter().cloned());
    }

    // For each param and bound in <>, if there's an existing clause, append the bounds to it.
    // Otherwise, add a new bound
    generics.params.iter().for_each(|param| match param {
        syn::GenericParam::Lifetime(param) => {
            let found = where_bounds.iter_mut().any(|clause| {
                if let syn::WherePredicate::Lifetime(life) = clause {
                    if life.lifetime == param.lifetime {
                        life.bounds.extend(param.bounds.iter().cloned());
                        return true;
                    }
                }
                false
            });

            if !found {
                where_bounds.push(syn::WherePredicate::Lifetime(
                    syn::PredicateLifetime::new(param.lifetime.clone())
                        .with_bounds(param.bounds.iter().cloned()),
                ));
            }

            impl_bounds.push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
                param.lifetime.clone(),
            )));
        }
        syn::GenericParam::Type(param) => {
            let found = where_bounds.iter_mut().any(|clause| {
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
                let path = syn::Path::from(param.ident.clone());
                let predicate = syn::PredicateType::new(syn::TypePath::new(path)).with_bounds(
                    param
                        .bounds
                        .iter()
                        .cloned()
                        .filter(|bound| !is_maybe(bound)),
                );

                where_bounds.push(syn::WherePredicate::Type(predicate));
            }

            let bounds = param.bounds.iter().cloned().filter(is_maybe);

            impl_bounds.push(syn::GenericParam::Type(
                syn::TypeParam::new(param.ident.clone()).with_bounds(bounds),
            ))
        }
        syn::GenericParam::Const(_) => impl_bounds.push(param.clone()),
    });

    let mut key_bounds: Vec<syn::WherePredicate> = Vec::new();

    where_bounds.iter_mut().for_each(|clause| {
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
                    bounds.push(syn::TypeParamBound::Trait(syn::TraitBound::new(
                        syn::Path::new([syn::Ident::new("Sized", Span::call_site())]),
                    )));
                }

                key_bounds.push(syn::WherePredicate::Type(
                    syn::PredicateType::new(syn::TypePath::new(syn::Path::new(new_path)))
                        .with_bounds(bounds),
                ));
            }
        }
    });

    where_bounds.extend(key_bounds);

    Bounds {
        impl_bounds,
        where_bounds,
    }
}

fn not_outlives_bound(cfg: &Config, lifetime: Lifetime) -> syn::TypeParamBound {
    syn::TypeParamBound::Trait(syn::TraitBound::new(syn::Path::new([
        syn::PathSegment::from(cfg.crate_name.ident()),
        syn::PathSegment::from(syn::Ident::new("value", Span::call_site())),
        syn::PathSegment::from(syn::Ident::new("NotOutlives", Span::call_site()))
            .with_bracketed(syn::GenericArgument::Lifetime(lifetime)),
    ])))
}

fn outlives_bounds(cfg: &Config, generics: &syn::Generics) -> Bounds {
    let mut impl_bounds: Vec<syn::GenericParam> = Vec::new();
    let mut where_bounds: Punctuated<syn::WherePredicate, Token![,]> = Punctuated::new();
    let mut lifetimes: Vec<syn::Lifetime> = Vec::new();

    if let Some(clause) = &generics.where_clause {
        where_bounds.extend(clause.predicates.iter().cloned());
    }

    let mut count = 0;
    let base_lifetime = Lifetime::new("'no", Span::call_site());
    impl_bounds.push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
        base_lifetime.clone(),
    )));

    generics.params.iter().for_each(|param| match param {
        syn::GenericParam::Lifetime(param) => {
            where_bounds.iter_mut().for_each(|clause| {
                if let syn::WherePredicate::Lifetime(life) = clause {
                    if life.lifetime == base_lifetime {
                        life.bounds.push(param.lifetime.clone())
                    }
                }
            });

            // If there's an existing where clause for this lifetime, use it.
            // Otherwise, add one
            let found = where_bounds.iter_mut().any(|clause| {
                if let syn::WherePredicate::Lifetime(life) = clause {
                    if life.lifetime == param.lifetime {
                        life.bounds.extend(param.bounds.iter().cloned());
                        return true;
                    }
                }
                false
            });

            if !found {
                where_bounds.push(syn::WherePredicate::Lifetime(
                    syn::PredicateLifetime::new(param.lifetime.clone())
                        .with_bounds(param.bounds.iter().cloned()),
                ));
            }

            lifetimes.push(param.lifetime.clone());

            impl_bounds.push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
                param.lifetime.clone(),
            )));
        }
        syn::GenericParam::Type(param) => {
            let new_lifetime = syn::Lifetime::new(&format!("'a{}", count), Span::call_site());
            impl_bounds.push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
                new_lifetime.clone(),
            )));
            count += 1;

            let found = where_bounds.iter_mut().any(|clause| {
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
                let bounds = param
                    .bounds
                    .iter()
                    .cloned()
                    .filter(|bound| !is_maybe(bound))
                    .chain(iter::once(not_outlives_bound(cfg, new_lifetime)));

                where_bounds.push(syn::WherePredicate::Type(
                    syn::PredicateType::new(syn::TypePath::new(syn::Path::from(
                        param.ident.clone(),
                    )))
                    .with_bounds(bounds),
                ))
            }

            let bounds = param.bounds.iter().cloned().filter(is_maybe);

            impl_bounds.push(syn::GenericParam::Type(
                syn::TypeParam::new(param.ident.clone()).with_bounds(bounds),
            ))
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
    where_bounds.push(syn::WherePredicate::Lifetime(
        syn::PredicateLifetime::new(base_lifetime).with_bounds(lifetimes.into_iter()),
    ));

    Bounds {
        impl_bounds,
        where_bounds,
    }
}

pub fn ty_id(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Array(ty) => format!(
            "[{}; {}]",
            ty_id(&ty.elem),
            ty.len.to_token_stream().to_string()
        ),
        syn::Type::Group(ty) => ty_id(&ty.elem),
        syn::Type::Paren(ty) => ty_id(&ty.elem),
        syn::Type::Path(ty) => ty.to_token_stream().to_string(),
        syn::Type::Tuple(ty) => format!(
            "({})",
            ty.elems
                .iter()
                .map(|ty| ty_id(ty))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => unimplemented!(),
    }
}
