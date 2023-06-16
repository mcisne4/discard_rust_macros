#[macro_use]
extern crate darling;
extern crate syn;

#[derive(FromDeriveInput)]
#[darling(attributes(parent, child), forward_attrs)]
struct StreamInput {
    ident: syn::Ident,
    vis: syn::Visibility,
    // generics: dyn darling::FromGenerics,
    generics: syn::Generics,
    // data: syn::Data,
    attrs: Vec<syn::Attribute>,
}

#[proc_macro_derive(Version02, attributes(parent, child))]
pub fn version_02_derive_macro(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
    let x: StreamInput = darling::FromDeriveInput::from_derive_input(&ast).unwrap();

    for y in x.attrs.iter() {
        // y.
    }

    // darling::
    // darling::ast::Style::Struct

    todo!()
}
