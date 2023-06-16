use super::types::ParentAttributes;

pub fn get_parent_attributes(ast: &mut syn::DeriveInput) -> deluxe::Result<(u8, u8)> {
    let parent_attributes = deluxe::extract_attributes::<syn::DeriveInput, ParentAttributes>(ast)?;

    Ok((parent_attributes.a, parent_attributes.b))
}
