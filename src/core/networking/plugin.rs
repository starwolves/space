use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use bevy_renet::RenetServerPlugin;

use crate::core::space_plugin::plugin::PreUpdateLabels;

use super::networking::{
    connections, incoming_messages, startup_listen_connections, ReliableServerMessage,
};
use bevy::app::CoreStage::PreUpdate;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetServerPlugin)
            .insert_resource(startup_listen_connections())
            .add_system_to_stage(
                PreUpdate,
                incoming_messages.after(PreUpdateLabels::NetEvents),
            )
            .add_system_to_stage(PreUpdate, connections.label(PreUpdateLabels::NetEvents));
    }
}

pub const RENET_RELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const RENET_BLOCKING_CHANNEL_ID: u8 = 2;

pub struct NetEvent {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
