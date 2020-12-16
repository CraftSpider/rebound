
use syn::Lifetime;
use syn::spanned::Spanned;

pub fn static_or_anon(life: &Lifetime) -> Lifetime {
    if life.ident == "static" {
        life.clone()
    } else {
        Lifetime::new("'_", life.span())
    }
}
