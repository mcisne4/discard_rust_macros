#[proc_macro_derive(Ver01, attributes(sample, sample2))]
pub fn version_01_derive_macro(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // --- Dev --- //
    let mut data = String::new();

    // --- Parse Token Stream --- //
    let ast: syn::DeriveInput = syn::parse(tokens.clone()).unwrap();
    // eprintln!("ast: {:#?}", ast);

    // --- Attributes --- //
    for attr in ast.attrs {
        let mut header = String::new();

        for path_segment in &attr.path().segments {
            let name = path_segment.ident.to_string();
            header += &name;
        }

        data += "\n\tpath_segment: ";
        data += &header;
        data += "\n";
    }

    // --- Data Type --- //
    data += match ast.vis {
        syn::Visibility::Inherited => "",
        syn::Visibility::Public(_) => "pub ",
        syn::Visibility::Restricted(_) => "",
    };

    // // --- Data Type --- //
    // let data_type = match ast.data {
    //     syn::Data::Enum(_d) => "enum",
    //     syn::Data::Struct(_d) => "struct",
    //     syn::Data::Union(_d) => "union",
    // };
    // data += data_type;

    // --- Identifier --- //
    let ident = ast.ident;
    data += &ident.to_string();
    data += " {\n";

    // --- Generate Token Stream --- //
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // concat!("ident: ", #ident_str, " data_type: '", #data_type, "'")
    quote::quote!(
      impl #impl_generics Ver01 for #ident #type_generics #where_clause {
        fn dev(&self) -> &'static str {
          #data
        }
      }
    )
    .into()
}
