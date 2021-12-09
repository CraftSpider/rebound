use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::{Item, Token};

mod generate;
mod utils;

use generate::*;
use utils::*;

type Result<T> = core::result::Result<T, String>;

struct AttrInput {
    values: Punctuated<syn::NestedMeta, Token![,]>,
}

impl Parse for AttrInput {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let values = Punctuated::parse_terminated(input)?;

        Ok(AttrInput { values })
    }
}

pub struct Config {
    crate_name: syn::Ident,
    name_replace: Option<(String, String)>,
    debug_out: bool,
    no_get: bool,
    no_set: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            crate_name: syn::Ident::new("rebound", Span::call_site()),
            name_replace: None,
            debug_out: false,
            no_get: false,
            no_set: false,
        }
    }
}

fn parse_attrs(attrs: TokenStream) -> Result<Config> {
    let args: AttrInput = syn::parse2(attrs).map_err(|err| err.to_string())?;

    let mut crate_name = None;
    let mut name_replace = None;
    let mut debug_out = false;

    let mut no_get = false;
    let mut no_set = false;

    for i in args.values {
        match i {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::List(..) => return Err("Found unexpected list element".to_string()),
                syn::Meta::NameValue(nv) => {
                    let str = path_to_string(&nv.path);

                    if str == "crate_name" {
                        crate_name = Some(lit_as_str(&nv.lit)?);
                    } else if str == "name_replace" {
                        let str = lit_as_str(&nv.lit)?;
                        let (pat, replace) = str.split_once("/").ok_or_else(|| {
                            "name_replace should be a / delimited pair of patterns".to_string()
                        })?;
                        name_replace = Some((pat.to_owned(), replace.to_owned()));
                    } else {
                        return Err(format!(
                            "Found unexpected name/value pair {}",
                            path_to_string(&nv.path)
                        ));
                    }
                }
                syn::Meta::Path(path) => {
                    let str = path_to_string(&path);

                    if str == "debug_out" {
                        debug_out = true;
                    } else if str == "no_get" {
                        no_get = true;
                    } else if str == "no_set" {
                        no_set = true;
                    } else {
                        return Err(format!(
                            "Found unexpected path element {}",
                            path_to_string(&path)
                        ));
                    }
                }
            },
            syn::NestedMeta::Lit(..) => return Err("Found unexpected literal argument".to_string()),
        }
    }

    let crate_name = syn::Ident::new(
        &crate_name.unwrap_or_else(|| "rebound".to_string()),
        Span::call_site(),
    );

    Ok(Config {
        crate_name,
        debug_out,
        name_replace,
        no_get,
        no_set,
    })
}

fn verify_item(input: TokenStream) -> Result<Item> {
    let item = syn::parse2(input).map_err(|err| {
        eprintln!("SYN PARSE ERROR");
        err.to_string()
    })?;

    let err = match &item {
        Item::Enum(..) | Item::Impl(..) | Item::Struct(..) | Item::Trait(..) | Item::Union(..) => {
            None
        }

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
        Item::Use(..) => Some("a use declaration"),
        Item::Verbatim(..) => Some("an unknown top-level item"),
        _ => Some("an unhandled item"),
    };

    match err {
        Some(name) => Err(format!("#[rebound] can only be applied to a struct, enum, trait, or impl block. Instead got {}", name)),
        None => Ok(item)
    }
}

struct Items {
    items: Vec<syn::Item>,
}

impl syn::parse::Parse for Items {
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<Items> {
        Ok(Items {
            items: {
                let mut items = Vec::new();
                while !parser.is_empty() {
                    items.push(parser.parse()?);
                }
                items
            },
        })
    }
}

pub fn extern_items(input: TokenStream) -> TokenStream {
    let config = Config {
        crate_name: syn::Ident::new("crate", Span::call_site()),
        name_replace: Some(("rebound::__impls::".into(), "".into())),
        no_get: true,
        no_set: true,
        ..Default::default()
    };

    let item = syn::parse2(input)
        .map_err(|err| err.to_string())
        .and_then(|Items { items }| {
            items
                .into_iter()
                .map(|item| generate_reflect(&config, item))
                .collect::<Result<Vec<_>>>()
        })
        .map(|vec| {
            vec.into_iter().fold(TokenStream::new(), |mut acc, ts| {
                acc.append_all(ts);
                acc
            })
        });

    match item {
        Ok(ts) => {
            if config.debug_out {
                println!("extern_items! generated code: {}", ts.to_string())
            }
            ts
        }
        Err(msg) => quote!(compile_error!(#msg)),
    }
}

#[allow(dead_code)]
struct FnSig {
    attrs: Vec<syn::Attribute>,
    sig: syn::Signature,
}

impl syn::parse::Parse for FnSig {
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<FnSig> {
        Ok(FnSig {
            attrs: syn::Attribute::parse_outer(parser)?,
            sig: parser.parse()?,
        })
    }
}

struct AssocFnSigs {
    ty: syn::Type,
    #[allow(dead_code)]
    at: Token![@],
    defs: Punctuated<FnSig, Token![;]>,
}

impl syn::parse::Parse for AssocFnSigs {
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<AssocFnSigs> {
        Ok(AssocFnSigs {
            ty: parser.parse()?,
            at: parser.parse()?,
            defs: parser.parse_terminated(FnSig::parse)?,
        })
    }
}

pub fn extern_assoc_fns(input: TokenStream) -> TokenStream {
    let config = Config {
        crate_name: syn::Ident::new("crate", Span::call_site()),
        ..Default::default()
    };

    let input: AssocFnSigs = syn::parse2(input).expect("Couldn't parse assocfn def");

    let mut output = Vec::new();
    for sig in &input.defs {
        let attrs = &sig.attrs;
        let afn = generate_assoc_fn(&config, &input.ty, &sig.sig);
        match afn {
            Ok(ts) => output.push(quote!(#(#attrs)* #ts)),
            Err(msg) => return quote!(compile_error!(#msg)),
        }
    }

    quote!(unsafe { ::std::vec![ #(#output,)* ] })
}

#[allow(dead_code)]
struct IdentType {
    attrs: Vec<syn::Attribute>,
    ident: Box<syn::Ident>,
    colon_token: Token![:],
    ty: Box<syn::Type>,
}

impl syn::parse::Parse for IdentType {
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<IdentType> {
        Ok(IdentType {
            attrs: syn::Attribute::parse_outer(parser)?,
            ident: parser.parse()?,
            colon_token: parser.parse()?,
            ty: parser.parse()?,
        })
    }
}

#[allow(dead_code)]
struct AssocConstSigs {
    ty: syn::Type,
    at: Token![@],
    defs: Punctuated<IdentType, Token![;]>,
}

impl syn::parse::Parse for AssocConstSigs {
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<AssocConstSigs> {
        Ok(AssocConstSigs {
            ty: parser.parse()?,
            at: parser.parse()?,
            defs: parser.parse_terminated(IdentType::parse)?,
        })
    }
}

pub fn extern_assoc_consts(input: TokenStream) -> TokenStream {
    let config = Config {
        crate_name: syn::Ident::new("crate", Span::call_site()),
        ..Default::default()
    };

    let input: AssocConstSigs = syn::parse2(input).expect("Couldn't parse assocconst def");

    let mut output = Vec::new();
    for sig in &input.defs {
        let aconst = generate_assoc_const(&config, &input.ty, &sig.ident, &sig.ty);
        match aconst {
            Ok(ts) => output.push(ts),
            Err(msg) => return quote!(compile_error!(#msg)),
        }
    }

    quote!(unsafe { ::std::vec![ #(#output,)* ] })
}

pub fn post_wire(input: TokenStream) -> TokenStream {
    let mut item: syn::ItemImpl = syn::parse2(input)
        .unwrap();
    generate_post_wire(&mut item);
    quote!( #item )
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
