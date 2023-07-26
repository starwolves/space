use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use networking::messaging::{register_reliable_message, MessageSender};

use crate::{
    input::{broadcast_global_chat_message, chat_net_input, GlobalChatMessage},
    net::{ChatClientMessage, ChatServerMessage},
};
use resources::{is_server::is_server, sets::MainSet};
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                (chat_net_input, broadcast_global_chat_message).in_set(MainSet::Update),
            )
            .add_event::<GlobalChatMessage>();
        }

        register_reliable_message::<ChatClientMessage>(app, MessageSender::Client);
        register_reliable_message::<ChatServerMessage>(app, MessageSender::Server);
    }
}
