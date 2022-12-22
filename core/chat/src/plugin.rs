use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use networking::messaging::{init_reliable_message, MessageSender};

use crate::{
    chat::{
        chat_message, send_entity_proximity_messages, EntityProximityMessage,
        EntityProximityMessages, NewChatMessage,
    },
    net::ChatClientMessage,
    networking::incoming_messages,
};
use bevy::app::CoreStage::PreUpdate;
use resources::is_server::is_server;
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<EntityProximityMessage>()
                .add_system(send_entity_proximity_messages.label(EntityProximityMessages::Send))
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<NewChatMessage>()
                .add_system(chat_message);
        }

        init_reliable_message::<ChatClientMessage>(app, MessageSender::Client);
    }
}
