use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;

use crate::space::PostUpdateLabels;

use self::{
    events::{net_system, InputChatMessage, NetChatMessage},
    systems::chat_message_input_event,
};

pub mod components;
pub mod events;
pub mod functions;
pub mod systems;

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputChatMessage>()
            .add_system(chat_message_input_event)
            .add_event::<NetChatMessage>()
            .add_system_to_stage(
                PostUpdate,
                net_system.after(PostUpdateLabels::VisibleChecker),
            );
    }
}
use bevy_app::CoreStage::PostUpdate;
