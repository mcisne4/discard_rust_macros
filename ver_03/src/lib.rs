mod enum_data;
mod parent_attr;
mod types;
mod variant_data;

// === MAIN MACRO === //

#[proc_macro_derive(Version03, attributes(parent, child))]
pub fn version_03_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item2: proc_macro2::TokenStream = item.into();

    match version_03_derive_macro2(item2) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

// === SUB MACRO === //

fn version_03_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // --- Parse TokenStream --- //
    let mut ast = syn::parse2::<syn::DeriveInput>(item)?;

    // --- Data Type --- //
    let mut edata = enum_data::get_enum_data(&mut ast)?;

    // --- Parent Attributes --- //
    let (parent_a, parent_b) = parent_attr::get_parent_attributes(&mut ast)?;

    // --- Variant Data --- //
    let x = variant_data::get_variant_data(&mut edata)?;

    // --- Ident and Generics --- //
    let ident = ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // --- Output --- //
    Ok(quote::quote! {
      impl #impl_generics Version03 for #ident #type_generics #where_clause {
        fn get_parent(&self) -> &'static str {
          concat!("Parent: { a: ", #parent_a, ", b: ", #parent_b, " }" )
        }

        fn get_children() -> &'static str {
          concat!("Children: { ", " }")
        }
      }
    })
}
