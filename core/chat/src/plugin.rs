use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use networking::server::net_system;
use resources::labels::{PostUpdateLabels, PreUpdateLabels};

use crate::{
    chat::{
        chat_message, send_entity_proximity_messages, EntityProximityMessage,
        EntityProximityMessages, NetChatMessage, NetProximityMessage, NetSendEntityUpdates,
        NewChatMessage,
    },
    networking::incoming_messages,
};
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_event::<EntityProximityMessage>()
                .add_event::<NetProximityMessage>()
                .add_system(send_entity_proximity_messages.label(EntityProximityMessages::Send))
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetProximityMessage>)
                        .with_system(net_system::<NetChatMessage>)
                        .with_system(net_system::<NetSendEntityUpdates>),
                )
                .add_event::<NetChatMessage>()
                .add_system_to_stage(
                    PreUpdate,
                    incoming_messages
                        .after(PreUpdateLabels::NetEvents)
                        .label(PreUpdateLabels::ProcessInput),
                )
                .add_event::<NetSendEntityUpdates>()
                .add_event::<NewChatMessage>()
                .add_system(chat_message);
        }
    }
}
