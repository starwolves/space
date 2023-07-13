use bevy::prelude::{App, Plugin, Update};
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
            app.add_systems(Update, (chat_net_input, broadcast_global_chat_message))
                .add_event::<GlobalChatMessage>();
        }

        register_reliable_message::<ChatClientMessage>(app, MessageSender::Client);
        register_reliable_message::<ChatServerMessage>(app, MessageSender::Server);
    }
}
