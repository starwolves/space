use crate::space_core::structs::network_messages::{UIInputAction, UIInputNodeClass};

pub struct UIInput {
    pub handle : u32,
    pub node_class : UIInputNodeClass,
    pub action : UIInputAction,
    pub node_name : String,
    pub ui_type : String
}
