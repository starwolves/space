use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin};

use crate::{
    chat::{
        chat_message, send_entity_proximity_messages, EntityProximityMessage,
        EntityProximityMessages, NewChatMessage,
    },
    networking::incoming_messages,
};
use bevy::app::CoreStage::PreUpdate;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_event::<EntityProximityMessage>()
                .add_system(send_entity_proximity_messages.label(EntityProximityMessages::Send))
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<NewChatMessage>()
                .add_system(chat_message);
        }
    }
}
