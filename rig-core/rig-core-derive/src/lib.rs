extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod custom;
mod embeddable;

pub(crate) const EMBED: &str = "embed";

// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
// https://doc.rust-lang.org/reference/procedural-macros.html

#[proc_macro_derive(Embed, attributes(embed))]
pub fn derive_embedding_trait(item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    embeddable::expand_derive_embedding(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}