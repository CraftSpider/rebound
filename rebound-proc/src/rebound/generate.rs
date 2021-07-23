use super::utils::*;
use super::{Config, Result};

use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::sync::RwLock;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

pub fn generate_assoc_fn(
    cfg: &Config,
    self_ty: &syn::Type,
    sig: &syn::Signature,
) -> Result<TokenStream> {
    if !sig.generics.params.is_empty() {
        return Err(
            "Rebound does not support generic functions, this may work in a future version"
                .to_string(),
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
            syn::FnArg::Typed(pat_ty) => {
                let ty = &pat_ty.ty;
                Some(quote!(#ty))
            }
        })
        .collect::<Vec<_>>();

    let ret_ty = match output {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, ty) => quote!(#ty),
    };

    if !inputs.is_empty() && matches!(inputs[0], syn::FnArg::Receiver(..)) {
        let receiver = if let syn::FnArg::Receiver(arg) = &inputs[0] {
            if arg.reference.is_some() {
                let mutability = &arg.mutability;
                quote!(& #mutability #self_ty)
            } else {
                quote!(#self_ty)
            }
        } else {
            unreachable!()
        };

        Ok(quote!(
            #crate_name::AssocFn::new_dynamic(
                #[allow(unused_mut, unused_variables)]
                |this, mut args| {
                    use ::core::convert::From;
                    let v = #crate_name::Value::from( <#self_ty>::#fn_name(this.cast_unsafe::<#receiver>(), #( args.remove(0).cast_unsafe::<#args>(), )* ) );
                    // SAFETY: Value cannot be safely constructed with a `'a` that outlives the T.
                    //         As such, we know that the lifetimes here should never be violated.
                    unsafe { ::core::mem::transmute::<#crate_name::Value, #crate_name::Value>(v) }
                },
                stringify!(#fn_name),
                #crate_name::Type::from::<#self_ty>(),
                #crate_name::Type::from::<#receiver>(),
                &[#( #crate_name::Type::from::<#args>(), )*],
                #crate_name::Type::from::<#ret_ty>(),
            )
        ))
    } else {
        Ok(quote!(
            #crate_name::AssocFn::new_static(
                #[allow(unused_mut, unused_variables)]
                |mut args| {
                    use ::core::convert::From;
                    let v = #crate_name::Value::from( <#self_ty>::#fn_name( #( args.remove(0).cast_unsafe::<#args>(), )* ) );
                    // SAFETY: Value cannot be safely constructed with a `'a` that outlives the T.
                    //         As such, we know that the lifetimes here should never be violated.
                    unsafe { ::core::mem::transmute::<#crate_name::Value, #crate_name::Value>(v) }
                },
                stringify!(#fn_name),
                #crate_name::Type::from::<#self_ty>(),
                &[#( #crate_name::Type::from::<#args>(), )*],
                #crate_name::Type::from::<#ret_ty>(),
            )
        ))
    }
}

pub fn generate_assoc_const(
    cfg: &Config,
    self_ty: &syn::Type,
    const_name: &syn::Ident,
    const_ty: &syn::Type,
) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;

    Ok(quote!(
        #crate_name::AssocConst::new(
            ::std::boxed::Box::new(|| {
                use ::core::convert::From;
                #crate_name::Value::from(<#self_ty>::#const_name)
            }),
            stringify!(#const_name),
            #crate_name::Type::from::<#self_ty>(),
            #crate_name::Type::from::<#const_ty>(),
        )
    ))
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
            quote!(new_named),
            quote!(stringify!(#field_name)),
        ),
        None => {
            let access = syn::Index::from(idx);
            (quote!(#access), quote!(new_tuple), quote!(#idx))
        }
    };

    let accessor = if !no_get {
        quote!(::core::option::Option::Some(|this| {
            let inner = this.borrow_unsafe::<#name>();
            let v = #crate_name::Value::from_ref(&inner.#field_name);
            // SAFETY: See rebound::ty::Ref
            #[allow(unused_unsafe)]
            unsafe { ::core::mem::transmute(v) }
        }))
    } else {
        quote!(None)
    };

    let setter = if !no_set {
        quote!(::core::option::Option::Some(|this, value| {
            let inner = this.borrow_unsafe_mut::<#name>();
            inner.#field_name = value.cast_unsafe();
        }))
    } else {
        quote!(None)
    };

    Ok(quote!(
        #crate_name::Field::#fn_name(
            #accessor,
            #setter,
            #name_arg,
            #crate_name::Type::from::<#name>(),
            #crate_name::Type::from::<#field_ty>(),
        )
    ))
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
        quote!(::core::option::Option::Some(|this| {
            let inner = this.borrow_unsafe::<#name>();
            if let #simple_name::#var_name #field_access = inner {
                let v = #crate_name::Value::from_ref(field);
                // SAFETY: Value cannot be safely constructed with a `'a` that outlives the T.
                //         As such, we know that the lifetimes here should never be violated.
                ::core::mem::transmute::<#crate_name::Value, #crate_name::Value>(v)
            } else {
                ::core::unreachable!()
            }
        }))
    } else {
        quote!(None)
    };

    let setter = if !no_set {
        quote!(::core::option::Option::Some(|this, value| {
            let inner = this.borrow_unsafe_mut::<#name>();
            if let #simple_name::#var_name #field_access = inner {
                *field = value.cast_unsafe::<#field_ty>();
            } else {
                ::core::unreachable!()
            }
        }))
    } else {
        quote!(None)
    };

    Ok(quote!(
        #crate_name::Field::#fn_name(
            #accessor,
            #setter,
            #name_arg,
            #crate_name::Type::from::<#name>(),
            if let #crate_name::Type::Enum(info) = #crate_name::Type::from::<#name>() {
                use ::core::iter::{Iterator, IntoIterator};
                info.variants().into_iter().filter(|var| var.name() == stringify!(#var_name)).nth(0).unwrap()
            } else {
                ::core::unreachable!()
            },
            #crate_name::Type::from::<#field_ty>(),
        )
    ))
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
    let field_name = field.ident.as_ref().unwrap();

    let accessor = if !no_get {
        quote!(::core::option::Option::Some(|this| {
            let inner = this.borrow_unsafe::<#name>();
            let v = #crate_name::Value::from_ref(unsafe { &inner.#field_name });
            // SAFETY: Value cannot be safely constructed with a `'a` that outlives the T.
            //         As such, we know that the lifetimes here should never be violated.
            ::core::mem::transmute::<#crate_name::Value, #crate_name::Value>(v)
        }))
    } else {
        quote!(None)
    };

    let setter = if !no_set {
        quote!(::core::option::Option::Some(|this, value| {
            let inner = this.borrow_unsafe_mut::<#name>();
            unsafe { inner.#field_name = value.cast_unsafe::<#field_ty>() };
        }))
    } else {
        quote!(None)
    };

    Ok(quote!(
        #crate_name::UnionField::new(
            #accessor,
            #setter,
            stringify!(#field_name),
            #crate_name::Type::from::<#name>(),
            #crate_name::Type::from::<#field_ty>(),
        )
    ))
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

    match &variant.fields {
        syn::Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .enumerate()
                .map(|(idx, field)| generate_enum_field(cfg, item, var_name, idx, field))
                .collect::<Result<Vec<_>>>()?;

            Ok(quote!(
                #crate_name::Variant::Struct(#crate_name::info::StructVariant::new(
                    stringify!(#var_name),
                    #crate_name::Type::from::<#name>(),
                    || { ::std::vec![ #(#fields),* ] },
                    |val| {
                        if let #simple_name::#var_name { .. } = unsafe { val.borrow_unsafe::<#name>() } {
                            true
                        } else {
                            false
                        }
                    }
                ))
            ))
        }
        syn::Fields::Unit => Ok(quote!(
            #crate_name::Variant::Unit(#crate_name::info::UnitVariant::new(
                stringify!(#var_name),
                #crate_name::Type::from::<#name>(),
                |val| {
                    if let #name::#var_name = val.borrow_unsafe::<#name>() {
                        true
                    } else {
                        false
                    }
                },
            ))
        )),
        syn::Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, field)| generate_enum_field(cfg, item, var_name, idx, field))
                .collect::<Result<Vec<_>>>()?;

            Ok(quote!(
                #crate_name::Variant::Tuple(#crate_name::info::TupleVariant::new(
                    stringify!(#var_name),
                    #crate_name::Type::from::<#name>(),
                    || { ::std::vec![ #(#fields),* ] },
                    |val| {
                        if let #name::#var_name(..) = val.borrow_unsafe::<#name>() {
                            true
                        } else {
                            false
                        }
                    }
                ))
            ))
        }
    }
}

pub fn generate_reflect_enum(cfg: &Config, item: syn::ItemEnum) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let (impl_bounds, where_bounds) = item.reflect_bounds(cfg);
    let name = item.name(cfg, NameTy::Path);

    let mut variant_impls = Vec::new();

    for i in &item.variants {
        variant_impls.push(generate_variant(cfg, &item, i)?)
    }

    Ok(quote!(
        impl #impl_bounds #crate_name::reflect::ReflectedEnum for #name #where_bounds {
            fn variants() -> ::std::vec::Vec<#crate_name::Variant> {
                unsafe {
                    ::std::vec![
                        #(#variant_impls),*
                    ]
                }
            }
        }
    ))
}

static IMPLS_PER_TY: SyncOnceCell<RwLock<HashMap<String, u8>>> = SyncOnceCell::new();

pub fn generate_reflect_impl(cfg: &Config, item: syn::ItemImpl) -> Result<TokenStream> {
    if item.trait_.is_some() {
        return Err(
            "Rebound does not yet support trait reflection, this may work in a future version"
                .to_string(),
        );
    }
    if !item.generics.params.is_empty() {
        return Err(
            "Rebound does not yet support generic impls, this may work in a future version"
                .to_string(),
        );
    }

    let mut impls = IMPLS_PER_TY
        .get_or_init(|| RwLock::new(HashMap::new()))
        .write()
        .unwrap();

    let crate_name = &cfg.crate_name;
    let self_ty = &item.self_ty;
    let id = ty_id(self_ty)?;

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

    let out = quote!(
        impl #crate_name::reflect::ReflectedImpl<#num> for #self_ty {
            fn assoc_fns() -> ::std::vec::Vec<#crate_name::AssocFn> {
                unsafe {
                    ::std::vec![
                        #(#impl_fns,)*
                    ]
                }
            }

            fn assoc_consts() -> ::std::vec::Vec<#crate_name::AssocConst> {
                unsafe {
                    ::std::vec![
                        #(#impl_consts,)*
                    ]
                }
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

    let struct_impl;

    match &item.fields {
        syn::Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .enumerate()
                .map(|(idx, field)| generate_struct_field(cfg, &item, idx, field))
                .collect::<Result<Vec<_>>>()?;

            struct_impl = quote!(
                impl #impl_bounds #crate_name::reflect::ReflectedStruct for #name #where_bounds {
                    #[allow(unused_unsafe)]
                    fn fields() -> ::std::vec::Vec<#crate_name::Field> {
                        unsafe {
                            ::std::vec![
                                #(#fields,)*
                            ]
                        }
                    }
                }
            )
        }
        syn::Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, field)| generate_struct_field(cfg, &item, idx, field))
                .collect::<Result<Vec<_>>>()?;

            struct_impl = quote!(
                impl #impl_bounds #crate_name::reflect::ReflectedTupleStruct for #name #where_bounds {
                    #[allow(unused_unsafe)]
                    fn fields() -> ::std::vec::Vec<#crate_name::Field> {
                        unsafe {
                            ::std::vec![
                                #(#fields,)*
                            ]
                        }
                    }
                }
            )
        }
        syn::Fields::Unit => {
            struct_impl = quote!(
                impl #impl_bounds #crate_name::reflect::ReflectedUnitStruct for #name #where_bounds {}
            )
        }
    }

    Ok(quote!(
        #struct_impl
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

    Ok(quote!(
        impl #impl_bounds #crate_name::reflect::ReflectedUnion for #name #where_bounds {
            #[allow(unused_unsafe)]
            fn fields() -> ::std::vec::Vec<#crate_name::UnionField> {
                unsafe {
                    ::std::vec![
                        #(#fields,)*
                    ]
                }
            }
        }
    ))
}

pub fn generate_reflect_type(cfg: &Config, item: &Item) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let (reflect_impl_bounds, reflect_where_bounds) = item.reflect_bounds(cfg);
    let (out_impl_bounds, out_where_bounds) = item.outlives_bounds(cfg);
    let name = item.name(cfg, NameTy::Path);
    let static_name = item.name(cfg, NameTy::StaticPath);
    let rebound_name = item.name(cfg, NameTy::ReboundName);
    let new_fn = item.new_fn_name();

    Ok(quote!(
        impl #reflect_impl_bounds #crate_name::reflect::Reflected for #name #reflect_where_bounds {
            type Key = #static_name;

            fn name() -> ::std::string::String {
                #rebound_name
            }

            unsafe fn init() {
                #crate_name::Type::#new_fn::<#name>();
            }
        }

        // For each T, need `T: NotOutlives<'a>, 'b: 'a`
        unsafe impl #out_impl_bounds #crate_name::value::NotOutlives<'_> for #name #out_where_bounds {}
    ))
}

pub fn generate_reflect_trait(_item: syn::ItemTrait) -> Result<TokenStream> {
    todo!()
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
        Item::Trait(item) => generate_reflect_trait(item),
        Item::Impl(item) => generate_reflect_impl(cfg, item),
        _ => unreachable!(),
    }?;

    Ok(quote!(
        #reflected_impl
        #item_out
    ))
}
