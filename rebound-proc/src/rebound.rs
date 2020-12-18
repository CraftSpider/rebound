use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::sync::RwLock;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Item, Token};

mod utils;
use utils::*;

type Result<T> = std::result::Result<T, String>;

struct AttrInput {
    values: Punctuated<syn::NestedMeta, Token![,]>,
}

impl Parse for AttrInput {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let values = Punctuated::parse_terminated(input)?;

        Ok(AttrInput { values })
    }
}

struct Config {
    crate_name: syn::Ident,
    debug_out: bool,
}

fn parse_attrs(attrs: TokenStream) -> Result<Config> {
    let args: AttrInput = syn::parse2(attrs).map_err(|err| err.to_string())?;

    let mut crate_name = None;
    let mut debug_out = false;

    for i in args.values {
        match i {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::List(..) => return Err(format!("Found unexpected list element")),
                syn::Meta::NameValue(nv) => {
                    if path_to_string(&nv.path) == "crate_name" {
                        crate_name = Some(lit_as_str(&nv.lit)?);
                    } else {
                        return Err(format!(
                            "Found unexpected name/value pair {}",
                            path_to_string(&nv.path)
                        ));
                    }
                }
                syn::Meta::Path(path) => {
                    if path_to_string(&path) == "debug_out" {
                        debug_out = true;
                    } else {
                        return Err(format!(
                            "Found unexpected path element {}",
                            path_to_string(&path)
                        ));
                    }
                }
            },
            syn::NestedMeta::Lit(..) => return Err(format!("Found unexpected literal argument")),
        }
    }

    let crate_name = syn::Ident::new(
        &crate_name.unwrap_or_else(|| "rebound".to_string()),
        Span::call_site(),
    );

    Ok(Config {
        crate_name,
        debug_out,
    })
}

fn verify_item(input: TokenStream) -> Result<Item> {
    let item = syn::parse2(input).map_err(|err| {
        eprintln!("SYN PARSE ERROR");
        err.to_string()
    })?;

    let err = match &item {
        Item::Enum(..) | Item::Impl(..) | Item::Struct(..) | Item::Trait(..) => None,

        Item::Const(..) => Some("a const"),
        Item::ExternCrate(..) => Some("an extern crate"),
        Item::Fn(..) => Some("a function"),
        Item::ForeignMod(..) => Some("an extern block"),
        Item::Macro(..) => Some("a macro invocation"),
        Item::Macro2(..) => Some("a decl macro"),
        Item::Mod(..) => Some("a module"),
        Item::Static(..) => Some("a static"),
        Item::TraitAlias(..) => Some("a trait alias"),
        Item::Type(..) => Some("a type alias"),
        Item::Union(..) => Some("a union"),
        Item::Use(..) => Some("a use declaration"),
        Item::Verbatim(..) => Some("an unknown top-level item"),
        _ => Some("an unhandled item"),
    };

    match err {
        Some(name) => Err(format!("#[rebound] can only be applied to a struct, enum, trait, or impl block. Instead got {}", name)),
        None => Ok(item)
    }
}

fn generate_reflect_enum(cfg: &Config, item: syn::ItemEnum) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let impl_bounds = impl_bounds(&item.generics);
    let name = item_name(&item.ident, &item.generics);
    let pat_name = item_pattern_name(&item.ident, &item.generics);
    let qual_name = item_qual_name(&item.ident, &item.generics);

    let mut variant_impls = Vec::new();

    for i in &item.variants {
        let var_name = &i.ident;

        match &i.fields {
            syn::Fields::Named(fields) => {
                let fields = fields.named.iter()
                    .map(|field| {
                        let field_name = &field.ident;
                        let field_ty = sanitized_field_ty(&field.ty);

                        quote!(
                            #crate_name::Field::new_enum_named(
                                #crate_name::__helpers::__make_enum_named_ref_accessor!(#name, #name::#var_name, #field_name),
                                #crate_name::__helpers::__make_enum_named_setter!(#name, #name::#var_name, #field_name),
                                stringify!(#field_name),
                                #crate_name::Type::from::<#name>(),
                                if let #crate_name::Type::Enum(info) = #crate_name::Type::from::<#name>() {
                                    *info.variants().iter().filter(|var| var.name() == stringify!(#var_name)).collect::<Vec<_>>()[0]
                                } else {
                                    unreachable!()
                                },
                                #crate_name::Type::from::<#field_ty>(),
                            )
                        )
                    })
                    .collect::<Vec<_>>();

                variant_impls.push(quote!(
                    #crate_name::VariantInfo::Struct(#crate_name::info::StructVariant::new(
                        stringify!(#var_name),
                        #crate_name::Type::from::<#name>(),
                        || { vec![ #(#fields),* ] }
                    ))
                ))
            }
            syn::Fields::Unnamed(fields) => {
                let fields = fields.unnamed.iter()
                    .enumerate()
                    .map(|(idx, field)| {
                        let field_ty = sanitized_field_ty(&field.ty);

                        let mut skip: Vec<_> = Vec::new();
                        for _ in 0..idx {
                            skip.push(syn::Ident::new("_", Span::call_site()));
                        }

                        quote!(
                            #crate_name::Field::new_enum_tuple(
                                Box::new(|this| {
                                    let inner = this.borrow::<#name>();
                                    if let #pat_name::#var_name(#(#skip,)* wanted, ..) = inner {
                                        #crate_name::Value::from_ref(wanted)
                                    } else {
                                        unreachable!()
                                    }
                                }),
                                Box::new(|this, value| {
                                    let inner = this.borrow_mut::<#name>();
                                    if let #pat_name::#var_name(#(#skip,)* wanted, ..) = inner {
                                        *wanted = value.cast();
                                    } else {
                                        unreachable!()
                                    }
                                }),
                                #idx,
                                #crate_name::Type::from::<#name>(),
                                if let #crate_name::Type::Enum(info) = #crate_name::Type::from::<#name>() {
                                    *info.variants().iter().filter(|var| var.name() == stringify!(#var_name)).collect::<Vec<_>>()[0]
                                } else {
                                    unreachable!()
                                },
                                #crate_name::Type::from::<#field_ty>(),
                            )
                        )
                    })
                    .collect::<Vec<_>>();

                variant_impls.push(quote!(
                    #crate_name::VariantInfo::Tuple(#crate_name::info::TupleVariant::new(
                        stringify!(#var_name),
                        #crate_name::Type::from::<#name>(),
                        || { vec![ #(#fields),* ] }
                    ))
                ))
            }
            syn::Fields::Unit => variant_impls.push(quote!(
                #crate_name::VariantInfo::Unit(#crate_name::info::UnitVariant::new(
                    stringify!(#i),
                    #crate_name::Type::from::<#name>(),
                ))
            )),
        }
    }

    Ok(quote!(
        impl #impl_bounds #crate_name::reflect::Reflected for #name {
            fn name() -> String {
                #qual_name.to_string()
            }

            unsafe fn init() {
                #crate_name::Type::new_enum::<#name>()
            }
        }

        impl #impl_bounds #crate_name::reflect::ReflectedEnum for #name {
            fn variants() -> Vec<#crate_name::VariantInfo> {
                unsafe {
                    vec![
                        #(#variant_impls),*
                    ]
                }
            }
        }
    ))
}

static IMPLS_PER_TY: SyncOnceCell<RwLock<HashMap<String, u8>>> = SyncOnceCell::new();

fn generate_reflect_impl(cfg: &Config, item: syn::ItemImpl) -> Result<TokenStream> {
    if item.trait_.is_some() {
        return Err(
            "Rebound does not yet support trait reflection, this may work in a future version"
                .to_string(),
        );
    }
    if item.generics.params.len() > 0 {
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
                let fn_name = &impl_item.sig.ident;

                let (helper, receiver_ty) = if impl_item.sig.inputs.len() > 0
                    && matches!(impl_item.sig.inputs[0], syn::FnArg::Receiver(..))
                {
                    let receiver = if let syn::FnArg::Receiver(arg) = &impl_item.sig.inputs[0] {
                        if arg.reference.is_some() {
                            let mutability = &arg.mutability;
                            quote!(& #mutability #self_ty)
                        } else {
                            quote!(#self_ty)
                        }
                    } else {
                        unreachable!()
                    };

                    (
                        quote!(#crate_name::__helpers::__make_dyn_helper!(#self_ty::#fn_name, #receiver)),
                        quote!(Some(#crate_name::Type::from::<#receiver>())),
                    )
                } else {
                    (
                        quote!(#crate_name::__helpers::__make_static_helper!(#self_ty::#fn_name)),
                        quote!(None),
                    )
                };

                let args = impl_item
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        syn::FnArg::Receiver(..) => None,
                        syn::FnArg::Typed(ty) => Some(quote!( #crate_name::Type::from::<#ty>() )),
                    })
                    .collect::<Vec<_>>();

                let ret_ty = match &impl_item.sig.output {
                    syn::ReturnType::Default => quote!(()),
                    syn::ReturnType::Type(_, ty) => quote!(#ty),
                };

                impl_fns.push(quote!(
                    #crate_name::AssocFn::new(
                        #helper,
                        stringify!(#fn_name),
                        #crate_name::Type::from::<#self_ty>(),
                        #receiver_ty,
                        &[#(#args,)*],
                        #crate_name::Type::from::<#ret_ty>(),
                    )
                ));
            }
            syn::ImplItem::Const(impl_item) => {
                let const_name = &impl_item.ident;
                let const_ty = &impl_item.ty;

                let helper = quote!( #crate_name::__helpers::__make_const_accessor!(#self_ty::#const_name) );

                impl_consts.push(quote!(
                    #crate_name::AssocConst::new(
                        #helper,
                        stringify!(#const_name),
                        #crate_name::Type::from::<#self_ty>(),
                        #crate_name::Type::from::<#const_ty>(),
                    )
                ));
            }
            _ => {
                use proc_macro::{Diagnostic, Level};
                Diagnostic::new(Level::Warning, "Rebound currently only supports reflecting fns and consts in impls")
                    .emit();
            }
        }
    }

    let out = quote!(
        impl #crate_name::reflect::ReflectedImpl<#num> for #self_ty {
            fn assoc_fns() -> Option<Vec<#crate_name::AssocFn>> {
                unsafe {
                    Some(vec![
                        #(#impl_fns,)*
                    ])
                }
            }

            fn assoc_consts() -> Option<Vec<#crate_name::AssocConst>> {
                unsafe {
                    Some(vec![
                        #(#impl_consts,)*
                    ])
                }
            }
        }
    );

    *num += 1;

    Ok(out)
}

fn generate_reflect_struct(cfg: &Config, item: syn::ItemStruct) -> Result<TokenStream> {
    let crate_name = &cfg.crate_name;
    let impl_bounds = impl_bounds(&item.generics);
    let name = item_name(&item.ident, &item.generics);
    let qual_name = item_qual_name(&item.ident, &item.generics);

    let new_fn;
    let struct_impl;

    match &item.fields {
        syn::Fields::Named(fields) => {
            new_fn = syn::Ident::new("new_struct", item.span());

            let fields = fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_ty = sanitized_field_ty(&field.ty);

                    quote!(
                        #crate_name::Field::new_named(
                            #crate_name::__helpers::__make_ref_accessor!(#name, #field_name),
                            #crate_name::__helpers::__make_setter!(#name, #field_name),
                            stringify!(#field_name),
                            #crate_name::Type::from::<#name>(),
                            #crate_name::Type::from::<#field_ty>(),
                        )
                    )
                })
                .collect::<Vec<_>>();

            struct_impl = quote!(
                impl #impl_bounds #crate_name::reflect::ReflectedStruct for #name {
                    #[allow(unused_unsafe)]
                    fn fields() -> Vec<#crate_name::Field> {
                        unsafe {
                            vec![
                                #(#fields,)*
                            ]
                        }
                    }
                }
            )
        }
        syn::Fields::Unnamed(fields) => {
            new_fn = syn::Ident::new("new_tuple_struct", item.span());

            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    let access = syn::Index::from(idx);
                    let field_ty = sanitized_field_ty(&field.ty);

                    quote!(
                        #crate_name::Field::new_tuple(
                            #crate_name::__helpers::__make_ref_accessor!(#name, #access),
                            #crate_name::__helpers::__make_setter!(#name, #access),
                            #idx,
                            #crate_name::Type::from::<#name>(),
                            #crate_name::Type::from::<#field_ty>(),
                        )
                    )
                })
                .collect::<Vec<_>>();

            struct_impl = quote!(
                impl #impl_bounds #crate_name::reflect::ReflectedTupleStruct for #name {
                    #[allow(unused_unsafe)]
                    fn fields() -> Vec<#crate_name::Field> {
                        unsafe {
                            vec![
                                #(#fields,)*
                            ]
                        }
                    }
                }
            )
        }
        syn::Fields::Unit => {
            new_fn = syn::Ident::new("new_unit_struct", item.span());

            struct_impl = quote!(
                impl #impl_bounds #crate_name::reflect::ReflectedUnitStruct for #name {}
            )
        }
    }

    Ok(quote!(
        impl #impl_bounds #crate_name::reflect::Reflected for #name {
            fn name() -> String {
                #qual_name.to_string()
            }

            unsafe fn init() {
                #crate_name::Type::#new_fn::<#name>()
            }
        }

        #struct_impl
    ))
}

fn generate_reflect_trait(_item: syn::ItemTrait) -> Result<TokenStream> {
    todo!()
}

fn generate_reflect(cfg: &Config, item: Item) -> Result<TokenStream> {
    match item {
        Item::Enum(item) => generate_reflect_enum(cfg, item),
        Item::Impl(item) => generate_reflect_impl(cfg, item),
        Item::Struct(item) => generate_reflect_struct(cfg, item),
        Item::Trait(item) => generate_reflect_trait(item),
        _ => unreachable!(),
    }
}

fn rebound_impl(attrs: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let orig_item = item.clone();
    let cfg = parse_attrs(attrs)?;

    let item = verify_item(item)?;
    let gen_items = generate_reflect(&cfg, item)?;

    let final_output = quote!(
        #orig_item
        #gen_items
    );

    if cfg.debug_out {
        eprintln!("#[rebound] generated code: {}", final_output.to_string())
    }

    Ok(final_output)
}

pub fn rebound(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let res = rebound_impl(attrs, item);

    match res {
        Ok(ts) => ts,
        Err(msg) => quote!( compile_error!(#msg); ),
    }
}
