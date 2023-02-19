use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Make sure the supplied EntityType has a field named "identifier" of type String.
#[proc_macro_derive(Identity)]
pub fn derive_entity_type(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    format!(
        "impl EntityType for {} {{
    fn get_identity(&self) -> String {{
        self.identifier.clone()
    }}
    fn get_clean_identity(&self) -> String {{
        match self.identifier.rsplit_once(\":\") {{
            Some(s) => s.1.to_string(),
            None => self.identifier.clone()
        }}
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
