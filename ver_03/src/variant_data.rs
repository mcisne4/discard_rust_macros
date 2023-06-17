use super::types::ChildAttributes;
use std::collections::HashMap;

pub fn get_variant_data(
    enum_data: &mut syn::DataEnum,
) -> deluxe::Result<HashMap<&'static str, &'static str>> {
    let mut variant_data: HashMap<&'static str, &'static str> = HashMap::new();

    for variant in enum_data.variants.iter_mut() {
        let ident = variant.ident.to_string();

        let attrs = deluxe::extract_attributes::<syn::Variant, ChildAttributes>(variant)?;

        // variant.fields.
    }

    Ok(variant_data)
}
