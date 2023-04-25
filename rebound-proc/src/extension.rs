use proc_macro2::{Ident, Span, TokenStream};
use std::iter::FromIterator;
use syn::punctuated::Punctuated;
use syn::{
    AngleBracketedGenericArguments, Expr, ExprCall, ExprMacro, ExprPath, GenericArgument, Lifetime,
    LifetimeDef, Macro, MacroDelimiter, ParenthesizedGenericArguments, Path, PathArguments,
    PathSegment, PredicateLifetime, PredicateType, QSelf, ReturnType, Token, TraitBound,
    TraitBoundModifier, Type, TypeArray, TypeParam, TypeParamBound, TypePath, TypeReference,
    TypeTuple,
};

#[derive(Debug)]
pub enum LitType {
    String,
    ByteString,
    Byte,
    Char,
    Int,
    Float,
    Bool,
    Verbatim,
}

pub enum StructType {
    Named,
    Tuple,
    Unit,
}

pub trait LitExtension {
    fn ty(&self) -> LitType;
    fn as_str(&self) -> Option<String>;
}

impl LitExtension for syn::Lit {
    fn ty(&self) -> LitType {
        match self {
            syn::Lit::Str(_) => LitType::String,
            syn::Lit::ByteStr(_) => LitType::ByteString,
            syn::Lit::Byte(_) => LitType::Byte,
            syn::Lit::Char(_) => LitType::Char,
            syn::Lit::Int(_) => LitType::Int,
            syn::Lit::Float(_) => LitType::Float,
            syn::Lit::Bool(_) => LitType::Bool,
            syn::Lit::Verbatim(_) => LitType::Verbatim,
        }
    }

    fn as_str(&self) -> Option<String> {
        if let syn::Lit::Str(s) = self {
            Some(s.value())
        } else {
            None
        }
    }
}

pub trait PathExtension {
    fn new<T: Into<PathSegment>, I: IntoIterator<Item = T>>(segments: I) -> Self;

    fn with_leading(self) -> Self;

    fn as_simple_str(&self) -> Option<String>;
}

impl PathExtension for Path {
    fn new<T: Into<PathSegment>, I: IntoIterator<Item = T>>(segments: I) -> Self {
        Path {
            leading_colon: None,
            segments: Punctuated::from_iter(segments.into_iter().map(T::into)),
        }
    }

    fn with_leading(mut self) -> Self {
        self.leading_colon = Some(Token![::](Span::call_site()));
        self
    }

    fn as_simple_str(&self) -> Option<String> {
        let out = self
            .segments
            .iter()
            .map(|seg| {
                if seg.arguments.is_empty() {
                    Some(seg.ident.to_string())
                } else {
                    None
                }
            })
            .collect::<Option<Vec<_>>>()?
            .join("::");

        Some(out)
    }
}

pub trait StructExtension {
    fn ty(&self) -> StructType;
    fn fields(&self) -> Vec<&syn::Field>;
}

impl StructExtension for syn::ItemStruct {
    fn ty(&self) -> StructType {
        match &self.fields {
            syn::Fields::Named(_) => StructType::Named,
            syn::Fields::Unnamed(_) => StructType::Tuple,
            syn::Fields::Unit => StructType::Unit,
        }
    }

    fn fields(&self) -> Vec<&syn::Field> {
        match &self.fields {
            syn::Fields::Named(fields) => fields.named.iter().collect(),
            syn::Fields::Unnamed(fields) => fields.unnamed.iter().collect(),
            syn::Fields::Unit => vec![],
        }
    }
}

pub trait VariantExtension {
    fn ty(&self) -> StructType;
    fn fields(&self) -> Vec<&syn::Field>;
}

impl VariantExtension for syn::Variant {
    fn ty(&self) -> StructType {
        match &self.fields {
            syn::Fields::Named(_) => StructType::Named,
            syn::Fields::Unnamed(_) => StructType::Tuple,
            syn::Fields::Unit => StructType::Unit,
        }
    }

    fn fields(&self) -> Vec<&syn::Field> {
        match &self.fields {
            syn::Fields::Named(fields) => fields.named.iter().collect(),
            syn::Fields::Unnamed(fields) => fields.unnamed.iter().collect(),
            syn::Fields::Unit => vec![],
        }
    }
}

pub trait PathSegmentExtension {
    fn with_paren_arg(self, argument: Type) -> Self;
    fn with_all_paren_args<I: IntoIterator<Item = Type>>(self, arguments: I) -> Self;
    fn with_paren_ret(self, argument: ReturnType) -> Self;

    fn with_bracketed(self, argument: GenericArgument) -> Self;
    fn with_all_bracketed<I: IntoIterator<Item = GenericArgument>>(self, arguments: I) -> Self;
}

impl PathSegmentExtension for PathSegment {
    fn with_paren_arg(mut self, argument: Type) -> Self {
        self.arguments = match self.arguments {
            PathArguments::None => PathArguments::Parenthesized(ParenthesizedGenericArguments {
                paren_token: syn::token::Paren(Span::call_site()),
                inputs: Punctuated::from_iter([argument]),
                output: ReturnType::Default,
            }),
            PathArguments::Parenthesized(mut a) => {
                a.inputs.push(argument);
                PathArguments::Parenthesized(a)
            }
            PathArguments::AngleBracketed(_) => {
                panic!("Expected path segment to have parenthesized arguments")
            }
        };
        self
    }

    fn with_all_paren_args<I: IntoIterator<Item = Type>>(mut self, arguments: I) -> Self {
        for arg in arguments {
            self = self.with_paren_arg(arg);
        }
        self
    }

    fn with_paren_ret(mut self, ret: ReturnType) -> Self {
        self.arguments = match self.arguments {
            PathArguments::None => PathArguments::Parenthesized(ParenthesizedGenericArguments {
                paren_token: syn::token::Paren(Span::call_site()),
                inputs: Punctuated::new(),
                output: ret,
            }),
            PathArguments::Parenthesized(mut a) => {
                a.output = ret;
                PathArguments::Parenthesized(a)
            }
            PathArguments::AngleBracketed(_) => {
                panic!("Expected path segment to have parenthesized arguments")
            }
        };
        self
    }

    fn with_bracketed(mut self, argument: GenericArgument) -> Self {
        self.arguments = match self.arguments {
            PathArguments::None => PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: Some(Token![::](Span::call_site())),
                lt_token: Token![<](Span::call_site()),
                gt_token: Token![>](Span::call_site()),
                args: Punctuated::from_iter([argument]),
            }),
            PathArguments::AngleBracketed(mut a) => {
                a.args.push(argument);
                PathArguments::AngleBracketed(a)
            }
            PathArguments::Parenthesized(_) => {
                panic!("Expected path segment to have angle-bracketed arguments")
            }
        };
        self
    }

    fn with_all_bracketed<I: IntoIterator<Item = GenericArgument>>(mut self, arguments: I) -> Self {
        for arg in arguments {
            self = self.with_bracketed(arg);
        }
        self
    }
}

pub trait LifetimeExt {
    fn anon(span: Span) -> Self;
    fn static_(span: Span) -> Self;
}

impl LifetimeExt for Lifetime {
    fn anon(span: Span) -> Self {
        Lifetime::new("'_", span)
    }

    fn static_(span: Span) -> Self {
        Lifetime::new("'static", span)
    }
}

pub trait TypeTupleExt {
    fn empty() -> Self;
}

impl TypeTupleExt for TypeTuple {
    fn empty() -> Self {
        TypeTuple {
            paren_token: syn::token::Paren(Span::call_site()),
            elems: Punctuated::new(),
        }
    }
}

pub trait TypeReferenceExt {
    fn new(elem: Type) -> Self;

    fn with_mutability(self, m: Option<Token![mut]>) -> Self;
    fn with_lifetime(self, lt: Option<Lifetime>) -> Self;
}

impl TypeReferenceExt for TypeReference {
    fn new(elem: Type) -> Self {
        TypeReference {
            and_token: Token![&](Span::call_site()),
            lifetime: None,
            mutability: None,
            elem: Box::new(elem),
        }
    }

    fn with_mutability(mut self, m: Option<Token![mut]>) -> Self {
        self.mutability = m;
        self
    }

    fn with_lifetime(mut self, lt: Option<Lifetime>) -> Self {
        self.lifetime = lt;
        self
    }
}

pub trait TypePathExt {
    fn new(path: Path) -> Self;
    fn with_qself(self, qself: Option<QSelf>) -> Self;
}

impl TypePathExt for TypePath {
    fn new(path: Path) -> TypePath {
        TypePath { qself: None, path }
    }

    fn with_qself(mut self, qself: Option<QSelf>) -> Self {
        self.qself = qself;
        self
    }
}

pub trait TypeArrayExt {
    fn new(elem: Type, len: Expr) -> Self;
}

impl TypeArrayExt for TypeArray {
    fn new(elem: Type, len: Expr) -> Self {
        TypeArray {
            bracket_token: syn::token::Bracket(Span::call_site()),
            elem: Box::new(elem),
            semi_token: Token![;](Span::call_site()),
            len,
        }
    }
}

pub trait TraitBoundExt {
    fn new(path: Path) -> Self;
}

impl TraitBoundExt for TraitBound {
    fn new(path: Path) -> Self {
        TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path,
        }
    }
}

pub trait ExprPathExt {
    fn new(path: Path) -> Self;
}

impl ExprPathExt for ExprPath {
    fn new(path: Path) -> Self {
        ExprPath {
            attrs: Vec::new(),
            qself: None,
            path,
        }
    }
}

pub trait ExprCallExt {
    fn new(path: Expr) -> Self;
}

impl ExprCallExt for ExprCall {
    fn new(func: Expr) -> Self {
        ExprCall {
            attrs: Vec::new(),
            func: Box::new(func),
            paren_token: syn::token::Paren(Span::call_site()),
            args: Punctuated::new(),
        }
    }
}

pub trait ExprMacroExt {
    fn new(mac: Macro) -> Self;
}

impl ExprMacroExt for ExprMacro {
    fn new(mac: Macro) -> Self {
        ExprMacro {
            attrs: Vec::new(),
            mac,
        }
    }
}

pub trait MacroExt {
    fn new(path: Path) -> Self;

    fn with_tokens(self, tokens: TokenStream) -> Self;
}

impl MacroExt for Macro {
    fn new(path: Path) -> Macro {
        Macro {
            path,
            bang_token: Token![!](Span::call_site()),
            delimiter: MacroDelimiter::Paren(syn::token::Paren(Span::call_site())),
            tokens: TokenStream::new(),
        }
    }

    fn with_tokens(mut self, tokens: TokenStream) -> Self {
        self.tokens = tokens;
        self
    }
}

pub trait LifetimeDefExt {
    fn new(lt: Lifetime) -> Self;
}

impl LifetimeDefExt for LifetimeDef {
    fn new(lt: Lifetime) -> Self {
        LifetimeDef {
            attrs: Vec::new(),
            lifetime: lt,
            colon_token: None,
            bounds: Punctuated::new(),
        }
    }
}

pub trait PredicateLifetimeExt {
    fn new(lifetime: Lifetime) -> Self;

    fn with_bound(self, lifetime: Lifetime) -> Self;
    fn with_bounds<I: IntoIterator<Item = Lifetime>>(self, iter: I) -> Self;
}

impl PredicateLifetimeExt for PredicateLifetime {
    fn new(lifetime: Lifetime) -> Self {
        PredicateLifetime {
            lifetime,
            colon_token: Token![:](Span::call_site()),
            bounds: Punctuated::new(),
        }
    }

    fn with_bound(mut self, lifetime: Lifetime) -> Self {
        self.bounds.push(lifetime);
        self
    }

    fn with_bounds<I: IntoIterator<Item = Lifetime>>(mut self, iter: I) -> Self {
        self.bounds.extend(iter);
        self
    }
}

pub trait PredicateTypeExt {
    fn new<T: Into<Type>>(ty: T) -> Self;

    fn with_bounds<I: IntoIterator<Item = TypeParamBound>>(self, iter: I) -> Self;
}

impl PredicateTypeExt for PredicateType {
    fn new<T: Into<Type>>(ty: T) -> Self {
        PredicateType {
            lifetimes: None,
            bounded_ty: ty.into(),
            colon_token: Token![:](Span::call_site()),
            bounds: Punctuated::new(),
        }
    }

    fn with_bounds<I: IntoIterator<Item = TypeParamBound>>(mut self, iter: I) -> Self {
        self.bounds.extend(iter);
        self
    }
}

pub trait TypeParamExt {
    fn new(ident: Ident) -> Self;

    fn with_bounds<I: IntoIterator<Item = TypeParamBound>>(self, iter: I) -> Self;
}

impl TypeParamExt for TypeParam {
    fn new(ident: Ident) -> Self {
        TypeParam {
            attrs: Vec::new(),
            ident,
            colon_token: None,
            bounds: Punctuated::new(),
            eq_token: None,
            default: None,
        }
    }

    fn with_bounds<I: IntoIterator<Item = TypeParamBound>>(mut self, iter: I) -> Self {
        if self.colon_token.is_none() {
            self.colon_token = Some(Token![:](Span::call_site()));
        }
        self.bounds.extend(iter);
        self
    }
}
