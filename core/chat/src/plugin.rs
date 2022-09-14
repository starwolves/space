use api::data::PostUpdateLabels;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::messages::net_system;

use crate::proximity_message::{
    send_entity_proximity_messages, EntityProximityMessage, EntityProximityMessages,
    NetProximityMessage,
};
use bevy::app::CoreStage::PostUpdate;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityProximityMessage>()
            .add_event::<NetProximityMessage>()
            .add_system(send_entity_proximity_messages.label(EntityProximityMessages::Send))
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetProximityMessage>),
            );
    }
}
