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
    fn as_simple_str(&self) -> Option<String>;
}

impl PathExtension for syn::Path {
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
