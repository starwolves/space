use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use networking::messaging::{register_reliable_message, MessageSender};

use crate::{
    input::{broadcast_global_chat_message, chat_net_input, GlobalChatMessage},
    net::{ChatClientMessage, ChatServerMessage},
};
use resources::{modes::is_server_mode, ordering::Update};
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (
                    chat_net_input,
                    broadcast_global_chat_message.after(chat_net_input),
                ),
            )
            .add_event::<GlobalChatMessage>();
        }

        register_reliable_message::<ChatClientMessage>(app, MessageSender::Client, true);
        register_reliable_message::<ChatServerMessage>(app, MessageSender::Server, true);
    }
}
