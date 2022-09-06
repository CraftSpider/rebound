use core::fmt;

use proc_macro2::TokenStream;
use quote::quote;

use crate::extension::LitType;

#[derive(Debug)]
pub enum Error {
    ParseError(syn::Error),
    NotSupported(String),
    InvalidLiteral(LitType, LitType),
    InvalidIdent(String),
    InvalidItem(String),
    Simple(String),
}

impl Error {
    pub fn to_compile_error(self) -> TokenStream {
        let out = format!("{}", self);
        quote!(compile_error!(#out);)
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error::ParseError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
             Error::ParseError(err) => write!(f, "Parse Error: {}", err),
             Error::NotSupported(what) => write!(f, "Rebound does not support {}, this may work in a future version", what),
             Error::InvalidLiteral(actual, expected) => write!(f, "Expected a `{:?}` literal, got `{:?}`", expected, actual),
             Error::InvalidIdent(actual) => write!(f, "Expected a valid identifier, got `{}`", actual),
             Error::InvalidItem(what) => write!(f, "#[rebound] can only be applied to a struct, enum, trait, or impl block. Instead got {}", what),
             Error::Simple(msg) => write!(f, "Rebound Error: {}", msg),
         }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
