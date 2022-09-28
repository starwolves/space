extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// The derivation for netcode messages.
#[proc_macro_derive(NetMessage)]
pub fn derive_net_message(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    format!(
        "
    impl PendingMessage for {} {{
        fn get_message(&self) -> PendingNetworkMessage {{
            PendingNetworkMessage {{
                handle: self.handle,
                message: self.message.clone(),
            }}
        }}
    }}
    ",
        struct_name
    )
    .parse()
    .unwrap()
}
