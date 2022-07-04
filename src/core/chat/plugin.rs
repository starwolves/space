use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};

use crate::core::space_plugin::plugin::PostUpdateLabels;

use super::message::{chat_message_input_event, InputChatMessage};
use super::net::{net_system, NetChatMessage};

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputChatMessage>()
            .add_system(chat_message_input_event)
            .add_event::<NetChatMessage>()
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            );
    }
}
use bevy::app::CoreStage::PostUpdate;
