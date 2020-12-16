
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;
use syn::spanned::Spanned;

mod utils;
use utils::*;

type Result<T> = std::result::Result<T, String>;

fn verify_item(input: TokenStream) -> Result<Item> {
    let item = syn::parse2(input)
        .map_err(|err| { eprintln!("SYN PARSE ERROR"); err.to_string() })?;

    let err = match &item {
        Item::Enum(..)
        | Item::Impl(..)
        | Item::Struct(..)
        | Item::Trait(..) => None,

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
        _ => Some("an unhandled item")
    };

    match err {
        Some(name) => Err(format!("#[rebound] can only be applied to a struct, enum, trait, or impl block. Instead got {}", name)),
        None => Ok(item)
    }
}

fn impl_bounds(generics: &syn::Generics) -> TokenStream {
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

fn item_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
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

fn item_qual_name(name: &syn::Ident, generics: &syn::Generics) -> TokenStream {
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
                    quote!(concat!("const ", stringify!(#ident)))
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

fn sanitized_field_ty(ty: &syn::Type) -> TokenStream {
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

fn generate_reflect_enum(_item: syn::ItemEnum) -> Result<TokenStream> {
    todo!()
}

fn generate_reflect_impl(_item: syn::ItemImpl) -> Result<TokenStream> {
    todo!()
}

fn generate_reflect_struct(item: syn::ItemStruct) -> Result<TokenStream> {

    let impl_bounds = impl_bounds(&item.generics);
    let name = item_name(&item.ident, &item.generics);
    let qual_name = item_qual_name(&item.ident, &item.generics);

    let new_fn;
    let struct_impl;

    match &item.fields {
        syn::Fields::Named(fields) => {
            new_fn = syn::Ident::new("new_struct", item.span());

            let fields = fields.named.iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_ty = sanitized_field_ty(&field.ty);

                    quote!(
                        rebound::NamedField::new(
                            rebound::__helpers::__make_ref_accessor!(#name, #field_name),
                            rebound::__helpers::__make_setter!(#name, #field_name),
                            stringify!(#field_name),
                            rebound::TypeInfo::from::<#name>(),
                            rebound::TypeInfo::from::<#field_ty>(),
                        )
                    )
                })
                .collect::<Vec<_>>();

            struct_impl = quote!(
                impl #impl_bounds rebound::reflect::ReflectedStruct for #name {
                    #[allow(unused_unsafe)]
                    fn fields() -> Vec<rebound::NamedField> {
                        unsafe {
                            vec![
                                #(#fields,)*
                            ]
                        }
                    }
                }
            )
        },
        syn::Fields::Unnamed(fields) => {
            new_fn = syn::Ident::new("new_tuple_struct", item.span());

            let fields = fields.unnamed.iter()
                .enumerate()
                .map(|(idx, field)| {
                    let access = syn::Index::from(idx);
                    let field_ty = sanitized_field_ty(&field.ty);

                    quote!(
                        rebound::TupleField::new(
                            rebound::__helpers::__make_ref_accessor!(#name, #access),
                            rebound::__helpers::__make_setter!(#name, #access),
                            #idx,
                            rebound::TypeInfo::from::<#name>(),
                            rebound::TypeInfo::from::<#field_ty>(),
                        )
                    )
                })
                .collect::<Vec<_>>();

            struct_impl = quote!(
                impl #impl_bounds rebound::reflect::ReflectedTupleStruct for #name {
                    #[allow(unused_unsafe)]
                    fn fields() -> Vec<rebound::TupleField> {
                        unsafe {
                            vec![
                                #(#fields,)*
                            ]
                        }
                    }
                }
            )
        },
        syn::Fields::Unit => {
            new_fn = syn::Ident::new("new_unit_struct", item.span());

            struct_impl = quote!(
                impl #impl_bounds rebound::reflect::ReflectedUnitStruct for #name {}
            )
        },
    }

    Ok(quote!(
        impl #impl_bounds rebound::reflect::Reflected for #name {
            fn name() -> String {
                #qual_name.to_string()
            }

            unsafe fn init() {
                rebound::TypeInfo::#new_fn::<#name>()
            }
        }

        #struct_impl
    ))
}

fn generate_reflect_trait(_item: syn::ItemTrait) -> Result<TokenStream> {
    todo!()
}

fn generate_reflect(item: Item) -> Result<TokenStream> {
    match item {
        Item::Enum(item) => generate_reflect_enum(item),
        Item::Impl(item) => generate_reflect_impl(item),
        Item::Struct(item) => generate_reflect_struct(item),
        Item::Trait(item) => generate_reflect_trait(item),
        _ => unreachable!()
    }
}

pub fn rebound(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let gen_res = verify_item(item.clone())
        .and_then(generate_reflect);

    match gen_res {
        Ok(gen_items) => quote!(
            #item
            #gen_items
        ).into(),
        Err(msg) => quote!( compile_error!(#msg); )
    }
}
