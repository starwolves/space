use bevy::prelude::{App, Plugin};
use networking::messaging::{register_reliable_message, MessageSender};

use crate::{
    input::{broadcast_global_chat_message, chat_net_input, GlobalChatMessage},
    net::{ChatClientMessage, ChatServerMessage},
};
use resources::is_server::is_server;
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(chat_net_input)
                .add_event::<GlobalChatMessage>()
                .add_system(broadcast_global_chat_message);
        }

        register_reliable_message::<ChatClientMessage>(app, MessageSender::Client);
        register_reliable_message::<ChatServerMessage>(app, MessageSender::Server);
    }
}
