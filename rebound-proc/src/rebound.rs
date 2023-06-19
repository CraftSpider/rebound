use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Item, PathSegment, Token};

use crate::error::{Error, Result};
use crate::extension::PathExtension;

mod generate;
mod utils;

use generate::*;
use quasi_quote::expand::Expand;

#[derive(Clone, Debug)]
pub enum CrateName {
    Crate(syn::token::Crate),
    Ident(syn::Ident),
}

impl CrateName {
    fn local() -> CrateName {
        CrateName::Crate(Token![crate](Span::call_site()))
    }

    fn external() -> CrateName {
        CrateName::Ident(syn::Ident::new("rebound", Span::call_site()))
    }

    fn ident(&self) -> syn::Ident {
        match self {
            CrateName::Crate(_) => syn::Ident::new("crate", Span::call_site()),
            CrateName::Ident(i) => i.clone(),
        }
    }
}

impl Parse for CrateName {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Token![crate]) {
            Ok(CrateName::Crate(input.parse()?))
        } else {
            Ok(CrateName::Ident(input.parse()?))
        }
    }
}

impl ToTokens for CrateName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CrateName::Crate(t) => t.to_tokens(tokens),
            CrateName::Ident(i) => i.to_tokens(tokens),
        }
    }
}

impl Expand<PathSegment> for &CrateName {
    fn expand(self) -> PathSegment {
        PathSegment::from(self.ident())
    }
}

pub struct Config {
    crate_name: CrateName,
    name_replace: Option<(String, String)>,
    debug_out: bool,
    no_get: bool,
    no_set: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            crate_name: CrateName::external(),
            name_replace: None,
            debug_out: false,
            no_get: false,
            no_set: false,
        }
    }
}

fn parse_attrs(attrs: TokenStream) -> Result<Config> {
    let mut crate_name = None;
    let mut name_replace = None;
    let mut debug_out = false;

    let mut no_get = false;
    let mut no_set = false;

    if !attrs.is_empty() {
        syn::meta::parser(|meta| {
            let str = meta.path.as_simple_str().ok_or(syn::Error::new(
                meta.path.span(),
                "Expected a simple path without generics",
            ))?;

            if str == "crate_name" {
                let stream = meta.value()?;
                crate_name = Some(stream.parse::<syn::LitStr>()?.value());
            } else if str == "name_replace" {
                let stream = meta.value()?;
                let lit = stream.parse::<syn::LitStr>()?;
                let str = lit.value();
                let (pat, replace) = str.split_once("/").ok_or_else(|| {
                    syn::Error::new(
                        lit.span(),
                        "name_replace should be a / delimited pair of patterns",
                    )
                })?;
                name_replace = Some((pat.to_owned(), replace.to_owned()));
            } else if str == "debug_out" {
                debug_out = true;
            } else if str == "no_get" {
                no_get = true;
            } else if str == "no_set" {
                no_set = true;
            } else {
                return Err(syn::Error::new(
                    meta.path.span(),
                    format!("Found unexpected path element {}", str),
                ));
            }

            Ok(())
        })
        .parse2(attrs)
        .map_err(Error::ParseError)?;
    }

    let crate_name = crate_name.unwrap_or_else(|| "rebound".to_string());

    let crate_name =
        syn::parse_str::<CrateName>(&crate_name).map_err(|_| Error::InvalidIdent(crate_name))?;

    Ok(Config {
        crate_name,
        debug_out,
        name_replace,
        no_get,
        no_set,
    })
}

fn verify_item(input: TokenStream) -> Result<Item> {
    let item = syn::parse2(input)?;

    let err = match &item {
        Item::Enum(..) | Item::Impl(..) | Item::Struct(..) | Item::Trait(..) | Item::Union(..) => {
            None
        }

        Item::Const(..) => Some("a const"),
        Item::ExternCrate(..) => Some("an extern crate"),
        Item::Fn(..) => Some("a function"),
        Item::ForeignMod(..) => Some("an extern block"),
        Item::Macro(..) => Some("a macro invocation"),
        Item::Mod(..) => Some("a module"),
        Item::Static(..) => Some("a static"),
        Item::TraitAlias(..) => Some("a trait alias"),
        Item::Type(..) => Some("a type alias"),
        Item::Use(..) => Some("a use declaration"),
        Item::Verbatim(..) => Some("an unknown top-level item"),
        _ => Some("an unhandled item"),
    };

    match err {
        Some(name) => Err(Error::InvalidItem(name.to_string())),
        None => Ok(item),
    }
}

struct Items {
    items: Vec<syn::Item>,
}

impl Parse for Items {
    fn parse(parser: ParseStream<'_>) -> syn::Result<Items> {
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
        crate_name: CrateName::local(),
        name_replace: Some(("rebound::__impls::".into(), "".into())),
        no_get: true,
        no_set: true,
        ..Default::default()
    };

    let item = syn::parse2(input)
        .map_err(Error::from)
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
        Err(msg) => msg.to_compile_error(),
    }
}

struct FnSig {
    attrs: Vec<syn::Attribute>,
    sig: syn::Signature,
}

impl Parse for FnSig {
    fn parse(parser: ParseStream<'_>) -> syn::Result<FnSig> {
        Ok(FnSig {
            attrs: syn::Attribute::parse_outer(parser)?,
            sig: parser.parse()?,
        })
    }
}

struct AssocFnSigs {
    ty: syn::Type,
    _at: Token![@],
    defs: Punctuated<FnSig, Token![;]>,
}

impl Parse for AssocFnSigs {
    fn parse(parser: ParseStream<'_>) -> syn::Result<AssocFnSigs> {
        Ok(AssocFnSigs {
            ty: parser.parse()?,
            _at: parser.parse()?,
            defs: parser.parse_terminated(FnSig::parse, Token![;])?,
        })
    }
}

pub fn extern_assoc_fns(input: TokenStream) -> TokenStream {
    let config = Config {
        crate_name: CrateName::local(),
        ..Default::default()
    };

    let input: AssocFnSigs = syn::parse2(input).expect("Couldn't parse assocfn def");

    let mut output = Vec::new();
    for sig in &input.defs {
        let attrs = &sig.attrs;
        let afn = generate_assoc_fn(&config, &input.ty, &sig.sig);
        match afn {
            Ok(ts) => output.push(quote!(#(#attrs)* #ts)),
            Err(msg) => return msg.to_compile_error(),
        }
    }

    quote!(::std::vec![ #(#output,)* ])
}

struct IdentType {
    attrs: Vec<syn::Attribute>,
    ident: Box<syn::Ident>,
    _colon_token: Token![:],
    ty: Box<syn::Type>,
}

impl Parse for IdentType {
    fn parse(parser: ParseStream<'_>) -> syn::Result<IdentType> {
        Ok(IdentType {
            attrs: syn::Attribute::parse_outer(parser)?,
            ident: parser.parse()?,
            _colon_token: parser.parse()?,
            ty: parser.parse()?,
        })
    }
}

struct AssocConstSigs {
    ty: syn::Type,
    _at: Token![@],
    defs: Punctuated<IdentType, Token![;]>,
}

impl Parse for AssocConstSigs {
    fn parse(parser: ParseStream<'_>) -> syn::Result<AssocConstSigs> {
        Ok(AssocConstSigs {
            ty: parser.parse()?,
            _at: parser.parse()?,
            defs: parser.parse_terminated(IdentType::parse, Token![;])?,
        })
    }
}

pub fn extern_assoc_consts(input: TokenStream) -> TokenStream {
    let config = Config {
        crate_name: CrateName::local(),
        ..Default::default()
    };

    let input: AssocConstSigs = syn::parse2(input).expect("Couldn't parse assocconst def");

    let mut output = Vec::new();
    for sig in &input.defs {
        let attrs = &sig.attrs;
        let aconst = generate_assoc_const(&config, &input.ty, &sig.ident, &sig.ty);
        match aconst {
            Ok(ts) => output.push(quote!(#(#attrs)* #ts)),
            Err(msg) => return msg.to_compile_error(),
        }
    }

    quote!(::std::vec![ #(#output,)* ])
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
        Err(msg) => msg.to_compile_error(),
    }
}

pub fn impl_tuple(item: TokenStream) -> TokenStream {
    assert!(item.is_empty());
    let mut out = TokenStream::new();

    for len in 1..=26 {
        let types = (0..len)
            .map(|idx| syn::Ident::new(&format!("TupleTy{}", idx), Span::call_site()))
            .collect::<Vec<_>>();
        let indices = (0..len)
            .map(|idx| syn::Index::from(idx))
            .collect::<Vec<_>>();

        out.extend(quote!(
            unsafe impl<#(#types),*> Reflected for (#(#types,)*)
            where
                #(#types: Reflected,)*
                #(#types::Key: Sized,)*
            {
                const TYPE: Type = Type::new_tuple::<Self>();

                type Key = (#(#types::Key,)*);

                fn name() -> String {
                    let names = [#(#types::name()),*];
                    format!("({})", names.join(", "))
                }
            }

            impl<#(#types),*> ReflectedTuple for (#(#types,)*)
            where
                #(#types: Reflected,)*
                #(#types::Key: Sized,)*
            {
                const FIELDS: &'static [Field] = {
                    use crate::info::{AccessHelper, SetHelper};
                    use crate::value::Value;

                    &[#( {
                        let get_ptr: Option<AccessHelper> = Some(|this| {
                            // SAFETY: We know we won't borrow the item past the lifetime of the
                            //         containing value
                            let inner = unsafe { this.borrow_unsafe::<Self>() };
                            let v = Value::from_ref(&inner.#indices);
                            // SAFETY: See rebound::ty::Ref
                            unsafe { core::mem::transmute::<Value<'_>, Value<'_>>(v) }
                        });

                        let set_ptr: Option<SetHelper> = Some(|mut this, value| {
                            // SAFETY: We know we won't borrow the item past the lifetimes off the
                            //         containing value
                            let inner = unsafe { this.borrow_unsafe_mut::<Self>() };
                            // SAFETY: The passed value is expected to be static, so we can only
                            //         cast the lifetime lower here
                            inner.#indices = unsafe { value.cast_unsafe::<#types>() };
                        });

                        Field::new_tuple(get_ptr, set_ptr, #indices, Self::TYPE, #types::TYPE)

                    } ),*]
                };
            }
        ))
    }

    out
}
