use api::{
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    ui::{UIInputAction, UIInputNodeClass},
};

pub struct InputUIInput {
    pub handle: u64,
    pub node_class: UIInputNodeClass,
    pub action: UIInputAction,
    pub node_name: String,
    pub ui_type: String,
}

pub struct InputUIInputTransmitText {
    pub handle: u64,
    pub ui_type: String,
    pub node_path: String,
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
