use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Make sure the supplied EntityType has a field named "identifier" of type String.
#[proc_macro_derive(Identity)]
pub fn derive_entity_type(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    format!(
        "impl EntityType for {} {{
    fn to_string(&self) -> String {{
        self.identifier.clone()
    }}

    fn new() -> Self
    where
        Self: Sized,
    {{
        {}::default()
    }}
    fn is_type(&self, identifier: String) -> bool {{
        self.identifier == identifier
    }}
}}",
        struct_name, struct_name
    )
    .parse()
    .unwrap()
}
