use api::{
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    ui::{UIInputAction, UIInputNodeClass},
};
use networking_macros::NetMessage;

/// Event as client input , interaction with UI.
pub struct InputUIInput {
    /// Handle of the connection that input this.
    pub handle: u64,
    /// The Godot node class of the input element.
    pub node_class: UIInputNodeClass,
    /// The action ID.
    pub action: UIInputAction,
    /// The Godot node name of the input element.
    pub node_name: String,
    /// The UI this input was submitted from.
    pub ui_type: String,
}

/// Client input submitting text event.
pub struct InputUIInputTransmitText {
    /// Handle of the connection that input this.
    pub handle: u64,
    /// The UI this input was submitted from.
    pub ui_type: String,
    /// The Godot node path of the input element.
    pub node_path: String,
    /// The input text from the client.
    pub input_text: String,
}
#[derive(NetMessage)]
pub struct NetUIInputTransmitData {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
