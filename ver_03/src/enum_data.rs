pub fn get_enum_data(ast: &mut syn::DeriveInput) -> deluxe::Result<syn::DataEnum> {
    let enum_data = match &ast.data {
        syn::Data::Enum(data) => data,
        syn::Data::Struct(_data) => {
            return Err(syn::Error::new_spanned(
                _data.struct_token,
                "implementation of Version03 for structs is not supported\nconsider using an enum instead",
            ))
        }
        syn::Data::Union(_data) => {
            return Err(syn::Error::new_spanned(
                _data.union_token,
                "implementation of Version03 for unions is not supported\nconsider using an enum instead",
            ))
        }
    };

    Ok(enum_data.to_owned())
}
