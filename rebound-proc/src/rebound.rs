use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::{Item, Token};

mod generate;
mod utils;

use generate::*;
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

pub struct Config {
    crate_name: syn::Ident,
    debug_out: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            crate_name: syn::Ident::new("rebound", Span::call_site()),
            debug_out: false,
        }
    }
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

struct AssocFnSig {
    constness: Option<syn::Token![const]>,
    asyncness: Option<syn::Token![async]>,
    unsafety: Option<syn::Token![unsafe]>,
    abi: Option<syn::Abi>,
    fn_token: syn::Token![fn],
    ty: syn::Type,
    #[allow(dead_code)]
    at: syn::Token![@],
    ident: syn::Ident,
    generics: syn::Generics,
    paren_tok: syn::token::Paren,
    inputs: syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
    output: syn::ReturnType,
}

impl AssocFnSig {
    fn as_signature(&self) -> syn::Signature {
        syn::Signature {
            constness: self.constness.clone(),
            asyncness: self.asyncness.clone(),
            unsafety: self.unsafety.clone(),
            abi: self.abi.clone(),
            fn_token: self.fn_token.clone(),
            ident: self.ident.clone(),
            generics: self.generics.clone(),
            paren_token: self.paren_tok.clone(),
            inputs: self.inputs.clone(),
            variadic: None,
            output: self.output.clone(),
        }
    }
}

impl syn::parse::Parse for AssocFnSig {
    fn parse(parser: syn::parse::ParseStream) -> syn::Result<AssocFnSig> {
        let content;
        Ok(AssocFnSig {
            constness: parser.parse()?,
            asyncness: parser.parse()?,
            unsafety: parser.parse()?,
            abi: parser.parse()?,
            fn_token: parser.parse()?,
            ty: parser.parse()?,
            at: parser.parse()?,
            ident: parser.parse()?,
            generics: parser.parse()?,
            paren_tok: syn::parenthesized!(content in parser),
            inputs: content.parse_terminated(syn::FnArg::parse)?,
            output: parser.parse()?,
        })
    }
}

pub fn assoc_fn_from_def(input: TokenStream) -> TokenStream {
    let mut config = Config::default();
    config.crate_name = syn::Ident::new("crate", Span::call_site());

    let input: AssocFnSig = syn::parse2(input).expect("Couldn't parse assocfn def");

    let sig = input.as_signature();

    let res = generate_assoc_fn(&config, &input.ty, &sig);

    match res {
        Ok(ts) => ts,
        Err(msg) => quote!(compile_error!(#msg)),
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
