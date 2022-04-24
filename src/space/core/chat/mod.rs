use bevy_app::{App, Plugin};

use self::{
    events::{InputChatMessage, NetChatMessage},
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
            .add_event::<NetChatMessage>();
    }
}
