use api::{
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    ui::{UIInputAction, UIInputNodeClass},
};

/// Client input , interaction with UI.
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

/// Client input submitting text.
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

pub struct NetUIInputTransmitData {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetUIInputTransmitData {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
