use super::utils::*;
use super::Config;
use crate::error::{Error, Result};
use crate::extension::{StructExtension, StructType, VariantExtension};

use std::collections::HashMap;
use std::sync::RwLock;

use proc_macro2::{Span, TokenStream};
use once_cell::sync::OnceCell;
use quote::{quote, quote_spanned};
use syn::Item;
use syn::spanned::Spanned;

pub fn generate_assoc_fn(
    cfg: &Config,
    self_ty: &syn::Type,
    sig: &syn::Signature,
) -> Result<TokenStream> {
    if !sig.generics.params.is_empty() {
        return Err(
            Error::NotSupported("generic functions".to_string())
        );
    }

    let crate_name = &cfg.crate_name;
    let fn_name = &sig.ident;
    let inputs = &sig.inputs;
    let output = &sig.output;

    let args = inputs
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(..) => None,
            syn::FnArg::Typed(pat_ty) => Some(&*pat_ty.ty),
        })
        .collect::<Vec<_>>();

    let ret_ty = match output {
        syn::ReturnType::Default => quote!( () ),
        syn::ReturnType::Type(_, ty) => quote!(#ty),
    };

    if let Some(syn::FnArg::Receiver(arg)) = inputs.first() {
        let receiver = if arg.reference.is_some() {
            let mutability = &arg.mutability;
            syn::Type::Reference(syn::TypeReference {
                and_token: syn::Token![&](Span::call_site()),
                lifetime: None,
                mutability: mutability.clone(),
                elem: Box::new(self_ty.clone()),
            })
        } else {
            self_ty.clone()
        };

        Ok(quote_spanned!(sig.span() => {
            #[allow(unused_mut)]
            let call = |this: #crate_name::Value<'_>, mut args: ::std::vec::Vec<#crate_name::Value<'_>>| {
                use ::core::convert::From;
                // SAFETY: TODO
                let v = #crate_name::Value::from(unsafe {
                    <#self_ty>::#fn_name(
                        this.cast_unsafe::<#receiver>(),
                        #( args.remove(0).cast_unsafe::<#args>(), )*
                    )
                });
                ::core::debug_assert_eq!(args.len(), 0);
                // SAFETY: Value cannot be safely constructed with a `'a` that outlives the T.
                //         As such, we know that the lifetimes here should never be violated.
                unsafe { ::core::mem::transmute::<#crate_name::Value<'_>, #crate_name::Value<'_>>(v) }
            };
            let name = stringify!(#fn_name);
            let assoc_ty = #crate_name::Type::from::<#self_ty>();
            let self_ty = #crate_name::Type::from::<#receiver>();
            let args = &[#( #crate_name::Type::from::<#args>(), )*];
            let ret = #crate_name::Type::from::<#ret_ty>();

            // SAFETY: Generated implementation is assured correct
            unsafe { #crate_name::AssocFn::new_dynamic(call, name, assoc_ty, self_ty, args, ret) }
        }))
    } else {
        Ok(quote_spanned!(sig.span() => {
            #[allow(unused_mut, unused_unsafe)]
            let call = |mut args: ::std::vec::Vec<#crate_name::Value<'_>>| {
                use ::core::convert::From;
                // SAFETY: TODO
                let v = #crate_name::Value::from(unsafe {
                    <#self_ty>::#fn_name(
                        #( args.remove(0).cast_unsafe::<#args>(), )*
                    )
                });
                ::core::debug_assert_eq!(args.len(), 0);
                // SAFETY: Value cannot be safely constructed with a `'a` that outlives the T.
                //         As such, we know that the lifetimes here should never be violated.
                unsafe { ::core::mem::transmute::<#crate_name::Value<'_>, #crate_name::Value<'_>>(v) }
            };
            let name = stringify!(#fn_name);
            let assoc_ty = #crate_name::Type::from::<#self_ty>();
            let args = &[#( #crate_name::Type::from::<#args>(), )*];
            let ret = #crate_name::Type::from::<#ret_ty>();

            // SAFETY: Generated implementation is assured correct
            unsafe { #crate_name::AssocFn::new_static(call, name, assoc_ty, args, ret) }
        }))
    }
}

pub fn generate_assoc_const(
    cfg: &Config,
    self_ty: &syn::Type,
    const_name: &syn::Ident,
    const_ty: &syn::Type,
) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let span = const_name.span().join(const_ty.span())
        .unwrap_or_else(|| const_name.span());

    Ok(quote_spanned!(span => {
        let ptr: ::std::boxed::Box<fn() -> _> = ::std::boxed::Box::new(|| {
            use ::core::convert::From;
            #crate_name::Value::from(<#self_ty>::#const_name)
        });
        let name = stringify!(#const_name);
        let assoc_ty = #crate_name::Type::from::<#self_ty>();
        let ty = #crate_name::Type::from::<#const_ty>();

        // SAFETY: Generated implementation is assured correct
        unsafe { #crate_name::AssocConst::new(ptr, name, assoc_ty, ty) }
    }))
}

pub fn generate_struct_field(
    cfg: &Config,
    item: &syn::ItemStruct,
    idx: usize,
    field: &syn::Field,
) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let no_get = cfg.no_get;
    let no_set = cfg.no_set;

    let name = item.name(cfg, NameTy::Path);

    let field_ty = sanitized_field_ty(&field.ty);

    let (field_name, fn_name, name_arg) = match &field.ident {
        Some(field_name) => (
            quote!(#field_name),
            syn::Ident::new("new_named", Span::call_site()),
            quote!(stringify!(#field_name)),
        ),
        None => {
            let access = syn::Index::from(idx);
            (
                quote!(#access),
                syn::Ident::new("new_tuple", Span::call_site()),
                quote!(#idx)
            )
        }
    };

    let accessor = if !no_get {
        quote_spanned!(field.span() =>
            ::core::option::Option::Some(|this| {
                // SAFETY: TODO
                let inner = unsafe { this.borrow_unsafe::<#name>() };
                let v = #crate_name::Value::from_ref(&inner.#field_name);
                // SAFETY: See rebound::ty::Ref
                unsafe { ::core::mem::transmute(v) }
            }
        ))
    } else {
        quote!(None)
    };

    let setter = if !no_set {
        quote_spanned!(field.span() =>
            ::core::option::Option::Some(|this, value| {
                // SAFETY: TODO
                let inner = unsafe { this.borrow_unsafe_mut::<#name>() };
                // SAFETY: TODO
                inner.#field_name = unsafe { value.cast_unsafe() };
            }
        ))
    } else {
        quote!(None)
    };

    Ok(quote_spanned!(field.span() => {
        let accessor: ::core::option::Option<for<'__a, '__b> fn(&'__a #crate_name::Value<'__b>) -> #crate_name::Value<'__a>> = #accessor;
        let setter: ::core::option::Option<for<'__r, '__s> fn(&'__r mut #crate_name::Value<'__s>, #crate_name::Value<'_>)> = #setter;
        let name = #name_arg;
        let assoc_ty = #crate_name::Type::from::<#name>();
        let field_ty = #crate_name::Type::from::<#field_ty>();

        // SAFETY: Generated implementation is assured correct
        unsafe { #crate_name::Field::#fn_name(accessor, setter, name, assoc_ty, field_ty) }
    }))
}

pub fn generate_enum_field(
    cfg: &Config,
    item: &syn::ItemEnum,
    var_name: &syn::Ident,
    idx: usize,
    field: &syn::Field,
) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let no_get = cfg.no_get;
    let no_set = cfg.no_set;

    let name = item.name(cfg, NameTy::Path);
    let simple_name = item.name(cfg, NameTy::Ident);

    let field_ty = sanitized_field_ty(&field.ty);

    let (field_access, fn_name, name_arg) = match &field.ident {
        Some(field_name) => (
            quote!({ #field_name: field, .. }),
            quote!(new_enum_named),
            quote!(stringify!(#field_name)),
        ),
        None => {
            let field_name = syn::Index::from(idx);
            (
                quote!({ #field_name: field, .. }),
                quote!(new_enum_tuple),
                quote!(#idx),
            )
        }
    };

    let accessor = if !no_get {
        quote_spanned!(field.span() =>
            ::core::option::Option::Some(|this| {
                // SAFETY: TODO
                let inner = unsafe { this.borrow_unsafe::<#name>() };
                if let #simple_name::#var_name #field_access = inner {
                    let v = #crate_name::Value::from_ref(field);
                    // SAFETY: Value cannot be safely constructed with a `'a` that outlives the T.
                    //         As such, we know that the lifetimes here should never be violated.
                    unsafe { ::core::mem::transmute::<#crate_name::Value, #crate_name::Value>(v) }
                } else {
                    #[cfg(debug_assertions)]
                    ::core::unreachable!("Enum variant was incorrect despite Field::get_ref check");
                    // SAFETY: Our current instance should match the requested variant type. This
                    //         is checked by `Field`
                    #[cfg(not(debug_assertions))]
                    unsafe { ::core::hint::unreachable_unchecked(); }
                }
            }
        ))
    } else {
        quote!(None)
    };

    let setter = if !no_set {
        quote_spanned!(field.span() =>
            ::core::option::Option::Some(|this, value| {
                // SAFETY: TODO
                let inner = unsafe { this.borrow_unsafe_mut::<#name>() };
                if let #simple_name::#var_name #field_access = inner {
                    // SAFETY: TODO
                    *field = unsafe { value.cast_unsafe::<#field_ty>() };
                } else {
                    #[cfg(debug_assertions)]
                    ::core::unreachable!("Enum variant was incorrect despite Field::set check");
                    // SAFETY: Our current instance should match the requested variant type. This
                    //         is checked by `Field`
                    #[cfg(not(debug_assertions))]
                    unsafe { ::core::hint::unreachable_unchecked(); }
                }
            }
        ))
    } else {
        quote!(None)
    };

    Ok(quote_spanned!(field.span() => {
        let accessor: ::core::option::Option<for<'__a, '__b> fn(&'__a #crate_name::Value<'__b>) -> #crate_name::Value<'__a>> = #accessor;
        let setter: ::core::option::Option<for<'__r, '__s> fn(&'__r mut #crate_name::Value<'__s>, #crate_name::Value<'_>)> = #setter;
        let name = #name_arg;
        let assoc_ty = #crate_name::Type::from::<#name>();
        let variant = if let #crate_name::Type::Enum(info) = #crate_name::Type::from::<#name>() {
            use ::core::iter::{Iterator, IntoIterator};
            info.variants()
                .into_iter()
                .find(|var| var.name() == stringify!(#var_name))
                .expect(::core::concat!("Enum ", stringify!(#name), " has a variant with the name ", stringify!(#var_name)))
        } else {
            #[cfg(debug_assertions)]
            ::core::unreachable!("Type::from for an enum didn't return a Type::Enum");
            // SAFETY: Type::from for an enum should by definition return a Type::Enum
            #[cfg(not(debug_assertions))]
            unsafe { ::core::hint::unreachable_unchecked(); }
        };
        let field_ty = #crate_name::Type::from::<#field_ty>();

        // SAFETY: Generated implementation is assured correct
        unsafe { #crate_name::Field::#fn_name(accessor, setter, name, assoc_ty, variant, field_ty) }
    }))
}

pub fn generate_union_field(
    cfg: &Config,
    name: &TokenStream,
    _: usize,
    field: &syn::Field,
) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let no_get = cfg.no_get;
    let no_set = cfg.no_set;

    let field_ty = sanitized_field_ty(&field.ty);
    let field_name = field
        .ident
        .as_ref()
        .expect("Unions always have named fields");

    let accessor = if !no_get {
        quote_spanned!(field.span() =>
            ::core::option::Option::Some(|this| {
                // SAFETY: TODO
                let inner = unsafe { this.borrow_unsafe::<#name>() };
                // SAFETY: TODO
                let v = #crate_name::Value::from_ref(unsafe { &inner.#field_name });
                // SAFETY: Value cannot be safely constructed with a `'a` that outlives the T.
                //         As such, we know that the lifetimes here should never be violated.
                unsafe { ::core::mem::transmute::<#crate_name::Value<'_>, #crate_name::Value<'_>>(v) }
            }
        ))
    } else {
        quote!(None)
    };

    let setter = if !no_set {
        quote_spanned!(field.span() =>
            ::core::option::Option::Some(|this, value| {
                // SAFETY: TODO
                let inner = unsafe { this.borrow_unsafe_mut::<#name>() };
                // SAFETY: TODO
                unsafe { inner.#field_name = value.cast_unsafe::<#field_ty>() };
            }
        ))
    } else {
        quote!(None)
    };

    Ok(quote_spanned!(field.span() => {
        let accessor: ::core::option::Option<for<'__a, '__b> fn(&'__a #crate_name::Value<'__b>) -> #crate_name::Value<'__a>> = #accessor;
        let setter: ::core::option::Option<for<'__r, '__s> fn(&'__r mut #crate_name::Value<'__s>, #crate_name::Value<'_>)> = #setter;
        let name = stringify!(#field_name);
        let assoc_ty = #crate_name::Type::from::<#name>();
        let field_ty = #crate_name::Type::from::<#field_ty>();

        // SAFETY: Generated implementation is assured correct
        unsafe { #crate_name::UnionField::new(accessor, setter, name, assoc_ty, field_ty) }
    }))
}

pub fn generate_variant(
    cfg: &Config,
    item: &syn::ItemEnum,
    variant: &syn::Variant,
) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;

    let name = item.name(cfg, NameTy::Path);
    let simple_name = item.name(cfg, NameTy::Ident);
    let var_name = &variant.ident;

    let (variant_ty, info_ty) = match variant.ty() {
        StructType::Named => ("Struct", "StructVariant"),
        StructType::Tuple => ("Tuple", "TupleVariant"),
        StructType::Unit => ("Unit", "UnitVariant"),
    };
    let variant_ty = syn::Ident::new(variant_ty, Span::call_site());
    let info_ty = syn::Ident::new(info_ty, Span::call_site());

    let fields = match variant.ty() {
        StructType::Unit => None,
        _ => {
            let fields = variant.fields()
                .into_iter()
                .enumerate()
                .map(|(idx, field)| generate_enum_field(cfg, item, var_name, idx, field))
                .collect::<Result<Vec<_>>>()?;

            Some(quote!(
                let fields = || { ::std::vec![ #( #fields ),* ] };
            ))
        }
    };

    let fields_ident = fields
        .as_ref()
        .map(|_| syn::Ident::new("fields", Span::call_site()))
        .into_iter();

    Ok(quote_spanned!(variant.span() => {
        let var_name = stringify!(#var_name);
        let assoc_ty = #crate_name::Type::from::<#name>();
        #fields
        let is_var: for<'__r, '__s> fn(&'__r #crate_name::Value<'__s>) -> _ = |val| {
            // SAFETY: TODO
            ::core::matches!(unsafe { val.borrow_unsafe::<#name>() }, #simple_name::#var_name { .. })
        };

        // SAFETY: Generated implementation is assured correct
        unsafe {
            #crate_name::Variant::#variant_ty(#crate_name::info::#info_ty::new(
                var_name,
                assoc_ty,
                #( #fields_ident, )*
                is_var,
            ))
        }
    }))
}

pub fn generate_reflect_enum(cfg: &Config, item: syn::ItemEnum) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let (impl_bounds, where_bounds) = item.reflect_bounds(cfg);
    let name = item.name(cfg, NameTy::Path);

    let mut variant_impls = Vec::new();

    for i in &item.variants {
        variant_impls.push(generate_variant(cfg, &item, i)?)
    }

    Ok(quote_spanned!(item.span() =>
        impl #impl_bounds #crate_name::reflect::ReflectedEnum for #name #where_bounds {
            fn variants() -> ::std::vec::Vec<#crate_name::Variant> {
                ::std::vec![ #(#variant_impls),* ]
            }
        }
    ))
}

static IMPLS_PER_TY: OnceCell<RwLock<HashMap<String, u8>>> = OnceCell::new();

pub fn generate_reflect_impl(cfg: &Config, item: syn::ItemImpl) -> Result<TokenStream> {
    if item.trait_.is_some() {
        return Err(Error::NotSupported("trait reflection".to_string()));
    }
    if !item.generics.params.is_empty() {
        return Err(Error::NotSupported("generic impls".to_string()));
    }

    let mut impls = IMPLS_PER_TY
        .get_or_init(|| RwLock::new(HashMap::new()))
        .write()
        .expect("IMPLS_PER_TY was poisoned");

    let crate_name = &cfg.crate_name;
    let self_ty = &item.self_ty;
    let id = ty_id(self_ty);

    let num = impls.entry(id).or_insert(0);

    let mut impl_fns = Vec::new();
    let mut impl_consts = Vec::new();
    for i in item.items {
        match i {
            syn::ImplItem::Method(impl_item) => {
                impl_fns.push(generate_assoc_fn(cfg, self_ty, &impl_item.sig)?);
            }
            syn::ImplItem::Const(impl_item) => {
                impl_consts.push(generate_assoc_const(
                    cfg,
                    self_ty,
                    &impl_item.ident,
                    &impl_item.ty,
                )?);
            }
            _ => {
                use proc_macro::{Diagnostic, Level};
                Diagnostic::new(
                    Level::Warning,
                    "Rebound currently only supports reflecting fns and consts in impls",
                )
                .emit();
            }
        }
    }

    let out = quote_spanned!(item.self_ty.span() =>
        impl #crate_name::reflect::ReflectedImpl<#num> for #self_ty {
            fn assoc_fns() -> ::std::vec::Vec<#crate_name::AssocFn> {
                ::std::vec![ #(#impl_fns,)* ]
            }

            fn assoc_consts() -> ::std::vec::Vec<#crate_name::AssocConst> {
                ::std::vec![ #(#impl_consts,)* ]
            }
        }
    );

    *num += 1;

    Ok(out)
}

pub fn generate_reflect_struct(cfg: &Config, item: syn::ItemStruct) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let (impl_bounds, where_bounds) = item.reflect_bounds(cfg);
    let name = item.name(cfg, NameTy::Path);

    let trait_name = match item.ty() {
        StructType::Named => "ReflectedStruct",
        StructType::Tuple => "ReflectedTupleStruct",
        StructType::Unit => "ReflectedUnitStruct",
    };

    let trait_name = syn::Ident::new(trait_name, Span::call_site());

    let fields = match item.ty() {
        StructType::Unit => None,
        _ => {
            let fields = item
                .fields()
                .into_iter()
                .enumerate()
                .map(|(idx, field)| generate_struct_field(cfg, &item, idx, field))
                .collect::<Result<Vec<_>>>()?;

            Some(quote_spanned!(item.span() =>
                #[allow(unused_unsafe)]
                fn fields() -> ::std::vec::Vec<#crate_name::Field> {
                    ::std::vec![ #( #fields, )* ]
                }
            ))
        },
    };

    Ok(quote_spanned!(item.span() =>
        impl #impl_bounds #crate_name::reflect::#trait_name for #name #where_bounds {
            #fields
        }
    ))
}

pub fn generate_reflect_union(cfg: &Config, item: syn::ItemUnion) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let (impl_bounds, where_bounds) = item.reflect_bounds(cfg);
    let name = item.name(cfg, NameTy::Path);

    let fields = item
        .fields
        .named
        .iter()
        .enumerate()
        .map(|(idx, field)| generate_union_field(cfg, &name, idx, field))
        .collect::<Result<Vec<_>>>()?;

    Ok(quote_spanned!(item.span() =>
        impl #impl_bounds #crate_name::reflect::ReflectedUnion for #name #where_bounds {
            fn fields() -> ::std::vec::Vec<#crate_name::UnionField> {
                ::std::vec![ #(#fields,)* ]
            }
        }
    ))
}

pub fn generate_reflect_type(cfg: &Config, item: &Item) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let (reflect_impl_bounds, reflect_where_bounds) = item.reflect_bounds(cfg);
    let (out_impl_bounds, out_where_bounds) = item.outlives_bounds(cfg);
    let name = item.name(cfg, NameTy::Path);
    let lifetime_name = item.name(cfg, NameTy::LifetimePath);
    let static_name = item.name(cfg, NameTy::StaticPath);
    let rebound_name = item.name(cfg, NameTy::ReboundName);
    let new_fn = item.new_fn_name();

    Ok(quote_spanned!(item.span() =>
        impl #reflect_impl_bounds #crate_name::reflect::Reflected for #name #reflect_where_bounds {
            type Key = #static_name;

            fn name() -> ::std::string::String {
                #rebound_name
            }

            unsafe fn init() {
                #crate_name::Type::#new_fn::<#name>();
            }
        }

        // NotOutlives bound generation:
        //   Impl for NotOutlives<'no>
        //   where:
        //     'no: 't0 .. 'tn + 'l0 .. 'ln
        //     T0: NotOutlives<'t0>
        //     ...
        //     TN: NotOutlives<'tn>
        // SAFETY: Generated implementation uses the above algorithm and is ensured correct
        unsafe impl #out_impl_bounds #crate_name::value::NotOutlives<'no> for #lifetime_name #out_where_bounds {}
    ))
}

pub fn generate_reflect(cfg: &Config, item: Item) -> Result<TokenStream> {
    let reflected_impl = match &item {
        Item::Struct(_) | Item::Enum(_) | Item::Union(_) => generate_reflect_type(cfg, &item),
        _ => Ok(TokenStream::new()),
    }?;

    let item_out = match item {
        Item::Struct(item) => generate_reflect_struct(cfg, item),
        Item::Enum(item) => generate_reflect_enum(cfg, item),
        Item::Union(item) => generate_reflect_union(cfg, item),
        Item::Impl(item) => generate_reflect_impl(cfg, item),
        _ => Err(Error::NotSupported(String::from("non-data-type items"))),
    }?;

    Ok(quote!(
        #reflected_impl
        #item_out
    ))
}
